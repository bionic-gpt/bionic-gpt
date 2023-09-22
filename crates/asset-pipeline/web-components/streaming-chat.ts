/**
 * Takes an attribute called prompt and sends it to the LLM API
 * Waits for the results to stream in and prints them in real time.
 */

export class StreamingChat extends HTMLElement {

    constructor() {
        super()
        const prompt = this.attributes.getNamedItem('prompt')
        const chatId = this.attributes.getNamedItem('chat-id')

        if(prompt && prompt.value && chatId && chatId.value) {
            this.streamResult(prompt.value, chatId.value)
        }
    }

    async streamResult(prompt: string, chatId: string) {

        const response = await fetch('/v1/completions', {
            method: 'POST',
            headers: {
            'Content-Type': 'application/json',
            },
            body: JSON.stringify({
            model: 'ggml-gpt4all-j',
            prompt: prompt,
            stream: true,
            }),
        });

        const reader = response.body?.pipeThrough(new TextDecoderStream()).getReader();
        var result = '';
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
                    if(json.choices[0].text) {
                        result += json.choices[0].text
                        this.innerHTML = `${result}`;
                    }
                }
            });
            if (dataDone) break;
        }

        // Save the results

        const form = document.getElementById(`chat-form-${chatId}`)
        const llmResult = document.getElementById(`chat-result-${chatId}`)

        if(form instanceof HTMLFormElement && llmResult instanceof HTMLInputElement) {
            llmResult.value = result
            form.submit()
        }
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('streaming-chat', StreamingChat)
    }
})