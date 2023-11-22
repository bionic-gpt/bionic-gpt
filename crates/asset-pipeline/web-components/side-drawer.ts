const template = document.createElement('template');

template.innerHTML = `
<div class="drawer" part="base">
    <div part="overlay" class="drawer__overlay" tabindex="-1">
    </div>
    <div part="panel" class="drawer__panel border-left" role="dialog" aria-modal="true"  tabindex="0">

        <header part="header" class="drawer__header border-b">
            <h4 part="title" class="drawer__title" id="title">Title</h4>
            <a href="#" class="drawer__close" name="x" library="system">X</a>
        </header>
        <div class="drawer__body">
        </div>
        <footer part="footer" class="drawer__footer">
            <slot name="footer"></slot>
        </footer>
    </div>
</div>
`

export class SideDrawer extends HTMLElement {

    constructor() {
        super()
        const bodyEle = this.querySelector("template[slot='body']")
        const footerEle = this.querySelector("template[slot='footer']")
        const titleText = this.attributes.getNamedItem('label')

        if(bodyEle != null && footerEle != null && titleText != null) {

            const body = bodyEle.cloneNode(true)
            const footer = footerEle.cloneNode(true)
            const title = titleText.value
            const templateNode = template.cloneNode(true)
    
            if(templateNode instanceof HTMLTemplateElement && body instanceof HTMLTemplateElement
                && footer instanceof HTMLTemplateElement) {
                const templateDocument = templateNode.content
                const drawerBody = templateDocument.querySelector(".drawer__body")
                const drawerFooter = templateDocument.querySelector(".drawer__footer")
                const templateTitle = templateDocument.querySelector(".drawer__title")
                const closeButton = templateDocument.querySelector(".drawer__close")
                const overlay = templateDocument.querySelector(".drawer__overlay")
                const panel = templateDocument.querySelector(".drawer__panel")

                if(drawerBody && drawerFooter && templateTitle && closeButton && overlay && panel) {

                    drawerBody.appendChild(body.content)
                    drawerFooter.appendChild(footer.content)
        
                    templateTitle.innerHTML = title
        
                    const thiz = this
        
                    closeButton.addEventListener("click", function(e) {
                        e.stopPropagation()
                        e.preventDefault()
                        thiz.open = false
                    });
        
                    overlay.addEventListener("click", function(e) {
                        e.stopPropagation()
                        e.preventDefault()
                        thiz.open = false
                    });
        
                    overlay.addEventListener('keydown', (event : Event) => {
                        console.log(event)
                        if(event instanceof KeyboardEvent) {
                            if (event.key === 'Escape') {
                                this.open = false
                            }
                        }
                      }, false);
        
                    // Catch all clicks in the panel so they don't propogate up to the document
                    panel.addEventListener("click", function(e) {
                        e.stopPropagation()
                    });
            
                    this.appendChild(templateDocument)
                } else {
                    console.error("side-drawer: could not find required elements.")
                }
            }
        } else {
            console.error("side-drawer: could not find required elements.")
        }

    }

    static get observedAttributes() {
        return ['open'];
    }

    get open(): Boolean {
        return Boolean(this.getAttribute('open'))
    }

    set open(value: Boolean) {
        this.setAttribute('open', value.toString())
    }

    attributeChangedCallback(name: string, oldVal: string, newVal: string) {
        const drawer = this.querySelector('.drawer')
        if (oldVal !== newVal) {
            switch (name) {
                case 'open':
                    var val = false
                    if(newVal == 'true') {
                        val = true
                    }
                    if(val == true && drawer) {
                        drawer.classList.remove('drawer--open')
                        drawer.classList.add('drawer--open')
                    } else if (drawer) {
                        drawer.classList.remove('drawer--open')
                    }
                    break;
            }
        }
    }
}

document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        customElements.define('side-drawer', SideDrawer);
    }
})
