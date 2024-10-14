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
                // Create or find the span for text content
                let textSpan = selectedOption.querySelector('span') as HTMLElement;
                if (!textSpan) {
                    textSpan = document.createElement('span'); // Create a new span if it doesn't exist
                    selectedOption.appendChild(textSpan); // Append it to selectedOption
                }
                textSpan.textContent = storedValue; // Set the displayed text
                selectMenu.setAttribute("data-value", storedValue); // Set data-value attribute
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
                    
                    // Get the first element inside the target option
                    const firstElement = target.querySelector('span') as HTMLElement; // Assuming the first element is a <span>
                    if (firstElement) {
                        // Create or find the span for text content
                        let textSpan = selectedOption.querySelector('span') as HTMLElement;
                        if (!textSpan) {
                            textSpan = document.createElement('span'); // Create a new span if it doesn't exist
                            selectedOption.appendChild(textSpan); // Append it to selectedOption
                        }
                        // Update the displayed text
                        textSpan.textContent = firstElement.textContent || ''; // Update the displayed text

                        // Store only the selected text in localStorage if the element has an ID
                        if (menuId) {
                            localStorage.setItem(menuId, firstElement.textContent || ''); // Store the selected text
                        }
                    }

                    options.classList.add("hidden"); // Hide options
                    console.log("Selected value:", value); // Log the value if needed
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
