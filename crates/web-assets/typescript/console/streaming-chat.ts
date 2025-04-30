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

    const stream = Stream.fromSSEResponse(res, abortController);
    const runner = ChatCompletionStream.fromReadableStream(stream.toReadableStream());

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

    runner.on('content', (_delta, snapshot) => {
        const delta = _delta as DeltaChunk;
        console.log(delta);

        if (delta.tool_calls) {
            console.log('Got a function call')
            isFunctionCall = true;
            if (delta.tool_calls.name) functionCall.name = delta.tool_calls.name;
            if (delta.tool_calls.arguments) functionCall.arguments += delta.tool_calls.arguments;
        }

        if (!isFunctionCall) {
            element.innerHTML = markdown.markdown(snapshot);
            result = snapshot;
        }
    });

    runner.on('error', (err: OpenAIError) => {
        console.log(err)
        element.innerHTML += `${err}`;
        result += err.toString();
    });

    runner.on('end', () => {
        console.log('Streaming ended.');
        submitResults();
    });

}

