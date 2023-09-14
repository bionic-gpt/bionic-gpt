+++
title = "Streaming Responses"
description = "T"
draft = false
weight = 20
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

The user doesn't want to ask a question and wait 20 seconds for a response. However, they will tolerate waiting 20 seconds if each work in the response is displayed on the screen as it is calculates.

![Example Response Streaming](../streaming-response.webp)

## LocalAI Streaming

Luckily [Local AI](https://github.com/go-skynet/LocalAI) supports streaming (as does the Open AI API) so we can leverage this from the browser.

The general idea would be to create a web component that when it appears on the screen streams the response via the API.

## Calling from the browser

This Javascript Proof of Concept shows how we can stream data from the API into the browser.

It can be cut and pasted into the browser console.

```typescript
/**
 * Takes an attribute called prompt and sends it to the LLM API
 * Waits for the results to stream in and prints them in real time.
 */

export class StreamingChat extends HTMLElement {

    constructor() {
        super()
        const prompt = this.attributes.getNamedItem('prompt')

        if(prompt && prompt.value) {
            this.streamResult(prompt.value)
        }
    }

    async streamResult(prompt: string) {

        const response = await fetch('http://localhost:7700/v1/completions', {
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
            const arr = value.split('\n');
            arr.forEach((data) => {
            if (data.length === 0) return; // ignore empty message
            if (data.startsWith(':')) return; // ignore sse comment message
            if (data === 'data: [DONE]') {
                dataDone = true;
            }
            const json = JSON.parse(data.substring(6));
            result += json.choices[0].text
            this.innerHTML = `${result}`;
            console.log(json.choices[0].text);
            });
            if (dataDone) break;
        }
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('streaming-chat', StreamingChat)
    }
})
```

## Consequences

This one was a tricky one to fix due to timeouts, however this article really helped.

[https://medium.com/@kaitmore/server-sent-events-http-2-and-envoy-6927c70368bb](https://medium.com/@kaitmore/server-sent-events-http-2-and-envoy-6927c70368bb)