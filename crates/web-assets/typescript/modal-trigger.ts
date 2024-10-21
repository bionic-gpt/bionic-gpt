export const modalTriggers = () => {
    document.querySelectorAll('[data-modal-target]').forEach(async (row) => {
        // Detect when a user clicks a row
        row.addEventListener('click', (event) => {
            event.stopImmediatePropagation()
            event.preventDefault()
            const attr = row.getAttribute('data-modal-target');
            if(attr) {
                const modal = document.getElementById(attr)
                if(modal instanceof HTMLDialogElement) {
                    modal.showModal()
                } else {
                    console.log(`The drawer ${attr} not there`)
                }
            } else {
                console.log("modal-trigger could not find data-modal-target")
            }
        })
    })
}