export const selectMenu = () => {
    const selectMenus = document.querySelectorAll('.select-menu'); // Select all elements with class 'select-menu'

    selectMenus.forEach(selectMenu => {
        const selectedOption = selectMenu.querySelector('.selected-option') as HTMLElement;
        const options = selectMenu.querySelector('.options') as HTMLElement;

        // Load stored value from localStorage if the element has an ID
        const menuId = selectMenu.id;
        if (menuId) {
            const storedValue = localStorage.getItem(menuId);
            if (storedValue) {
                // Find the corresponding option with the stored value
                const correspondingOption = options.querySelector(`[data-value="${storedValue}"]`) as HTMLElement;

                // Use the text content of the corresponding option if it exists
                if (correspondingOption) {
                    let textSpan = selectedOption.querySelector('span') as HTMLElement;
                    if (!textSpan) {
                        textSpan = document.createElement('span'); // Create a new span if it doesn't exist
                        selectedOption.appendChild(textSpan); // Append it to selectedOption
                    }// Use the text content of the first <span> inside the corresponding option if it exists
                    const firstSpan = correspondingOption.querySelector('span');
                    textSpan.textContent = firstSpan?.textContent?.trim() || storedValue;

                    selectMenu.setAttribute("data-value", storedValue); // Set data-value attribute
                }
            }
        }

        selectedOption.addEventListener("click", () => {
            options.classList.toggle("hidden");
        });

        options.addEventListener("click", (event: MouseEvent) => {
            const target = (event.target as HTMLElement).closest(".option") as HTMLElement;
            if (target) {
                const value = target.getAttribute("data-value");
                if (value) {
                    selectMenu.setAttribute("data-value", value);

                    // Get the text content from the target option
                    const optionText = target.textContent?.trim() || value;

                    // Create or find the span for text content
                    let textSpan = selectedOption.querySelector('span') as HTMLElement;
                    if (!textSpan) {
                        textSpan = document.createElement('span'); // Create a new span if it doesn't exist
                        selectedOption.appendChild(textSpan); // Append it to selectedOption
                    }

                    // Update the displayed text
                    textSpan.textContent = optionText;

                    // Store the selected value in localStorage if the element has an ID
                    if (menuId) {
                        localStorage.setItem(menuId, value); // Store the selected value
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
