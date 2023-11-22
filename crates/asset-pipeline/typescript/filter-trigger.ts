document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        // We can create a trigger to open drawers
        document.querySelectorAll('[data-filter-target]').forEach(async (row) => {
            // Detect when a user clicks a row
            row.addEventListener('click', (event) => {
    
                const target = row.getAttribute('data-filter-target')
                const lastRow = row.getAttribute('data-filter-last-row')
                if(target != null && lastRow != null) {
                    const form = document.getElementById(target)
                    if(form instanceof HTMLFormElement) {
                        const lastRowHiddenField = document.getElementById('last-row-id')
                        if(lastRowHiddenField instanceof HTMLInputElement) {
                            lastRowHiddenField.value = lastRow
                            form.submit()
                        }
                    }
                }
                event.stopImmediatePropagation()
            })
        })
    }
})