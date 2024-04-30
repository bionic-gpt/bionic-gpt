import './scss/index.scss'

// Web components
import { SnackBar } from './web-components/snack-bar'
import '@github/relative-time-element';

// Misc.
import { triggers } from './typescript/side-drawer-trigger'
import { streamingChat } from './typescript/streaming-chat'
import { drawers } from './typescript/side-drawer'
import { formatter } from './typescript/format-json'
import { copyPaste } from './typescript/copy-paste'
import './typescript/remember-form'
import './typescript/textarea-submit'
import './typescript/update-sidebar'
import './typescript/refresh-frame'
import './typescript/disable-submit-button'
import './typescript/theme-switcher'
import './typescript/copy-paste'

// Hotwired Turbo
import '@hotwired/turbo'

// Set everything up
function loadEverything() {
    if(customElements.get('snack-bar') === undefined) {
        customElements.define('snack-bar', SnackBar);
    }
    triggers()
    drawers()
    formatter()
    streamingChat()
    copyPaste()
}

document.addEventListener('turbo:load', () => {
    loadEverything()
})

document.addEventListener('turbo:fetch-request-error', (e) => {
    console.log('turbo:fetch-request-error')
    location.reload()
})