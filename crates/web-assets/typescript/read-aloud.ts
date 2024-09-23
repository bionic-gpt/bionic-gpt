export const readAloud = () => {
    const icons = document.querySelectorAll('.read-aloud')

    icons.forEach((icon) => {

        icon.addEventListener('click', () => {

            const parent = icon.parentElement
            const previousElement = parent?.parentElement?.previousElementSibling;

            if (previousElement && icon instanceof HTMLImageElement) {
                const previousElementContent = previousElement.innerHTML;
                readAloudContent(previousElementContent);
            }
        })
    })
}

function readAloudContent(text: string): void {

    fetch('/app/synthesize', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            "Accept":  "audio/mpeg"
        },
        body: JSON.stringify({
            model: "tts-1",
            voice: "alloy",
            input: text
        }),
    })
        .then(response => response.arrayBuffer())
        .then(buffer => {
            const blob = new Blob([buffer], { type: 'audio/mp3' });
            const url = URL.createObjectURL(blob);
            const audio = new Audio(url);
            audio.play();
        })
        .catch(error => alert(error));
}