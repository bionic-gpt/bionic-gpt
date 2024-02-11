/**
 * Takes an attribute called prompt and sends it to the LLM API
 * Waits for the results to stream in and prints them in real time.
 */

import { ResponseFormatter } from "./response-formatter"
import { Stream } from 'openai/streaming';
import { ChatCompletionStream } from 'openai/lib/ChatCompletionStream';

export class StreamingChat extends HTMLElement {

    controller: AbortController
    result: string

    constructor() {
        super()
        const chatId = this.attributes.getNamedItem('chat-id')

        if (chatId && chatId.value) {
            this.streamResult(chatId.value)
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

    async streamResult(chatId: string) {

        // Create a new AbortController instance
        this.controller = new AbortController();
        const signal = this.controller.signal;
        const markdown = new ResponseFormatter()

        fetch(`/completions/${chatId}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            signal
        }).then(async (res) => {
            const stream = Stream.fromSSEResponse(res, this.controller)
            const runner = ChatCompletionStream.fromReadableStream(stream.toReadableStream())

            runner.on('content', (delta, snapshot) => {
                this.innerHTML  = markdown.markdown(snapshot)
                this.result = snapshot
            })

            runner.on('end', () => {

                console.log("Saving the results")
                const form = document.getElementById(`chat-form-${chatId}`)
                const llmResult = document.getElementById(`chat-result-${chatId}`)
    
                if (form instanceof HTMLFormElement && llmResult instanceof HTMLInputElement) {
                    llmResult.value = this.result
                    this.result = ''
                    form.requestSubmit()
                }
            })
        });
    }
}