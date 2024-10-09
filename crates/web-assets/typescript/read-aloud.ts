export const readAloud = () => {
    const icons = document.querySelectorAll('.read-aloud');

    icons.forEach((icon) => {
        icon.addEventListener('click', () => {
            const parent = icon.parentElement;
            const previousElement = parent?.parentElement?.previousElementSibling;

            if (previousElement && icon instanceof HTMLImageElement) {
                const previousElementContent = previousElement.innerHTML;

                // Change the icon to the loading spinner using the 'loading-img' data attribute
                const loadingImg = icon.getAttribute('data-loading-img');
                if (loadingImg) {
                    icon.src = loadingImg;
                }

                // Start reading aloud the content
                readAloudContent(previousElementContent, icon);
            }
        });
    });
};

function readAloudContent(text: string, icon: HTMLImageElement): void {
    fetch('/app/synthesize', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'audio/mpeg',
        },
        body: JSON.stringify({
            model: 'tts-1',
            voice: 'alloy',
            input: text,
        }),
    })
        .then(response => response.arrayBuffer())
        .then(buffer => {
            const blob = new Blob([buffer], { type: 'audio/mp3' });
            const url = URL.createObjectURL(blob);
            const audio = new Audio(url);

            // Play the audio
            audio.play();

            // Change the icon to the stop image using the 'stop-image' data attribute
            const stopImg = icon.getAttribute('data-stop-img');
            if (stopImg) {
                icon.src = stopImg;
            }

            // Add an event listener to stop the audio when the stop icon is clicked
            icon.addEventListener('click', () => {
                if (icon.src === stopImg) {
                    audio.pause();
                    audio.currentTime = 0; // Reset the audio to the beginning

                    // Change the icon back to the play image using the 'play-image' data attribute
                    const playImg = icon.getAttribute('data-play-img');
                    if (playImg) {
                        icon.src = playImg;
                    }
                }
            }, { once: true });

            // Cleanup the URL object and reset the icon when audio ends
            audio.addEventListener('ended', () => {
                URL.revokeObjectURL(url);
                const playImg = icon.getAttribute('data-play-img');
                if (playImg) {
                    icon.src = playImg;
                }
            });
        })
        .catch(error => {
            alert(error);

            // Revert to the play image in case of an error
            const playImg = icon.getAttribute('data-play-img');
            if (playImg) {
                icon.src = playImg;
            }
        });
}
