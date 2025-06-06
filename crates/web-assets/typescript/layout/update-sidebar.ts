// When using turbo frames side nav does not get updated.
export const updateSidebar = () => {
    // Persist a form to local storage.
    document.querySelectorAll('a[data-turbo-frame="main-content"]').forEach((link) => {
        link.addEventListener("click", function() {

            // Remove all active
            document.querySelectorAll('a[data-turbo-frame="main-content"]').forEach((link) => {
                link.classList.remove('menu-active')
            })

            // Set this one
            link.classList.add('menu-active')
        }, {once : true});
    })
}
