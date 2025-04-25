export const selectMenu = () => {
    const selectMenus = document.querySelectorAll('.select-menu'); // Select all elements with class 'select-menu'

    selectMenus.forEach(selectMenu => {
        const selectedOption = selectMenu.querySelector('.selected-option') as HTMLElement;
        const selectedOptionSpan = selectMenu.querySelector('.selected-option span') as HTMLElement;
        const options = selectMenu.querySelector('.options') as HTMLElement;
        
        // Find the form that contains this select menu
        const form = selectMenu.closest('form');
        const promptIdInput = form?.querySelector('input[name="id"]') as HTMLInputElement;

        selectedOption.addEventListener("click", () => {
            options.classList.toggle("hidden");
        });

        options.addEventListener("click", (event: MouseEvent) => {
            const target = (event.target as HTMLElement).closest(".option") as HTMLElement;
            if (target) {
                const value = target.getAttribute("data-value");
                if (value) {
                    selectMenu.setAttribute("data-value", value);

                    // Get the text content from the first <span> of the target option
                    const firstSpan = target.querySelector('span');
                    selectedOptionSpan.textContent = firstSpan?.textContent?.trim() || value;
                    
                    // Update the hidden input with the selected prompt ID
                    if (promptIdInput) {
                        promptIdInput.value = value;
                    }
                    
                    // Submit the form
                    if (form) {
                        form.submit();
                    }

                    options.classList.add("hidden"); // Hide options
                }
            }
        });

        // Close dropdown if clicked outside
        document.addEventListener("click", (event: MouseEvent) => {
            if (!selectMenu.contains(event.target as Node)) {
                options.classList.add("hidden");
            }
        });
    });
};