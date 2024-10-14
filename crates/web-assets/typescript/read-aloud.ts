export const readAloud = (): void => {
    const icons = document.querySelectorAll<HTMLImageElement>('.read-aloud');

    icons.forEach((icon) => {
        const handleClick = () => {
            const parent = icon.parentElement;
            const previousElement = parent?.parentElement?.previousElementSibling;

            if (previousElement) {
                const previousElementContent = previousElement.innerHTML;

                // Change the icon to the loading spinner using the 'loading-img' data attribute
                const loadingImg = icon.getAttribute('data-loading-img');
                if (loadingImg) {
                    icon.src = loadingImg;
                }

                // Start reading aloud the content and remove the handleClick listener
                readAloudContent(previousElementContent, icon, handleClick);
            }
        };

        // Add the click listener initially
        icon.addEventListener('click', handleClick);
    });
};

function readAloudContent(
    text: string,
    icon: HTMLImageElement,
    handleClick: () => void
): void {
    let isPlaying = false;
    let audio: HTMLAudioElement | null = null;

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
            audio = new Audio(url);

            // Play the audio and update the state
            audio.play().then(() => {
                isPlaying = true;

                // Remove the handleClick listener to prevent restarting during playback
                icon.removeEventListener('click', handleClick);

                // Change the icon to the stop image using the 'stop-image' data attribute
                const stopImg = icon.getAttribute('data-stop-img');
                if (stopImg) {
                    icon.src = stopImg;
                }

                // Add an event listener to stop the audio when the stop icon is clicked
                const stopListener = () => {
                    if (isPlaying && audio) {
                        audio.pause();
                        audio.currentTime = 0; // Reset the audio to the beginning
                        isPlaying = false;

                        // Change the icon back to the play image using the 'play-image' data attribute
                        const playImg = icon.getAttribute('data-play-img');
                        if (playImg) {
                            icon.src = playImg;
                        }

                        // Remove the stop listener after stopping
                        icon.removeEventListener('click', stopListener);

                        // Re-add the handleClick listener to allow replaying
                        icon.addEventListener('click', handleClick);
                    }
                };

                icon.addEventListener('click', stopListener);

                // Cleanup when the audio ends
                audio?.addEventListener('ended', () => {
                    URL.revokeObjectURL(url);
                    isPlaying = false;

                    // Change the icon back to the play image using the 'play-image' data attribute
                    const playImg = icon.getAttribute('data-play-img');
                    if (playImg) {
                        icon.src = playImg;
                    }

                    // Remove the stop listener when audio ends
                    icon.removeEventListener('click', stopListener);

                    // Re-add the handleClick listener to allow replaying
                    icon.addEventListener('click', handleClick);
                });
            });
        })
        .catch(error => {
            console.error('Audio playback error:', error);
            alert('An error occurred while trying to play the audio.');

            // Revert to the play image in case of an error
            const playImg = icon.getAttribute('data-play-img');
            if (playImg) {
                icon.src = playImg;
            }

            // Re-add the handleClick listener to allow replaying in case of an error
            icon.addEventListener('click', handleClick);
        });
}
