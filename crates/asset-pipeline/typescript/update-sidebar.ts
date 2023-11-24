// When using turbo frames side nav does not get updated.
document.addEventListener('turbo:load', () => {
    // Persist a form to local storage.
    document.querySelectorAll('a[data-turbo-frame="main-content"]').forEach((link) => {
        link.addEventListener("click", function() {

            // Remove all active
            document.querySelectorAll('a[data-turbo-frame="main-content"]').forEach((link) => {
                link.classList.remove('active')
            })

            // Set this one
            link.classList.add('active')
        }, {once : true});
    })
})
