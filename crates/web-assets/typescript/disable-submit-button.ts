export const disableSubmitButton = () => {
    // Disable submit buttons only when the form is valid.
    document.querySelectorAll('button[type="submit"]').forEach((button) => {
        button.addEventListener("click", function () {
            if (button instanceof HTMLButtonElement) {
                const form = button.form;

                // If the form is invalid (e.g. no file selected), let the browser
                // handle validation and keep the button active.
                if (form && !form.checkValidity()) {
                    return;
                }

                // Otherwise disable the button and show the loading text.
                setTimeout(() => {
                    const text = button.getAttribute("data-disabled-text");
                    if (text) {
                        button.innerHTML = text;
                        button.disabled = true;
                    }
                }, 1);
            }
        });
    });
};
