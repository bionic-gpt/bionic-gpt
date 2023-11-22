document.addEventListener('turbo:frame-render', () => {
    // We can create a trigger to open drawers
    document.querySelectorAll('a.theme').forEach(async (row) => {
        // Detect when a user clicks a row
        row.addEventListener('click', (event) => {

            var theme = row.getAttribute('href')
            if(theme) {
                theme = theme.substring(1)
                const body = document.getElementsByTagName('body')[0]
    
                if(body) {
                    body.setAttribute('data-theme', theme)
                }
            }

            event.stopImmediatePropagation()
        })
    })
})