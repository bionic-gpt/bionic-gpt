document.addEventListener('turbo:load', () => {
    // Persist a form to local storage.
    document.querySelectorAll('turbo-frame').forEach((frame) => {
        const id = frame.getAttribute('id')

        if(id && id.startsWith("status-")) {
            let intId = setInterval(() => {
                frame.removeAttribute("complete")
                if(frame.innerHTML.indexOf("Processed") != -1) {
                    clearInterval(intId)
                }
            }, 1000);
        }
    })
})
