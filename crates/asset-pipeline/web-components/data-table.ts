export class DataTable extends HTMLElement {

    constructor() {
        super()
    }
    connectedCallback() {
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('data-table', DataTable)
    }
})