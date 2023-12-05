document.addEventListener('turbo:frame-render', () => {
    // We can create a trigger to open drawers
    document.querySelectorAll('a.theme').forEach(async (link) => {
        // Detect when a user clicks a row
        link.addEventListener('click', (event) => {

            var theme = link.getAttribute('href')
            if(theme) {
                theme = theme.substring(1)
                const body = document.getElementsByTagName('body')[0]
    
                if(body) {
                    localStorage.setItem('theme', theme)
                    setTheme()
                }
            }
            if(link instanceof HTMLAnchorElement) {
                link.blur()
            }
            event.stopImmediatePropagation()
        })
    })
})

document.addEventListener('turbo:load', () => {
    setTheme()
})

function setTheme() {
    const theme = localStorage.getItem('theme')
    const body = document.getElementsByTagName('body')[0]
    const sidenav = document.getElementsByTagName('nav')[0]
    const main = document.getElementById('main-content')

    if(body && sidenav && main) {
        body.removeAttribute('data-theme')
        sidenav.removeAttribute('data-theme')
        main.removeAttribute('data-theme')

        if(theme) {
            if(theme == 'mixed') {
                sidenav.setAttribute('data-theme', 'dark')
                main.setAttribute('data-theme', 'light')
            } else {
                body.setAttribute('data-theme', theme)
            }
        } else {
            sidenav.setAttribute('data-theme', 'dark')
            main.setAttribute('data-theme', 'light')
        }
    }
}