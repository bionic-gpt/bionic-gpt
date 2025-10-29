export const clickableCard = () => {
    const clickableElements = document.querySelectorAll<HTMLElement>('[data-clickable-link]');

    clickableElements.forEach(el => {
        el.addEventListener('click', (e: MouseEvent) => {
            const target = e.target as HTMLElement;

            // Ignore clicks on buttons or links inside the element
            if (
                target.closest('a') ||
                target.closest('button') ||
                target.tagName === 'A' ||
                target.tagName === 'BUTTON'
            ) return;

            const url = el.getAttribute('data-clickable-link');
            if (url) {
                window.location.href = url;
            }
        });
    });
};
