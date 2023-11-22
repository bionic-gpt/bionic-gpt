document.addEventListener('turbo:load', () => {
    console.log('here')
    // We can create a trigger to open drawers
    document.querySelectorAll('a.theme').forEach(async (row) => {
        console.log(row)
        // Detect when a user clicks a row
        row.addEventListener('click', (event) => {

            const theme = row.getAttribute('href')
            if(theme) {
                console.log(theme)
            }
            event.stopImmediatePropagation()
        })
    })
})