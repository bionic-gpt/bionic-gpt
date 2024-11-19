import './scss/index.scss'

// Highlight JS
import hljs from 'highlight.js';

// Web components
import '@github/relative-time-element';

// Misc.
import { triggers } from './typescript/components/side-drawer-trigger'
import { toggleVisibility } from './typescript/api-keys/toggle-visibility'
import { autoExpand } from './typescript/console/auto-expand'
import { modalTriggers } from './typescript/components/modal-trigger'
import { initializeSidebar } from './typescript/layout/responsive-nav'
import { streamingChat } from './typescript/console/streaming-chat'
import { drawers } from './typescript/components/side-drawer'
import { formatter } from './typescript/console/format-json'
import { copyPaste } from './typescript/console/copy-paste'
import { snackBar } from './typescript/layout/snackbar'
import { copy } from './typescript/console/copy'
import { selectMenu } from './typescript/components/select-menu'
import { readAloud } from './typescript/console/read-aloud'
import { modelChanged } from './typescript/components/select-menu-changed'
import { rememberForm } from './typescript/remember-form'
import { textareaSubmit } from './typescript/textarea-submit'
import { updateSidebar } from './typescript/layout/update-sidebar'
import { refreshFrame } from './typescript/layout/refresh-frame'
import { disableSubmitButton } from './typescript/disable-submit-button'
import { themeSwitcher, setTheme } from './typescript/layout/theme-switcher'

// Hotwired Turbo
import '@hotwired/turbo'

// Set everything up
function loadEverything() {
    hljs.highlightAll()
    triggers()
    toggleVisibility()
    autoExpand()
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

// Called when you click a link in the sidebar and it updates the main content
document.addEventListener('turbo:frame-load', (event: Event) => {
    console.log('turbo:frame-load')

    loadEverything();

    const frame = event.target as HTMLIFrameElement | null;
    if (frame?.id === "main-content") {
        const url = new URL(frame.src);
        history.pushState({}, '', url.toString());
    }
});

// ERROR HANDLING
document.addEventListener('turbo:before-fetch-response', (event: Event) => {
    console.log('turbo:before-fetch-response')
    const customEvent = event as CustomEvent<{ fetchResponse?: { succeeded: boolean; response: Response } }>;

    if (customEvent.detail?.fetchResponse) {
        const { fetchResponse } = customEvent.detail;

        if (!fetchResponse.succeeded || !fetchResponse.response.ok) {
            // Extract the response text and display it as an error message
            fetchResponse.response.text().then(responseText => {
                alert(responseText || "Failed to load the content. Please try again later.");
            }).catch(() => {
                alert("Fetch response failed");
            });
        }
    }
    event.stopPropagation();
});
