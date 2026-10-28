type JsonRpc = {
    jsonrpc: '2.0'
    id?: string | number
    method: string
    params?: Record<string, unknown>
}

export function mcpApps() {
    document.querySelectorAll<HTMLIFrameElement>('iframe[data-mcp-app="true"]').forEach((frame) => {
        if (frame.dataset.bridgeAttached === 'true') return
        frame.dataset.bridgeAttached = 'true'
        frame.setAttribute('sandbox', 'allow-scripts allow-forms')
        frame.setAttribute('loading', 'lazy')

        const send = (message: unknown) => frame.contentWindow?.postMessage(message, '*')
        const notify = (method: string, params: unknown) => send({ jsonrpc: '2.0', method, params })

        const onMessage = async (event: MessageEvent) => {
            if (event.source !== frame.contentWindow || event.origin !== 'null') return
            const message = event.data as JsonRpc
            if (!message || message.jsonrpc !== '2.0' || typeof message.method !== 'string') return

            if (message.method === 'ui/initialize') {
                send({
                    jsonrpc: '2.0',
                    id: message.id,
                    result: {
                        protocolVersion: '2025-11-25',
                        hostInfo: { name: 'Bionic', version: '1.0.0' },
                        hostCapabilities: { tools: { listChanged: false } },
                        hostContext: { theme: document.documentElement.dataset.theme || 'light' },
                    },
                })
                notify('ui/notifications/initialized', {})
                notify('ui/notifications/tool-input', { arguments: parseJson(frame.dataset.toolInput) })
                notify('ui/notifications/tool-result', parseJson(frame.dataset.toolResult))
                return
            }

            if (message.method === 'tools/call' && message.id !== undefined) {
                const call = message.params || {}
                try {
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

            if (message.method === 'ui/message') {
                const payload = message.params?.content || message.params?.message || ''
                const content = Array.isArray(payload)
                    ? payload.map((part) => typeof part === 'object' && part !== null && 'text' in part ? String(part.text) : '').filter(Boolean).join('\n')
                    : payload
                const input = document.querySelector<HTMLTextAreaElement>('textarea[name="message"]')
                if (input && typeof content === 'string') {
                    input.value = input.value ? `${input.value}\n${content}` : content
                    input.dispatchEvent(new Event('input', { bubbles: true }))
                    input.focus()
                }
            }
        }

        window.addEventListener('message', onMessage)
        const appSrc = frame.dataset.src
        if (appSrc) frame.src = appSrc
        frame.addEventListener('load', () => {
            // The view starts the MCP Apps handshake after loading. Keep host-side
            // listener installed before its first postMessage.
        }, { once: true })
    })
}

function parseJson(value: string | undefined): unknown {
    if (!value) return null
    try { return JSON.parse(value) } catch { return null }
}
