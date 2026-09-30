if (document.readyState !== 'loading') {
    initCopyPaste();
} else {
    document.addEventListener("DOMContentLoaded", function () {
        initCopyPaste();
    });
}

function initCopyPaste() {
    const copyButtonLabel = "Copy Code";
    const copyIcon = `
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
        </svg>
    `;
    const copiedIcon = `
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M20 6 9 17l-5-5"></path>
        </svg>
    `;

    let blocks = document.querySelectorAll("article pre, .prose pre");

    blocks.forEach((block) => {
        if (block.dataset.copyReady === "true") {
            return;
        }

        if (navigator.clipboard) {
            let button = document.createElement("button");
            button.type = "button";
            button.className = "code-copy-btn";
            button.dataset.copyCodeButton = "true";
            button.innerHTML = copyIcon;
            button.setAttribute("aria-label", copyButtonLabel);

            button.style.position = "absolute";
            button.style.top = "8px";
            button.style.right = "8px";
            button.style.display = "inline-flex";
            button.style.alignItems = "center";
            button.style.justifyContent = "center";
            button.style.width = "28px";
            button.style.height = "28px";
            button.style.color = "white";
            button.style.background = "rgba(15, 23, 42, 0.82)";
            button.style.border = "1px solid rgba(255, 255, 255, 0.16)";
            button.style.borderRadius = "7px";
            button.style.cursor = "pointer";
            button.style.padding = "0";
            button.style.backdropFilter = "blur(6px)";

            let wrapper = document.createElement("div");
            wrapper.style.position = "relative";

            block.parentNode.insertBefore(wrapper, block);
            wrapper.appendChild(block);
            wrapper.appendChild(button);
            block.dataset.copyReady = "true";

            button.addEventListener("click", async () => {
                await copyCode(block, button);
            });
        }
    });

    async function copyCode(block, button) {
        let text = block.innerText;

        await navigator.clipboard.writeText(text);

        button.innerHTML = copiedIcon;

        setTimeout(() => {
            button.innerHTML = copyIcon;
        }, 700);
    }
}
