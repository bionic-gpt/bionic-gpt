export const textareaSubmit = () => {
    // Persist a form to local storage.
    document.querySelectorAll('textarea.submit-on-enter').forEach((area) => {
        if(area instanceof HTMLTextAreaElement) {
            area.focus()
            area.addEventListener("keydown", (event) => {
    
                if(event instanceof KeyboardEvent) {
                    if (event.which === 13 && ! event.shiftKey) {
                        if (!event.repeat) {
    
                            // Find the containing form and submit it.
                            if(area.form) {
                                area.form.requestSubmit()
                            }
                        }
                        // Prevent the addition of a new line in the text field
                        event.preventDefault(); 
                    }
                }
            });
        }
    })
}