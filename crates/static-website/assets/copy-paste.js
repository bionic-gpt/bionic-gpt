if (document.readyState !== 'loading') {
    initCopyPaste();
} else {
    document.addEventListener("DOMContentLoaded", function () {
        initCopyPaste();
    });
}

function initCopyPaste() {
    const copyButtonLabel = "Copy Code";
    const svgIcon = `
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="white" viewBox="0 0 16 16">
            <path d="M10 1.5A1.5 1.5 0 0 1 11.5 3v1h-1V3a.5.5 0 0 0-.5-.5H4A1.5 1.5 0 0 0 2.5 4v8A1.5 1.5 0 0 0 4 13.5h6a.5.5 0 0 0 .5-.5v-1h1v1a1.5 1.5 0 0 1-1.5 1.5H4A2.5 2.5 0 0 1 1.5 12V4A2.5 2.5 0 0 1 4 1.5h6z"/>
            <path d="M13.5 4a.5.5 0 0 1 .5.5v8a.5.5 0 0 1-.5.5h-6a.5.5 0 0 1-.5-.5v-8a.5.5 0 0 1 .5-.5h6zm-.5 1h-5v7h5V5z"/>
        </svg>
    `;

    let blocks = document.querySelectorAll("pre");

    blocks.forEach((block) => {
        if (navigator.clipboard) {
            let button = document.createElement("button");
            button.innerHTML = svgIcon;
            button.setAttribute("aria-label", copyButtonLabel);

            button.style.position = "absolute";
            button.style.top = "5px";
            button.style.right = "5px";
            button.style.background = "black";
            button.style.border = "none";
            button.style.cursor = "pointer";
            button.style.padding = "4px";

            let wrapper = document.createElement("div");
            wrapper.style.position = "relative";

            block.parentNode.insertBefore(wrapper, block);
            wrapper.appendChild(block);
            wrapper.appendChild(button);

            button.addEventListener("click", async () => {
                await copyCode(block, button);
            });
        }
    });

    async function copyCode(block, button) {
        let text = block.innerText;

        await navigator.clipboard.writeText(text);

        button.innerHTML = "✔️";

        setTimeout(() => {
            button.innerHTML = svgIcon;
        }, 700);
    }
}