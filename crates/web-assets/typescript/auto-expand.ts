export const autoExpand = () => {
    const autoExpandElements = document.querySelectorAll("textarea.auto-expand") as NodeListOf<HTMLTextAreaElement>;
    autoExpandElements.forEach(element => {
            element.addEventListener("input", function () {
            this.style.height = "auto"; // Reset height
            this.style.height = `${this.scrollHeight}px`; // Set to scrollHeight
            if (this.scrollHeight > parseInt(getComputedStyle(this).maxHeight)) {
                this.style.overflowY = 'auto'; // Add scrollbar if height exceeds max-height
            } else {
                this.style.overflowY = 'hidden'; // Hide scrollbar
            }
        });
    });
};
