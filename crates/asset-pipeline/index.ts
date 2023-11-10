import './scss/index.scss'

// Web components
import './web-components/side-drawer'
import './web-components/side-drawer-trigger'
import './web-components/snack-bar'
import './web-components/data-table'
import './web-components/streaming-chat'
import './web-components/response-formatter'
import '@github/relative-time-element';
import hljs from 'highlight.js';

document.addEventListener('turbo:load', () => {
    hljs.highlightAll()
})

// Misc.
import './remember-form'
import './textarea-submit'
import './filter-trigger'
import './select-div'
import './refresh-status'
import './disable-submit-button'

import '@primer/view-components/app/components/primer/tab_container_component';

// Hotwired Turbo
import '@hotwired/turbo'