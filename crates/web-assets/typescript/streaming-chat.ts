import { Markdown } from "./markdown"
import { Stream } from 'openai/streaming';
import { ChatCompletionStream } from 'openai/lib/ChatCompletionStream';
import { OpenAIError } from "openai/error.mjs";

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
    const stopListener = () => {
        console.log('Attempting to abort stream.');
        abortController.abort();
    };

    if (stopButton) {
        stopButton.addEventListener('click', stopListener);
    } else {
        console.error('Debug: did not find stop button');
    }

    const submitResults = () => {
        console.log('Submitting results...');
        const form = document.getElementById(`chat-form-${chatId}`);
        const llmResult = document.getElementById(`chat-result-${chatId}`);

        if (form instanceof HTMLFormElement && llmResult instanceof HTMLInputElement) {
            result = result || "Result was blank" 
            llmResult.value = result;
            try {
                form.requestSubmit();
            } catch(error) {
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

    const stream = Stream.fromSSEResponse(res, abortController);
    const runner = ChatCompletionStream.fromReadableStream(stream.toReadableStream());

    runner.on('content', (delta, snapshot) => {
        element.innerHTML = markdown.markdown(snapshot);
        result = snapshot;
    });

    runner.on('error', (err: OpenAIError) => {
        element.innerHTML += `${err}`;
        result += err.toString();
    });

    runner.on('end', () => {
        console.log('Streaming ended.');
        submitResults();
    });
    
}

