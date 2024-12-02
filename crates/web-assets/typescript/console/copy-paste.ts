export const copyPaste = () => {
    const codeBlocks = document.querySelectorAll('pre code');

    codeBlocks.forEach((codeBlock) => {
        const copyButton = document.createElement('button');

        const svgContent = `
        <svg fill="white" height="16px" width="16px" version="1.1"
            xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
            viewBox="0 0 352.804 352.804">
            <g>
            <path d="M318.54,57.282h-47.652V15c0-8.284-6.716-15-15-15H34.264c-8.284,0-15,6.716-15,15v265.522c0,8.284,6.716,15,15,15h47.651
                v42.281c0,8.284,6.716,15,15,15H318.54c8.284,0,15-6.716,15-15V72.282C333.54,63.998,326.824,57.282,318.54,57.282z
                M49.264,265.522V30h191.623v27.282H96.916c-8.284,0-15,6.716-15,15v193.24H49.264z M303.54,322.804H111.916V87.282H303.54V322.804
                z"/>
            </g>
        </svg>
        `;

        //copyButton.innerHTML = svgContent
        copyButton.textContent = 'Copy';

        const pre = codeBlock.parentElement;

        if(pre) {
            pre.prepend(copyButton);
            pre.classList.add('relative')
            copyButton.classList.add('absolute')
            copyButton.classList.add('border')
            copyButton.classList.add('p-1')
            copyButton.classList.add('top-2')
            copyButton.classList.add('right-2')
        }

        copyButton.addEventListener('click', () => {
            if(codeBlock.textContent) {
                copyToClipboard(codeBlock.textContent);
                copyButton.textContent = 'Copied'
            }
        });
    });
};

function copyToClipboard(text: string): void {
    const textarea = document.createElement('textarea');
    textarea.value = text;
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand('copy');
    document.body.removeChild(textarea);
}
