export const disableSubmitButton = () => {
    // Persist a form to local storage.
    document.querySelectorAll('button[type="submit"]').forEach((button) => {
        button.addEventListener("click", function() {
            if(button instanceof HTMLButtonElement) {
                setTimeout(() => {
    
                    const text = button.getAttribute("data-disabled-text")
                    if(text) {
                        button.innerHTML = text
                        button.disabled = true
                    }

                }, 1)
            }
        }, {once : true});
    })
}
