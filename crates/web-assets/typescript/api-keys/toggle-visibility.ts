export const toggleVisibility = () => {
    const toggleButtons = document.querySelectorAll('.api-keys-toggle-visibility');
    console.log('Attaching handler')

    toggleButtons.forEach(button => {
        button.addEventListener('click', () => {
            console.log('Got Click')
            if (button instanceof HTMLButtonElement && button.parentElement) {
                console.log('Toggle Key Visibility')
                // Find the sibling input within the same flex container
                const input = button.parentElement.querySelector('input') as HTMLInputElement;

                if (input.type === 'password') {
                    console.log('Show')
                    input.type = 'text';
                    button.textContent = 'Hide';
                } else {
                    console.log('Hide')
                    input.type = 'password';
                    button.textContent = 'Show';
                }
            }
        });
    });
}