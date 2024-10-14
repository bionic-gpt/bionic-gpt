const COOKIE_NAME = 'flash_aargh';

export const snackBar = () => {
    const snackbar = document.getElementById('snackbar');
    if (!snackbar) {
        console.warn('Snackbar element not found');
        return;
    }   
    
    const message = getCookie(COOKIE_NAME);
    if (message) {
        console.log(`Cookie found: ${message}`);
        const messageElement = snackbar.querySelector('p');
        if (messageElement) {
            messageElement.textContent = message;
        }

        // Show the snackbar by removing the classes that hide it
        snackbar.classList.remove('translate-y-full', 'opacity-0'); // Show
        snackbar.classList.add('translate-y-0', 'opacity-100'); // Make it fully visible

        // Delete the cookie after reading its value
        deleteCookie(COOKIE_NAME);
        console.log(`Cookie ${COOKIE_NAME} deleted`);

        // Automatically hide the snackbar after 4 seconds
        setTimeout(() => {
            // Slide up and hide the snackbar
            snackbar.classList.remove('translate-y-0', 'opacity-100'); // Hide
            snackbar.classList.add('translate-y-full', 'opacity-0'); // Slide up
        }, 4000);
    } else {
        console.warn(`Cookie ${COOKIE_NAME} not found or is empty`);
    }
};


function getCookie(name: string): string | null {
    const nameLenPlus = name.length + 1;
    return document.cookie
        .split(';')
        .map(c => c.trim())
        .filter(cookie => {
            return cookie.substring(0, nameLenPlus) === `${name}=`;
        })
        .map(cookie => {
            return decodeURIComponent(cookie.substring(nameLenPlus));
        })[0] || null;
}

function deleteCookie(name: string) {
    console.log('Cookies before deletion:', document.cookie);
    document.cookie = `${name}=; Max-Age=0; path=/;`;
    console.log('Cookies after deletion:', document.cookie);
}