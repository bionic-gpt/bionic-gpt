// A select box which shows and hides divs
document.addEventListener('readystatechange', () => {
    if (document.readyState == 'complete') {
        document.querySelectorAll('select.select-div').forEach((select) => {

            if (select instanceof HTMLSelectElement) {

                hideAll(select)
                showSelected(select)

                // Add a click handler
                select.addEventListener("change", () => {
                    hideAll(select)
                    showSelected(select)
                })
            }

            function hideAll(select: HTMLSelectElement) {
                for (var i = 0; i < select.length; i++) {
                    const id = select.options[i].value
                    const div = document.getElementById(id)
                    if (div instanceof HTMLDivElement) {
                        div.style.display = "none";
                    }
                }
            }

            function showSelected(select: HTMLSelectElement) {
                const id = select.options[select.selectedIndex].value
                const div = document.getElementById(id)
                if (div instanceof HTMLDivElement) {
                    div.style.display = "block";
                }
            }
        })
    }
})