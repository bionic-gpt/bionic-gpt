import { Markdown } from "./response-formatter"
import { Stream } from 'openai/streaming';
import { ChatCompletionStream } from 'openai/lib/ChatCompletionStream';

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

    // Create a new AbortController instance
    const abortController = new AbortController();
    const signal = abortController.signal;
    const markdown = new Markdown()
    var result = ''


    const stopButton = document.getElementById('stop-processing')
    if (stopButton) {
        stopButton.addEventListener('click', () => {
            if (abortController) {
                abortController.abort()
          }
        })
    }

    fetch(`/completions/${chatId}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        signal
    }).then(async (res) => {
        const stream = Stream.fromSSEResponse(res, abortController)
        const runner = ChatCompletionStream.fromReadableStream(stream.toReadableStream())

        runner.on('content', (delta, snapshot) => {
            element.innerHTML = markdown.markdown(snapshot)
            result = snapshot
        })

        runner.on('end', () => {
            console.log("Finished, Saving the results")
            const form = document.getElementById(`chat-form-${chatId}`)
            const llmResult = document.getElementById(`chat-result-${chatId}`)

            if (form instanceof HTMLFormElement && llmResult instanceof HTMLInputElement) {
                llmResult.value = result
                form.requestSubmit()
            }
        })
    });
}
