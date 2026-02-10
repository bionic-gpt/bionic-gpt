import { renderMarkdownSafe } from "./markdown"

export const streamingChat = () => {
    const chat = document.getElementById('streaming-chat')

    const chatId = chat?.dataset.chatid

    if (chatId && chat) {
        streamResult(chatId, chat)
    }
}

async function streamResult(chatId: string, element: HTMLElement) {
    const abortController = new AbortController();
    const signal = abortController.signal;

    const stopButton = document.getElementById('streaming-button');
    const stopListener = () => {
        abortController.abort("User aborted");
    };

    if (stopButton) {
        stopButton.addEventListener('click', stopListener);
    }

    // Submit the existing form to trigger redirect/reset after streaming ends.
    // Stream persistence is handled by the backend.
    const finalizeUiState = () => {
        const form = document.getElementById(`chat-form-${chatId}`);

        if (form instanceof HTMLFormElement) {
            try {
                form.requestSubmit();
            } catch (error) {
                console.error('Error finalizing UI state:', error);
            }
        }
    };

    const res = await fetch(`/completions/${chatId}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        signal,
    });

    if (!res.body) {
        console.error('No response body');
        return;
    }

    const reader = res.body.getReader();
    const decoder = new TextDecoder();
    let buffer = '';
    let snapshot = '';

    const parseEvent = (chunk: string) => {
        const lines = chunk.split(/\n/);
        let data = '';
        for (const line of lines) {
            if (line.startsWith('data:')) {
                data += line.slice(5).trim();
            }
        }
        return data;
    };

    const handleV2Event = (data: string) => {
        try {
            const json = JSON.parse(data);
            if (typeof json?.type !== 'string') {
                return false;
            }

            if (json.type === 'text_delta') {
                const delta = json?.data?.delta;
                if (typeof delta === 'string' && delta.length > 0) {
                    snapshot += delta;
                    element.innerHTML = renderMarkdownSafe(snapshot);
                }
                return false;
            }

            if (json.type === 'done') {
                finalizeUiState();
                return true;
            }

            if (json.type === 'error') {
                const message = String(json?.data?.message ?? 'Unknown streaming error');
                element.innerHTML = renderMarkdownSafe(`${snapshot}\n\n${message}`);
                finalizeUiState();
                return true;
            }
        } catch (_e) {
            return false;
        }

        return false;
    };

    try {
        while (true) {
            const { value, done } = await reader.read();
            if (done) break;
            buffer += decoder.decode(value, { stream: true });

            let boundary: number;
            while ((boundary = buffer.indexOf('\n\n')) !== -1) {
                const raw = buffer.slice(0, boundary).trim();
                buffer = buffer.slice(boundary + 2);
                if (!raw) continue;

                const data = parseEvent(raw);
                if (!data) continue;

                if (handleV2Event(data)) {
                    return;
                }
            }
        }
        finalizeUiState();
    } catch (err) {
        console.error('Streaming failed', err);
        element.innerHTML += `${err}`;
        finalizeUiState();
    }

}
