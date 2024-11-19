const themeStorageName = 'ui-theme'

export const themeSwitcher = () => {
    // We can create a trigger to open drawers
    document.querySelectorAll('a.theme').forEach(async (link) => {
        // Detect when a user clicks a row
        link.addEventListener('click', (event) => {

            var theme = link.getAttribute('href')
            if (theme) {
                theme = theme.substring(1)

                localStorage.setItem(themeStorageName, theme)
                setTheme()
            }
            if (link instanceof HTMLAnchorElement) {
                link.blur()
            }
            event.stopImmediatePropagation()
        })
    })
}

export const setTheme = () => {
    const theme = localStorage.getItem(themeStorageName) || "system"

    if (theme) {

        if (theme === 'system') {
            const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            document.documentElement.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
        } else {
            document.documentElement.setAttribute('data-theme', theme);
        }

    }
}