export const drawers = () => {
    document.querySelectorAll('div.side-drawer').forEach(async (drawer) => {
        
        const drawerBody = drawer.querySelector(".drawer__body")
        const drawerFooter = drawer.querySelector(".drawer__footer")
        const closeButton = drawer.querySelector(".drawer__close")
        const overlay = drawer.querySelector(".drawer__overlay")
        const panel = drawer.querySelector(".drawer__panel")

        if(drawerBody && drawerFooter && closeButton && overlay && panel) {

            closeButton.addEventListener("click", function(e) {
                e.stopPropagation()
                e.preventDefault()
                close(drawer)
            });

            overlay.addEventListener("click", function(e) {
                e.stopPropagation()
                e.preventDefault()
                close(drawer)
            });

            overlay.addEventListener('keydown', (event : Event) => {
                console.log(event)
                if(event instanceof KeyboardEvent) {
                    if (event.key === 'Escape') {
                        close(drawer)
                    }
                }
              }, false);
    
        } else {
            console.error("side-drawer: could not find required elements.")
        }
    })
}

function close(overlay) {
    overlay.classList.remove('drawer--open')
}