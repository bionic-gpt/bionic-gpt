import './scss/index.scss'

// Highlight JS
import hljs from 'highlight.js';

// Web components
import { SnackBar } from './web-components/snack-bar'
import '@github/relative-time-element';

// Misc.
import { triggers } from './typescript/side-drawer-trigger'
import { initializeSidebar } from './typescript/responsive-nav'
import { streamingChat } from './typescript/streaming-chat'
import { drawers } from './typescript/side-drawer'
import { formatter } from './typescript/format-json'
import { copyPaste } from './typescript/copy-paste'
import { copy } from './typescript/copy'
import { selectMenu } from './typescript/select-menu'
import { readAloud } from './typescript/read-aloud'
import { modelChanged } from './typescript/select-menu-changed'
import './typescript/remember-form'
import './typescript/textarea-submit'
import './typescript/update-sidebar'
import './typescript/refresh-frame'
import './typescript/disable-submit-button'
import './typescript/theme-switcher'

// Hotwired Turbo
import '@hotwired/turbo'

// Set everything up
function loadEverything() {
    hljs.highlightAll()
    if(customElements.get('snack-bar') === undefined) {
        customElements.define('snack-bar', SnackBar);
    }
    triggers()
    drawers()
    formatter()
    streamingChat()
    copyPaste()
    selectMenu()
    modelChanged()
    copy()
    readAloud()
    initializeSidebar()
}

// Called when the page loads i.e. after a page refresh
document.addEventListener('turbo:load', () => {
    loadEverything()
})

// Called when you click a link in the sidebar and it updates the main content
document.addEventListener('turbo:frame-load', (event: Event) => {
    const frame = event.target as HTMLIFrameElement | null;
    if (frame?.id === "main-content") {
        const url = new URL(frame.src);
        history.pushState({}, '', url.toString());
        loadEverything();
    }
});

document.addEventListener('turbo:fetch-request-error', (e) => {
    console.log('turbo:fetch-request-error')
    location.reload()
})