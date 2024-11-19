export const copy = () => {
    const copyIcons = document.querySelectorAll('.copy-response')

    copyIcons.forEach((copyIcon) => {

        copyIcon.addEventListener('click', () => {

            const parent = copyIcon.parentElement
            const previousElement = parent?.parentElement?.previousElementSibling;

            const clickedImage = copyIcon.getAttribute("clicked-img")
    
            if (previousElement && clickedImage && copyIcon instanceof HTMLImageElement) {
                const previousElementContent = previousElement.innerHTML;
                copyToClipboard(previousElementContent);
                const originalImage = copyIcon.src
                copyIcon.src = clickedImage

                setTimeout(() => {
                    copyIcon.src = originalImage
                }, 3000);
            }
        })
    })
}

function copyToClipboard(text: string): void {
    const textarea = document.createElement('textarea');
    textarea.value = text;
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand('copy');
    document.body.removeChild(textarea);
}