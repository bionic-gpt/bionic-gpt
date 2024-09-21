export const readAloud = () => {
    const icons = document.querySelectorAll('.read-aloud')

    icons.forEach((icon) => {

        icon.addEventListener('click', () => {

            const parent = icon.parentElement
            const previousElement = parent?.parentElement?.previousElementSibling;
    
            if (previousElement && icon instanceof HTMLImageElement) {
                const previousElementContent = previousElement.innerHTML;
                readAloudContent(previousElementContent);
            }
        })
    })
}

function readAloudContent(text: string): void {
    alert(text)
}