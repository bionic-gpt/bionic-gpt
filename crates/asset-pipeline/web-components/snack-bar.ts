const template = document.createElement('template');

template.innerHTML = `
<div class="snackbar-container">
</div>
`

const COOKIE_NAME = 'flash_aargh'

export class SnackBar extends HTMLElement {

    constructor() {
        super()
        const templateNode = template.cloneNode(true)

        const message = this.getCookie(COOKIE_NAME)
        if (templateNode instanceof HTMLTemplateElement && message != null) {
            const templateDocument = templateNode.content
            this.appendChild(templateDocument)

            const div = this.querySelector('.snackbar-container')
            if(div instanceof HTMLDivElement) {
                const p = document.createElement('p')
                const text = document.createTextNode(message);
                p.appendChild(text)
                div.prepend(p)
                this.deleteCookie(COOKIE_NAME)

                const actionButton = document.createElement('button');
                actionButton.className = 'action';
                actionButton.innerHTML ='DISMISS';
                actionButton.setAttribute('aria-label', 'Dismiss, Description for Screen Readers');
                actionButton.addEventListener('click', function() {
                    div.classList.add('close')
                })
                div.appendChild(actionButton)
            }

            setInterval(() => {
                if (div instanceof HTMLDivElement) {
                    div.classList.add('close')
                }
            }, 4000);
        }

    }

    getCookie(name: string): string | null {
        const nameLenPlus = (name.length + 1);
        return document.cookie
            .split(';')
            .map(c => c.trim())
            .filter(cookie => {
                return cookie.substring(0, nameLenPlus) === `${name}=`;
            })
            .map(cookie => {
                return decodeURIComponent(cookie.substring(nameLenPlus));
            })[0] || null;
    }

    deleteCookie(name: string) {
        document.cookie = name+'=; Max-Age=-99999999;';  
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('snack-bar', SnackBar)
    }
})