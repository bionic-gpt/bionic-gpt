/**
 * Main function to attach STT listeners to an element with the id #speech-to-text-button.
 * Clicking the button will start speech recognition, and clicking again will stop it.
 * The recognized text can be displayed or processed as needed.
 */
export const speechToText = (): void => {
    const button = document.querySelector<HTMLButtonElement>('#speech-to-text-button');
    const textArea = document.querySelector<HTMLTextAreaElement>('.pt-3.auto-expand');
    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
  
    const startSVG = `
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="32" height="32" viewBox="0 0 256 256" xml:space="preserve">
        <defs>
        </defs>
        <g style="stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: none; fill-rule: nonzero; opacity: 1;" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)" >
            <path d="M 45 0 C 20.147 0 0 20.147 0 45 c 0 24.853 20.147 45 45 45 s 45 -20.147 45 -45 C 90 20.147 69.853 0 45 0 z M 38 24.844 c 0 -3.866 3.134 -7 7 -7 s 7 3.134 7 7 v 18.792 c 0 3.866 -3.134 7 -7 7 s -7 -3.134 -7 -7 V 24.844 z M 62.741 44.238 c 0 8.698 -6.297 15.939 -14.568 17.44 v 4.133 h 3.512 c 1.753 0 3.173 1.42 3.173 3.173 s -1.42 3.173 -3.173 3.173 H 38.314 c -1.752 0 -3.173 -1.42 -3.173 -3.173 s 1.421 -3.173 3.173 -3.173 h 3.513 v -4.133 c -8.271 -1.502 -14.568 -8.743 -14.568 -17.44 v -3.909 c 0 -1.752 1.421 -3.173 3.173 -3.173 c 1.752 0 3.173 1.421 3.173 3.173 v 3.909 c 0 6.283 5.112 11.395 11.395 11.395 s 11.395 -5.112 11.395 -11.395 v -3.909 c 0 -1.752 1.42 -3.173 3.173 -3.173 c 1.753 0 3.173 1.421 3.173 3.173 V 44.238 z" style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: rgb(0,0,0); fill-rule: nonzero; opacity: 1;" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" />
        </g>
        </svg>
    `
    const stopSVG = `
        <?xml version="1.0" encoding="UTF-8"?>
        <svg width="32" height="32" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
        <!-- Outer black circle -->
        <circle cx="50" cy="50" r="48" fill="black" stroke="none" />
        <!-- Inner red square -->
        <rect x="30" y="30" width="40" height="40" fill="red" stroke="none" />
        </svg>
    `

    if (!SpeechRecognition) {
        console.error('SpeechRecognition is not supported in this browser.');
        
        return;
    } else {
        console.log('SpeechRecognition is supported!');
        if (button) {
            button.classList.remove('hidden'); // Ensure the button is visible if supported
        }
    }

    const recognition = new SpeechRecognition();
    recognition.lang = 'en-US';
    recognition.interimResults = true; // Enable interim results
    recognition.maxAlternatives = 1;
    recognition.continuous = true;
    let textAreaValue = '';
    let finalTranscript = '';
    let isListening = false;
    let silenceTimeout: number | undefined;

    if (button) {
        const handleClick = () => {
            if (!isListening) {
                startRecognition();
            } else {
                stopRecognition();
            }
        };

        button.addEventListener('click', handleClick);
    }

    function startRecognition() {
        if (textArea) {
            textAreaValue = textArea.value;
        }
        finalTranscript = ''; // Reset finalTranscript
        isListening = true;
        if (button) {
            button.innerHTML = stopSVG;   
        }
        try {
            recognition.start();
            resetSilenceTimeout();
            recognition.addEventListener('result', resetSilenceTimeout);
        } catch (error) {
            console.error('Error starting speech recognition:', error);
            if (button) {
                revertToIdleState(button);
            }
        }
    }

    function stopRecognition() {
        isListening = false;
        recognition.stop();
        clearTimeout(silenceTimeout);
        recognition.removeEventListener('result', resetSilenceTimeout);
        if (button) {
            revertToIdleState(button);
        }
    }

    function resetSilenceTimeout() {
        clearTimeout(silenceTimeout);
        silenceTimeout = window.setTimeout(() => {
            stopRecognition();
        }, 3000);
    }

    function revertToIdleState(button: HTMLButtonElement) {
        button.innerHTML = startSVG;
    }

    recognition.addEventListener('speechend', () => {
        // recognition.stop();
        clearTimeout(silenceTimeout);
    });

    recognition.addEventListener('error', (event: SpeechRecognitionErrorEvent) => {
        console.error('Speech recognition error:', event.error);
        recognition.stop();
        clearTimeout(silenceTimeout);
    });
    
    recognition.addEventListener('result', (event: SpeechRecognitionEvent) => {
        let interimTranscript = '';

        for (let i = event.resultIndex; i < event.results.length; ++i) {
            const transcript = event.results[i][0].transcript;
            if (event.results[i].isFinal) {
                finalTranscript += transcript;

            } else {
                interimTranscript += transcript;
            }
        }

        if (textArea) {
            textArea.value = textAreaValue + finalTranscript + interimTranscript;
        }
        resetSilenceTimeout();
    });
};

// Define the types for the Web Speech API
interface SpeechRecognition extends EventTarget {
    continuous: boolean;
    interimResults: boolean;
    start(): void;
    stop(): void;
    abort(): void;
    addEventListener<K extends keyof SpeechRecognitionEventMap>(
        type: K,
        listener: (this: SpeechRecognition, ev: SpeechRecognitionEventMap[K]) => any,
        options?: boolean | AddEventListenerOptions
    ): void;
}

interface SpeechRecognitionEvent extends Event {
    readonly resultIndex: number;
    readonly results: SpeechRecognitionResultList;
}

interface SpeechRecognitionResultList {
    readonly length: number;
    item(index: number): SpeechRecognitionResult;
    [index: number]: SpeechRecognitionResult;
}

interface SpeechRecognitionResult {
    readonly length: number;
    readonly isFinal: boolean;
    item(index: number): SpeechRecognitionAlternative;
    [index: number]: SpeechRecognitionAlternative;
}

interface SpeechRecognitionAlternative {
    readonly transcript: string;
    readonly confidence: number;
}

interface SpeechRecognitionEventMap {
    result: SpeechRecognitionEvent;
    error: Event;
    end: Event;
    start: Event;
}