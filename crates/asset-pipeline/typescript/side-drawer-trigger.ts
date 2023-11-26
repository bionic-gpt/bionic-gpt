export const triggers = () => {
    document.querySelectorAll('[data-drawer-target]').forEach(async (row) => {
        // Detect when a user clicks a row
        row.addEventListener('click', (event) => {
            event.stopImmediatePropagation()
            event.preventDefault()
            const attr = row.getAttribute('data-drawer-target');
            if(attr) {
                const drawer = document.getElementById(attr)
                if(drawer) {
                    drawer.classList.remove('drawer--open')
                    drawer.classList.add('drawer--open')
                } else {
                    console.log(`The drawer ${attr} not there`)
                }
            } else {
                console.log("side-drawer-trigger could not find data-drawer-target")
            }
        })
    })
}