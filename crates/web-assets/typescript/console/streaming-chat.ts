import { Markdown } from "./markdown"

export const streamingChat = () => {
    const chat = document.getElementById('streaming-chat')

    const chatId = chat?.dataset.chatid
    const prompt = chat?.dataset.prompt

    if (chatId && chat) {
        console.log('Performing streaming')
        streamResult(chatId, chat)
    }
}

async function streamResult(chatId: string, element: HTMLElement) {
    const abortController = new AbortController();
    const signal = abortController.signal;
    const markdown = new Markdown();
    let result = '';

    const stopButton = document.getElementById('streaming-button');
    const stopListener = (event: Event) => {
        console.log('Attempting to abort stream.');
        abortController.abort("User aborted");
    };

    if (stopButton) {
        stopButton.addEventListener('click', stopListener);
    } else {
        console.error('Debug: did not find stop button');
    }

    // We submit a form with the chta_id and the LLM response we have so far.
    // The response should already have been saved by the LLM streaming proxy code
    // However in some cases (i.e. abort) this is not the case.
    // In the back end, if we don't have a response, we'll use this one.
    const submitResults = () => {
        console.log('Submitting results...');
        const form = document.getElementById(`chat-form-${chatId}`);
        const llmResult = document.getElementById(`chat-result-${chatId}`);

        if (form instanceof HTMLFormElement && llmResult instanceof HTMLInputElement) {
            llmResult.value = result;
            try {
                form.requestSubmit();
            } catch (error) {
                console.error('Error submitting results:', error);
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

    let isFunctionCall = false;
    let functionCall = {
        name: '',
        arguments: '',
    };

    interface DeltaChunk {
        content?: string;
        tool_calls?: {
            name?: string;
            arguments?: string;
        };
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
                if (data === '[DONE]') {
                    console.log('Streaming ended.');
                    submitResults();
                    return;
                }

                try {
                    const json = JSON.parse(data);
                    const delta = json.choices?.[0]?.delta || {};
                    if (delta.content) {
                        snapshot += delta.content;
                    }
                    const deltaChunk: DeltaChunk = {};
                    if (delta.content) deltaChunk.content = delta.content;
                    const tool = delta.tool_calls?.[0];
                    if (tool && tool.function) {
                        deltaChunk.tool_calls = {
                            name: tool.function.name,
                            arguments: tool.function.arguments || '',
                        };
                    }

                    if (deltaChunk.tool_calls) {
                        console.log('Got a function call');
                        isFunctionCall = true;
                        if (deltaChunk.tool_calls.name) functionCall.name = deltaChunk.tool_calls.name;
                        if (deltaChunk.tool_calls.arguments) functionCall.arguments += deltaChunk.tool_calls.arguments;
                    }

                    if (!isFunctionCall && delta.content) {
                        element.innerHTML = markdown.markdown(snapshot);
                        result = snapshot;
                    }
                } catch (e) {
                    console.error('Error parsing chunk', e);
                }
            }
        }
        console.log('Streaming ended.');
        submitResults();
    } catch (err) {
        console.log(err);
        element.innerHTML += `${err}`;
        result += String(err);
        submitResults();
    }

}

