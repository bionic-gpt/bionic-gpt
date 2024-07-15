export const copy = () => {
    const copyIcons = document.querySelectorAll('.copy-response')

    copyIcons.forEach((copyIcon) => {

        copyIcon.addEventListener('click', () => {
            alert('here')
        });
    })
}