import './scss/index.scss'

// Web components
import { SideDrawer } from './web-components/side-drawer'
import { SnackBar } from './web-components/snack-bar'
import { StreamingChat } from './web-components/streaming-chat'
import { ResponseFormatter } from './web-components/response-formatter'
import '@github/relative-time-element';
import hljs from 'highlight.js';

// Misc.
import { triggers} from './typescript/side-drawer-trigger'
import './typescript/remember-form'
import './typescript/textarea-submit'
// Remove this?
//import './typescript/filter-trigger'
import './typescript/update-sidebar'
import './typescript/refresh-status'
import './typescript/disable-submit-button'
import './typescript/theme-switcher'

// Hotwired Turbo
import '@hotwired/turbo'

// Set everything up
function loadEverything() {
    hljs.highlightAll()
    if(customElements.get('response-formatter') === undefined) {
        customElements.define('response-formatter', ResponseFormatter)
        customElements.define('side-drawer', SideDrawer);
        customElements.define('streaming-chat', StreamingChat);
        customElements.define('snack-bar', SnackBar);
    }
    triggers()
}

document.addEventListener('turbo:load', () => {
    console.log('turbo:load')
    loadEverything()
})