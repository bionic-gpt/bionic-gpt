import 'highlight.js/styles/a11y-dark.css'
import hljs from 'highlight.js';
import '@github/relative-time-element';

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

const runWhenPresent = (selector: string, callback: () => void) => {
    if (document.querySelector(selector)) {
        callback()
    }
}

function loadEverything() {
    hljs.highlightAll()

    runWhenPresent('[data-target]', modalTriggers)
    runWhenPresent('[data-clickable-link]', clickableCard)
    runWhenPresent('.api-keys-toggle-visibility', toggleVisibility)
    runWhenPresent('textarea.auto-expand', autoExpand)
    runWhenPresent('#streaming-chat', streamingChat)
    runWhenPresent('pre.json, .format-json', formatter)
    runWhenPresent('pre code', copyPaste)
    runWhenPresent('#snackbar', snackBar)
    runWhenPresent('.copy-trigger', copy)
    runWhenPresent('.select-menu', selectMenu)
    runWhenPresent('.read-aloud', readAloud)
    runWhenPresent('form.remember', rememberForm)
    runWhenPresent('textarea.submit-on-enter', textareaSubmit)
    runWhenPresent('form[data-disable-submit], button[data-disable-submit]', disableSubmitButton)
    runWhenPresent('#speech-to-text-button', speechToText)
    runWhenPresent('#attach-button', fileUpload)
    runWhenPresent('[data-example-prompts]', examplePrompts)
    runWhenPresent('#toggleButton', initializeSidebar)

    initInstantPage()

    setTheme()
    runWhenPresent('[data-theme-switcher]', themeSwitcher)
}

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', loadEverything);
} else {
    loadEverything();
}
