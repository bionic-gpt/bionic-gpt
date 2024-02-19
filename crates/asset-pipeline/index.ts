import './scss/index.scss'

// Web components
import { SnackBar } from './web-components/snack-bar'
import { ResponseFormatter } from './web-components/response-formatter'
import '@github/relative-time-element';

// Misc.
import { triggers } from './typescript/side-drawer-trigger'
import { streamingChat } from './typescript/streaming-chat'
import { drawers } from './typescript/side-drawer'
import { formatter } from './typescript/format-json'
import './typescript/remember-form'
import './typescript/textarea-submit'
import './typescript/update-sidebar'
import './typescript/refresh-status'
import './typescript/disable-submit-button'
import './typescript/theme-switcher'

// Hotwired Turbo
import '@hotwired/turbo'

// Set everything up
function loadEverything() {
    if(customElements.get('response-formatter') === undefined) {
        customElements.define('response-formatter', ResponseFormatter)
        customElements.define('snack-bar', SnackBar);
    }
    triggers()
    drawers()
    formatter()
    streamingChat()
}

document.addEventListener('turbo:load', () => {
    loadEverything()
})

document.addEventListener('turbo:fetch-request-error', (e) => {
    console.log('turbo:fetch-request-error')
    location.reload()
})