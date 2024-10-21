import './scss/index.scss'

// Highlight JS
import hljs from 'highlight.js';

// Web components
import '@github/relative-time-element';

// Misc.
import { triggers } from './typescript/side-drawer-trigger'
import { modalTriggers } from './typescript/modal-trigger'
import { initializeSidebar } from './typescript/responsive-nav'
import { streamingChat } from './typescript/streaming-chat'
import { drawers } from './typescript/side-drawer'
import { formatter } from './typescript/format-json'
import { copyPaste } from './typescript/copy-paste'
import { snackBar } from './typescript/snackbar'
import { copy } from './typescript/copy'
import { selectMenu } from './typescript/select-menu'
import { readAloud } from './typescript/read-aloud'
import { modelChanged } from './typescript/select-menu-changed'
import { rememberForm } from './typescript/remember-form'
import { textareaSubmit } from './typescript/textarea-submit'
import { updateSidebar } from './typescript/update-sidebar'
import { refreshFrame } from './typescript/refresh-frame'
import { disableSubmitButton } from './typescript/disable-submit-button'
import { themeSwitcher, setTheme } from './typescript/theme-switcher'

// Hotwired Turbo
import '@hotwired/turbo'

// Set everything up
function loadEverything() {
    hljs.highlightAll()
    triggers()
    modalTriggers()
    drawers()
    formatter()
    streamingChat()
    copyPaste()
    snackBar()
    selectMenu()
    modelChanged()
    copy()
    readAloud()
    initializeSidebar()
    rememberForm()
    textareaSubmit()
    updateSidebar()
    refreshFrame()
    disableSubmitButton()

    // Apply dark or light mode
    setTheme()
    themeSwitcher()
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