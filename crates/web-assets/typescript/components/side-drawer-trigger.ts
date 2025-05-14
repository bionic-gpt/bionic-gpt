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
                    if (drawer instanceof HTMLDialogElement) {
                        drawer.showModal();
                        // Handle cancel-modal button click
                        drawer.querySelectorAll('.cancel-modal').forEach((cancelBtn) => {
                            cancelBtn.addEventListener('click', () => {
                                drawer.close(); // Close the modal when the cancel button is clicked
                            });
                        });
                    } else {
                        drawer.classList.remove('drawer--open')
                        drawer.classList.add('drawer--open')
                    }
                } else {
                    console.log(`The drawer ${attr} not there`)
                }
            } else {
                console.log("side-drawer-trigger could not find data-drawer-target")
            }
        })
    })
}