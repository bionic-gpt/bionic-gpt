/**
 * Takes an attribute called prompt and sends it to the LLM API
 * Waits for the results to stream in and prints them in real time.
 */

export class ResponseFormatter extends HTMLElement {

    constructor() {
        super()
        const response = this.attributes.getNamedItem('response')

        if(response && response.value) {

            const md = response.value
                .trim()
                .replace(/^#{1,6} (.*)$/gim, '<h3>$1</h3>')
                .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
                .replace(/__(.*?)__/g, '<strong>$1</strong>')
                .replace(/\*(.*?)\*/g, '<em>$1</em>')
                .replace(/_(.*?)_/g, '<em>$1</em>')
                .replace(/```.*?\n([\s\S]*?)```/g, '<pre><code>$1</code></pre>')
                .replace(/`(.*?)`/g, '<code>$1</code>')
                .replace(/\n/gim, '<br />')
    
            this.innerHTML = `${md}`
        }
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('response-formatter', ResponseFormatter)
    }
})