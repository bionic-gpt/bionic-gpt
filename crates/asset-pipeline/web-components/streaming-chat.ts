/**
 * Takes an attribute called prompt and sends it to the LLM API
 * Waits for the results to stream in and prints them in real time.
 */

export class StreamingChat extends HTMLElement {

    controller: AbortController
    result: string

    constructor() {
        super()
        const prompt = this.attributes.getNamedItem('prompt')
        const chatId = this.attributes.getNamedItem('chat-id')

        if (prompt && prompt.value && chatId && chatId.value) {
            this.streamResult(prompt.value, chatId.value)
        }

        const stopButton = document.getElementById('stop-processing')
        const thiz = this
        if (stopButton) {
            stopButton.addEventListener('click', () => {
                if (thiz.controller) {
                    thiz.controller.abort()
              }
            })
        }
    }

    async streamResult(prompt: string, chatId: string) {

        // Create a new AbortController instance
        this.controller = new AbortController();
        const signal = this.controller.signal;

        try {
            const response = await fetch(`/completions/${chatId}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    model: 'ggml-gpt4all-j',
                    prompt: prompt,
                    stream: true,
                }),
                signal
            });

            const reader = response.body?.pipeThrough(new TextDecoderStream()).getReader();
            this.result = '';
            while (true && reader) {
                // eslint-disable-next-line no-await-in-loop
                const { value, done } = await reader.read();
                if (done) break;
                let dataDone = false;
                const arr = value.split(/\r?\n/);
                arr.forEach((data) => {
                    if (data.length === 0) return; // ignore empty message
                    if (data.startsWith(':')) return; // ignore sse comment message
                    if (data.substring(6) === '[DONE]') {
                        dataDone = true;
                    } else {
                        const json = JSON.parse(data.substring(6));
                        if (json.choices[0].text) {
                            this.result += json.choices[0].text
                            this.innerHTML = `${this.result}`;
                        }
                    }
                });
                if (dataDone) break;
            }
        } catch (error) {
            // Handle fetch request errors
            if (signal.aborted) {
                this.innerHTML = "Request aborted."
                this.result = 'Request aborted.'
            } else {
                console.error("Error:", error);
                this.innerText = "Error occurred while generating."
                this.result = 'Error occurred while generating.'
            }
        } finally {
            // Save the results
            const form = document.getElementById(`chat-form-${chatId}`)
            const llmResult = document.getElementById(`chat-result-${chatId}`)

            if (form instanceof HTMLFormElement && llmResult instanceof HTMLInputElement) {
                llmResult.value = this.result
                this.result = ''
                form.submit()
            }
        }
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('streaming-chat', StreamingChat)
    }
})