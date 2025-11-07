import 'highlight.js/styles/a11y-dark.css'
// Highlight JS
import hljs from 'highlight.js';

// Web components
import '@github/relative-time-element';

// Misc.
import { modalTriggers } from './typescript/components/modal-trigger'
import { clickableCard } from './typescript/components/clickable-card'
import { toggleVisibility } from './typescript/api-keys/toggle-visibility'
import { autoExpand } from './typescript/console/auto-expand'
import { initializeSidebar } from './typescript/layout/responsive-nav'
import { streamingChat } from './typescript/console/streaming-chat'
import { formatter } from './typescript/console/format-json'
import { copyPaste } from './typescript/console/copy-paste'
import { snackBar } from './typescript/layout/snackbar'
import { copy } from './typescript/console/copy'
import { selectMenu } from './typescript/components/select-menu'
import { readAloud } from './typescript/console/read-aloud'
import { rememberForm } from './typescript/remember-form'
import { textareaSubmit } from './typescript/textarea-submit'
import { disableSubmitButton } from './typescript/disable-submit-button'
import { themeSwitcher, setTheme } from './typescript/layout/theme-switcher'
import { speechToText } from './typescript/console/speach-to-text';
import { fileUpload } from './typescript/console/file-upload';
import { examplePrompts } from './typescript/console/example-prompts';
import { initInstantPage } from './typescript/instant-page';

// Set everything up
function loadEverything() {
    hljs.highlightAll()
    modalTriggers()
    clickableCard()
    toggleVisibility()
    autoExpand()
    examplePrompts()
    formatter()
    streamingChat()
    copyPaste()
    snackBar()
    selectMenu()
    copy()
    speechToText()
    readAloud()
    initializeSidebar()
    rememberForm()
    textareaSubmit()
    disableSubmitButton()
    fileUpload()
    initInstantPage()

    // Apply dark or light mode
    setTheme()
    themeSwitcher()
}

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', loadEverything);
} else {
    loadEverything();
}
