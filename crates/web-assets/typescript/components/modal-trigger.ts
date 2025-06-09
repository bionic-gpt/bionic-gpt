export const modalTriggers = () => {
    document.querySelectorAll('[data-target]').forEach(async (row) => {
        // Detect when a user clicks a row
        row.addEventListener('click', (event) => {
            event.stopImmediatePropagation()
            event.preventDefault()
            const attr = row.getAttribute('data-target');
            console.debug(attr)
            if (attr) {
                const modal = document.getElementById(attr);
                if (modal instanceof HTMLDialogElement) {
                    modal.showModal();
                    // Handle cancel-modal button click
                    modal.querySelectorAll('.cancel-modal').forEach((cancelBtn) => {
                        cancelBtn.addEventListener('click', () => {
                            modal.close(); // Close the modal when the cancel button is clicked
                        });
                    });
                } else {
                    console.log(`The drawer ${attr} not there`);
                }
            } else {
                console.log("modal-trigger could not find data-target");
            }
        });
    });
};
