type JsonRpc = {
    jsonrpc: '2.0'
    id?: string | number
    method: string
    params?: Record<string, unknown>
}

type AppState = {
    source: string
    lastPayload: string
    initialized: boolean
    displayMode: 'inline' | 'fullscreen' | 'pip'
    availableModes: Set<string>
    widgetState: unknown
}

const states = new WeakMap<HTMLIFrameElement, AppState>()

export function mcpApps() {
    initializePanelResizer()
    const initializeFrames = () => {
        document.querySelectorAll<HTMLIFrameElement>('iframe[data-mcp-app="true"]').forEach((frame) => {
            if (states.has(frame)) synchronizeFrame(frame)
            else connectFrame(frame)
        })
    }
    initializeFrames()

    const mutationObserver = new MutationObserver((mutations) => {
        for (const mutation of mutations) {
            if (mutation.type === 'childList') {
                initializeFrames()
                initializePanelResizer()
            }
            if (mutation.type === 'attributes' && mutation.target instanceof HTMLIFrameElement) {
                synchronizeFrame(mutation.target)
            }
        }
    })
    mutationObserver.observe(document.body, {
        childList: true,
        subtree: true,
        attributes: true,
        attributeFilter: [
            'data-src', 'data-server-id', 'data-resource-uri', 'data-tool-input',
            'data-tool-result', 'data-chat-id',
        ],
    })

    document.addEventListener('click', (event) => {
        const target = event.target
        if (!(target instanceof Element)) return

        const select = target.closest<HTMLButtonElement>('[data-mcp-app-select="true"]')
        if (select) {
            const frame = document.querySelector<HTMLIFrameElement>('#mcp-app-panel-frame')
            if (!frame) return
            try {
                const response = select.closest<HTMLElement>('[data-mcp-app-action]')
                    ?.querySelector<HTMLElement>('[data-mcp-app-result]')?.textContent || '{}'
                const result = JSON.parse(response) as Record<string, unknown>
                const app = result._bionic_mcp_app as Record<string, unknown> | undefined
                if (!app) return
                const serverId = Number(app.server_id)
                const resourceUri = typeof app.resource_uri === 'string' ? app.resource_uri : ''
                if (!Number.isInteger(serverId) || !resourceUri) return
                frame.dataset.serverId = String(serverId)
                frame.dataset.teamId = select.dataset.teamId || ''
                frame.dataset.resourceUri = resourceUri
                frame.dataset.toolInput = JSON.stringify(app.tool_input ?? {})
                frame.dataset.toolResult = JSON.stringify(app.tool_result ?? {})
                frame.dataset.chatId = String(app.chat_id ?? '')
                frame.dataset.src = select.dataset.appSrc || ''
                frame.title = select.dataset.appTitle || 'MCP App'
                const panel = document.querySelector<HTMLElement>('#mcp-app-panel')
                if (panel) panel.dataset.appTitle = frame.title
                const panelTitle = document.querySelector<HTMLElement>('#mcp-app-panel-title')
                if (panelTitle) panelTitle.textContent = frame.title
                synchronizeFrame(frame)
                showPanel(frame)
            } catch {
                return
            }
            return
        }

        const close = target.closest<HTMLElement>('[data-mcp-app-close="true"]')
        if (close) {
            document.querySelector<HTMLElement>('#mcp-app-panel')?.setAttribute('hidden', 'true')
            document.querySelector<HTMLElement>('#mcp-app-panel-resizer')?.setAttribute('hidden', 'true')
            return
        }

        const fullscreen = target.closest<HTMLElement>('[data-mcp-app-fullscreen="true"]')
        if (fullscreen) {
            const panel = document.querySelector<HTMLElement>('#mcp-app-panel')
            const frame = document.querySelector<HTMLIFrameElement>('#mcp-app-panel-frame')
            const state = frame && states.get(frame)
            if (panel && frame && state) setDisplayMode(frame, state.displayMode === 'fullscreen' ? 'pip' : 'fullscreen')
        }
    })
}

function initializePanelResizer() {
    const resizer = document.querySelector<HTMLElement>('[data-mcp-app-resizer="true"]')
    const panel = document.querySelector<HTMLElement>('#mcp-app-panel')
    const workspace = document.querySelector<HTMLElement>('#console-panel')
    if (!resizer || !panel || !workspace || resizer.dataset.resizeAttached === 'true') return

    const minWidth = 35
    const maxWidth = 80
    const defaultWidth = 2 / 3 * 100
    const storageKey = `bionic:mcp-app-panel-width:${resizer.dataset.resizeKey || ''}`
    const desktop = window.matchMedia('(min-width: 1024px)')
    let width = defaultWidth
    try {
        const savedWidth = Number(localStorage.getItem(storageKey))
        if (Number.isFinite(savedWidth) && savedWidth > 0) width = Math.max(minWidth, Math.min(maxWidth, savedWidth))
    } catch {
        // Keep the default width when browser storage is unavailable.
    }

    const update = (nextWidth: number, persist: boolean) => {
        width = Math.max(minWidth, Math.min(maxWidth, nextWidth))
        resizer.setAttribute('aria-valuenow', String(Math.round(width)))
        workspace.style.setProperty('--mcp-app-panel-width', `${width}%`)
        if (persist) {
            try {
                localStorage.setItem(storageKey, String(width))
            } catch {
                // Resizing still works for this page when browser storage is unavailable.
            }
        }
    }

    const applyBreakpoint = () => {
        if (desktop.matches) update(width, false)
        else workspace.style.removeProperty('--mcp-app-panel-width')
    }
    applyBreakpoint()
    desktop.addEventListener('change', applyBreakpoint)

    let dragging = false
    resizer.addEventListener('pointerdown', (event) => {
        if (!desktop.matches || event.button !== 0) return
        dragging = true
        resizer.setPointerCapture(event.pointerId)
        event.preventDefault()
    })
    resizer.addEventListener('pointermove', (event) => {
        if (!dragging) return
        const bounds = workspace.getBoundingClientRect()
        if (bounds.width <= 0) return
        update(((bounds.right - event.clientX) / bounds.width) * 100, false)
    })
    const finishDrag = (event: PointerEvent) => {
        if (!dragging) return
        dragging = false
        if (resizer.hasPointerCapture(event.pointerId)) resizer.releasePointerCapture(event.pointerId)
        try {
            localStorage.setItem(storageKey, String(width))
        } catch {
            // Resizing still works for this page when browser storage is unavailable.
        }
    }
    resizer.addEventListener('pointerup', finishDrag)
    resizer.addEventListener('pointercancel', finishDrag)
    resizer.addEventListener('keydown', (event) => {
        if (!desktop.matches) return
        let nextWidth: number | undefined
        if (event.key === 'ArrowLeft') nextWidth = width + (event.shiftKey ? 5 : 2)
        if (event.key === 'ArrowRight') nextWidth = width - (event.shiftKey ? 5 : 2)
        if (event.key === 'Home') nextWidth = minWidth
        if (event.key === 'End') nextWidth = maxWidth
        if (nextWidth === undefined) return
        event.preventDefault()
        update(nextWidth, true)
    })
    resizer.dataset.resizeAttached = 'true'
}

function connectFrame(frame: HTMLIFrameElement) {
    if (states.has(frame)) return
    frame.setAttribute('sandbox', 'allow-scripts allow-forms')
    frame.setAttribute('loading', 'lazy')

    const state: AppState = {
        source: '',
        lastPayload: '',
        initialized: false,
        displayMode: 'pip',
        availableModes: new Set(['inline', 'fullscreen', 'pip']),
        widgetState: null,
    }
    states.set(frame, state)

    const send = (message: unknown) => frame.contentWindow?.postMessage(message, '*')
    const notify = (method: string, params: unknown) => send({ jsonrpc: '2.0', method, params })
    const respond = (id: string | number, result: unknown) => send({ jsonrpc: '2.0', id, result })

    const onMessage = async (event: MessageEvent) => {
        if (event.source !== frame.contentWindow || event.origin !== 'null') return
        const message = event.data as JsonRpc
        if (!message || message.jsonrpc !== '2.0' || typeof message.method !== 'string') return

        if (message.method === 'bionic/openai/initialize' && message.id !== undefined) {
            const requestedModes = message.params?.availableDisplayModes
            if (Array.isArray(requestedModes) && requestedModes.length > 0) state.availableModes = new Set(requestedModes.filter((mode): mode is string => typeof mode === 'string'))
            const preferredMode = message.params?.preferredDisplayMode
            if (preferredMode === 'inline' || preferredMode === 'fullscreen' || preferredMode === 'pip') {
                if (state.availableModes.has(preferredMode)) setDisplayMode(frame, preferredMode)
            }
            const toolResult = parseJson(frame.dataset.toolResult)
            respond(message.id, {
                toolInput: parseJson(frame.dataset.toolInput),
                toolOutput: toolResult && typeof toolResult === 'object' ? (toolResult as Record<string, unknown>).structuredContent ?? null : null,
                widgetState: state.widgetState,
                displayMode: state.displayMode,
                availableDisplayModes: ['inline', 'fullscreen', 'pip'],
            })
            return
        }

        if (message.method === 'bionic/openai/set-widget-state' && message.id !== undefined) {
            state.widgetState = message.params?.state ?? null
            respond(message.id, {})
            return
        }

        if (message.method === 'bionic/assets/read' && message.id !== undefined) {
            try {
                const path = message.params?.path
                if (typeof path !== 'string' || !/^[a-zA-Z0-9_.-]+$/.test(path) || path.includes('..')) throw new Error('Invalid asset path')
                const url = `/o/${encodeURIComponent(frame.dataset.teamId || '')}/mcp-apps/${encodeURIComponent(frame.dataset.serverId || '')}/assets/${encodeURIComponent(path)}?uri=${encodeURIComponent(frame.dataset.resourceUri || '')}`
                const response = await fetch(url, { credentials: 'same-origin' })
                if (!response.ok) throw new Error(`Asset HTTP ${response.status}`)
                const bytes = await response.arrayBuffer()
                frame.contentWindow?.postMessage({ jsonrpc: '2.0', id: message.id, result: { bytes } }, '*', [bytes])
            } catch (error) {
                send({ jsonrpc: '2.0', id: message.id, error: { code: -32000, message: String(error) } })
            }
            return
        }

        if (message.method === 'ui/notifications/size-changed') {
            const height = message.params?.height
            if (state.displayMode === 'inline' && typeof height === 'number' && Number.isFinite(height)) frame.style.height = `${Math.max(200, Math.min(1200, height))}px`
            return
        }

        if (message.method === 'ui/initialize' && message.id !== undefined) {
            const modes = message.params?.appCapabilities && (message.params.appCapabilities as Record<string, unknown>).availableDisplayModes
            if (Array.isArray(modes) && modes.length > 0) state.availableModes = new Set(modes.filter((mode): mode is string => typeof mode === 'string'))
            send({
                jsonrpc: '2.0',
                id: message.id,
                result: {
                    protocolVersion: '2026-01-26',
                    hostInfo: { name: 'Bionic', version: '1.0.0' },
                    hostCapabilities: { tools: { listChanged: false }, message: { text: {} } },
                    hostContext: {
                        theme: document.documentElement.dataset.theme || 'light',
                        displayMode: state.displayMode,
                        availableDisplayModes: ['inline', 'fullscreen', 'pip'],
                    },
                },
            })
            state.initialized = true
            notify('ui/notifications/initialized', {})
            deliverCurrentResult(frame, notify, state)
            return
        }

        if (message.method === 'ui/request-display-mode' && message.id !== undefined) {
            const mode = message.params?.mode
            let actualMode = state.displayMode
            if (mode === 'inline' || mode === 'fullscreen' || mode === 'pip') {
                actualMode = setDisplayMode(frame, mode)
            }
            respond(message.id, { mode: actualMode })
            return
        }

        if (message.method === 'ui/notifications/request-teardown') {
            if (message.id !== undefined) respond(message.id, {})
            document.querySelector<HTMLElement>('#mcp-app-panel')?.setAttribute('hidden', 'true')
            return
        }

        if (message.method === 'ui/resource-teardown' && message.id !== undefined) {
            respond(message.id, {})
            return
        }

        if (message.method === 'tools/call' && message.id !== undefined) {
            try {
                const call = message.params || {}
                const teamId = encodeURIComponent(frame.dataset.teamId || '')
                const serverId = encodeURIComponent(frame.dataset.serverId || '')
                const response = await fetch(`/o/${teamId}/mcp-apps/${serverId}/call`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    credentials: 'same-origin',
                    body: JSON.stringify(call),
                })
                const result = await response.json()
                if (!response.ok) throw new Error(result?.message || `HTTP ${response.status}`)
                send({ jsonrpc: '2.0', id: message.id, result })
            } catch (error) {
                send({ jsonrpc: '2.0', id: message.id, error: { code: -32000, message: String(error) } })
            }
            return
        }

        if (message.method === 'ui/message' && message.id !== undefined) {
            const blocks = message.params?.content
            if (message.params?.role !== 'user' || !Array.isArray(blocks) || !blocks.every((block) => typeof block === 'object' && block !== null && 'type' in block && block.type === 'text' && 'text' in block && typeof block.text === 'string')) {
                send({ jsonrpc: '2.0', id: message.id, error: { code: -32602, message: 'Expected a user message with text content' } })
                return
            }
            const content = blocks.map((block) => (block as { text: string }).text).join('\n')
            const input = document.querySelector<HTMLTextAreaElement>('textarea[name="message"]')
            if (!content.trim() || !input || !input.form || input.disabled) {
                send({ jsonrpc: '2.0', id: message.id, result: { isError: true } })
                return
            }
            input.value = input.value ? `${input.value}\n${content}` : content
            input.dispatchEvent(new Event('input', { bubbles: true }))
            input.focus()
            respond(message.id, {})
        }
    }

    window.addEventListener('message', onMessage)
    synchronizeFrame(frame)
}

function synchronizeFrame(frame: HTMLIFrameElement) {
    const state = states.get(frame)
    if (!state) return
    const source = frame.dataset.src || ''
    if (source && source !== state.source) {
        state.source = source
        state.initialized = false
        state.displayMode = 'pip'
        state.availableModes = new Set(['inline', 'fullscreen', 'pip'])
        state.widgetState = null
        state.lastPayload = payload(frame)
        frame.style.removeProperty('height')
        const mount = document.querySelector<HTMLElement>('#mcp-app-panel-frame-mount')
        if (mount && frame.parentElement !== mount) mount.append(frame)
        const panel = document.querySelector<HTMLElement>('#mcp-app-panel')
        if (panel) {
            panel.dataset.displayMode = 'pip'
            panel.classList.remove('mcp-app-fullscreen')
            panel.classList.add('mcp-app-pip')
            panel.removeAttribute('hidden')
        }
        frame.src = source
        return
    }
    const currentPayload = payload(frame)
    if (state.initialized && currentPayload !== state.lastPayload) {
        state.lastPayload = currentPayload
        deliverCurrentResult(frame, (method, params) => frame.contentWindow?.postMessage({ jsonrpc: '2.0', method, params }, '*'), state)
    }
    ensurePlacement(frame, state)
}

function deliverCurrentResult(frame: HTMLIFrameElement, notify: (method: string, params: unknown) => void, state: AppState) {
    const toolInput = parseJson(frame.dataset.toolInput)
    const toolResult = parseJson(frame.dataset.toolResult)
    notify('ui/notifications/tool-input', { arguments: toolInput })
    notify('ui/notifications/tool-result', toolResult)
    notify('bionic/openai/set-globals', {
        toolInput,
        toolOutput: toolResult && typeof toolResult === 'object' ? (toolResult as Record<string, unknown>).structuredContent ?? null : null,
        widgetState: state.widgetState,
    })
}

function setDisplayMode(frame: HTMLIFrameElement, mode: 'inline' | 'fullscreen' | 'pip') {
    const state = states.get(frame)
    const panel = document.querySelector<HTMLElement>('#mcp-app-panel')
    if (!state || !panel || !state.availableModes.has(mode)) return state?.displayMode || 'pip'
    if (mode === 'inline') {
        if (!mountInline(frame)) return state.displayMode
        panel.setAttribute('hidden', 'true')
        document.querySelector<HTMLElement>('#mcp-app-panel-resizer')?.setAttribute('hidden', 'true')
    } else {
        const mount = document.querySelector<HTMLElement>('#mcp-app-panel-frame-mount')
        if (mount && frame.parentElement !== mount) mount.append(frame)
        panel.removeAttribute('hidden')
        document.querySelector<HTMLElement>('#mcp-app-panel-resizer')?.removeAttribute('hidden')
    }
    state.displayMode = mode
    if (mode !== 'inline') frame.style.removeProperty('height')
    panel.dataset.displayMode = mode
    panel.classList.toggle('mcp-app-fullscreen', mode === 'fullscreen')
    panel.classList.toggle('mcp-app-pip', mode === 'pip')
    frame.contentWindow?.postMessage({
        jsonrpc: '2.0',
        method: 'ui/notifications/host-context-changed',
        params: { displayMode: mode },
    }, '*')
    return mode
}

function showPanel(frame: HTMLIFrameElement) {
    const state = states.get(frame)
    if (!state) return
    setDisplayMode(frame, state.availableModes.has('pip') ? 'pip' : 'inline')
}

function mountInline(frame: HTMLIFrameElement) {
    const chatId = frame.dataset.chatId
    if (!chatId) return false
    const slot = Array.from(document.querySelectorAll<HTMLElement>('[data-mcp-app-inline-slot]'))
        .find((candidate) => candidate.dataset.mcpAppInlineSlot === chatId)
    if (!slot) return false
    if (frame.parentElement !== slot) slot.append(frame)
    return true
}

function ensurePlacement(frame: HTMLIFrameElement, state: AppState) {
    if (state.displayMode === 'inline') {
        mountInline(frame)
    } else {
        const mount = document.querySelector<HTMLElement>('#mcp-app-panel-frame-mount')
        if (mount && frame.parentElement !== mount) mount.append(frame)
    }
}

function payload(frame: HTMLIFrameElement) {
    return `${frame.dataset.toolInput || ''}\n${frame.dataset.toolResult || ''}`
}

function parseJson(value: string | undefined): unknown {
    if (!value) return null
    try { return JSON.parse(value) } catch { return null }
}
