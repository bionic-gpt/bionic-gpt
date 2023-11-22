import './scss/index.scss'

// Web components
import './web-components/side-drawer'
import './web-components/side-drawer-trigger'
import './web-components/snack-bar'
import './web-components/streaming-chat'
import './web-components/response-formatter'
import '@github/relative-time-element';
import hljs from 'highlight.js';

document.addEventListener('turbo:load', () => {
    hljs.highlightAll()
})

// Misc.
import './typescript/remember-form'
import './typescript/textarea-submit'
import './typescript/filter-trigger'
import './typescript/select-div'
import './typescript/refresh-status'
import './typescript/disable-submit-button'
import './typescript/theme-switcher'

// Hotwired Turbo
import '@hotwired/turbo'