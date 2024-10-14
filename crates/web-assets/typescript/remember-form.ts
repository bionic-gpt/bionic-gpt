export const rememberForm = () => {
    // Persist a form to local storage.
    document.querySelectorAll('form.remember').forEach((form) => {
        const forget = form.getAttribute('data-remember-reset')
        const name = form.getAttribute('data-remember-name')
        if (forget != null && name != null) {
            form.querySelectorAll('select').forEach((select) => {
                if (select instanceof HTMLSelectElement) {
                    // Restore from local storage
                    if (forget && forget == "true") {
                        localStorage.removeItem(name + '::' + select.name)
                    } else {
                        const selectedIndex = localStorage.getItem(name + '::' + select.name)
                        if (selectedIndex) {
                            select.selectedIndex = parseInt(selectedIndex)
                        }
                    }
                    // Add a click handler
                    select.addEventListener("change", () => {
                        localStorage.setItem(name + '::' + select.name, '' + select.selectedIndex)
                    })
                }
            })
        }
    })
}
