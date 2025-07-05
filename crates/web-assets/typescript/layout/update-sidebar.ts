// When using turbo frames side nav does not get updated.
export const updateSidebar = () => {
    const links = document.querySelectorAll<HTMLAnchorElement>('a[data-turbo-frame="main-content"]');

    // When a menu item is clicked make it active
    links.forEach((link) => {
        link.addEventListener('click', function () {
            links.forEach((l) => l.classList.remove('menu-active'));
            link.classList.add('menu-active');
        }, { once: true });
    });

    // Highlight the menu item that matches the current path
    links.forEach((link) => {
        const url = new URL(link.href, window.location.origin);
        if (url.pathname === window.location.pathname) {
            links.forEach((l) => l.classList.remove('menu-active'));
            link.classList.add('menu-active');
        }
    });
};
