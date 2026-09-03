// SSE event contract for `/completions/{chatId}`:
// - { type: "text_delta", data: { delta: string } }
// - { type: "done", data: {} }
// - { type: "error", data: { message: string } }
//
// The stream endpoint is backend-owned for persistence. This client only shows a
// temporary plain-text draft and submits the post-stream form so the backend can
// render the final markdown, tool calls, generated files, and reasoning state.
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

    // Submit the existing form only after the backend confirms a completed stream.
    // Stream persistence is handled by the backend.
    const finalizeUiState = () => {
        element.setAttribute('aria-busy', 'false');
        const form = document.getElementById(`chat-form-${chatId}`);

        if (form instanceof HTMLFormElement) {
            try {
                form.requestSubmit();
            } catch (error) {
                console.error('Error finalizing UI state:', error);
            }
        }
    };

    const showError = (message: string) => {
        element.replaceChildren();
        element.setAttribute('aria-busy', 'false');
        const error = document.createElement('p');
        error.className = 'text-error whitespace-pre-wrap break-words';
        error.textContent = message;
        element.appendChild(error);
    };

    let res: Response;
    try {
        res = await fetch(`/completions/${chatId}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            signal,
        });
    } catch (error) {
        if (signal.aborted) {
            showError('Generation stopped.');
        } else {
            console.error('Streaming request failed', error);
            showError(`Streaming failed: ${String(error)}`);
        }
        return;
    }

    if (!res.ok) {
        const message = await res.text().catch(() => '');
        showError(message || `Streaming failed with HTTP ${res.status}.`);
        return;
    }

    if (!res.body) {
        showError('Streaming failed: the server returned no response body.');
        return;
    }

    const reader = res.body.getReader();
    const decoder = new TextDecoder();
    let buffer = '';
    let hasStarted = false;

    const appendText = (text: string) => {
        if (!hasStarted) {
            element.replaceChildren();
            element.setAttribute('aria-busy', 'false');
            hasStarted = true;
        }
        element.appendChild(document.createTextNode(text));
    };

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
                    appendText(delta);
                }
                return false;
            }

            if (json.type === 'done') {
                finalizeUiState();
                return true;
            }

            if (json.type === 'error') {
                const message = String(json?.data?.message ?? 'Unknown streaming error');
                showError(message);
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
        showError('Streaming ended before the response was completed.');
    } catch (err) {
        console.error('Streaming failed', err);
        showError(signal.aborted ? 'Generation stopped.' : `Streaming failed: ${String(err)}`);
    }

}
