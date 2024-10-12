export const selectMenu = () => {
    const selectMenus = document.querySelectorAll('.select-menu'); // Select all elements with class 'select-menu'

    selectMenus.forEach(selectMenu => {
        const selectedOption = selectMenu.querySelector('.selected-option') as HTMLElement;
        const options = selectMenu.querySelector('.options') as HTMLElement;

        selectedOption.addEventListener("click", () => {
            options.classList.toggle("hidden");
        });

        options.addEventListener("click", (event: MouseEvent) => {
            const target = (event.target as HTMLElement).closest(".option") as HTMLElement;
            if (target) {
                const value = target.getAttribute("data-value");
                if(value) {
                    selectMenu.setAttribute("data-value", value)
                }

                // Get the first element inside the target option
                const firstElement = target.querySelector('span') as HTMLElement; // Assuming the first element is a <span>
                if (firstElement) {
                    selectedOption.innerHTML = firstElement.innerHTML; // Update the displayed value to the first element's content
                }
                
                options.classList.add("hidden"); // Hide options
                console.log("Selected value:", value); // Log the value if needed
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