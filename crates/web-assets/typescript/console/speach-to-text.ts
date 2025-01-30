/**
 * Main function to attach STT listeners to an element with the id #speech-to-text-button.
 * Clicking the button will start speech recognition, and clicking again will stop it.
 * The recognized text can be displayed or processed as needed.
 */
export const speechToText = (): void => {
    const button = document.querySelector<HTMLButtonElement>('#speech-to-text-button');
    const textArea = document.querySelector<HTMLTextAreaElement>('.pt-3.auto-expand');
    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;

    const recordingSVG = `
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16px" height="16px" viewBox="0 0 256 256" xml:space="preserve">
        <defs>
        </defs>
        <g style="stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: none; fill-rule: nonzero; opacity: 1;" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)" >
            <path d="M 45 0 c -8.481 0 -15.382 6.9 -15.382 15.382 v 29.044 c 0 8.482 6.9 15.382 15.382 15.382 s 15.382 -6.9 15.382 -15.382 V 15.382 C 60.382 6.9 53.481 0 45 0 z" style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: rgb(211,55,55); fill-rule: nonzero; opacity: 1;" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" />
            <path d="M 69.245 38.312 c -1.104 0 -2 0.896 -2 2 v 6.505 c 0 12.266 -9.979 22.244 -22.245 22.244 s -22.245 -9.979 -22.245 -22.244 v -6.505 c 0 -1.104 -0.896 -2 -2 -2 s -2 0.896 -2 2 v 6.505 c 0 13.797 10.705 25.134 24.245 26.16 V 86 h -9.126 c -1.104 0 -2 0.896 -2 2 s 0.896 2 2 2 h 22.252 c 1.104 0 2 -0.896 2 -2 s -0.896 -2 -2 -2 H 47 V 72.978 c 13.54 -1.026 24.245 -12.363 24.245 -26.16 v -6.505 C 71.245 39.208 70.35 38.312 69.245 38.312 z" style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: rgb(211,55,55); fill-rule: nonzero; opacity: 1;" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" />
        </g>
        </svg>
    `
    const startSVG = `
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16px" height="16px" viewBox="0 0 256 256" xml:space="preserve">
        <defs>
        </defs>
        <g style="stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: none; fill-rule: nonzero; opacity: 1;" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)" >
            <path d="M 45 0 c -8.481 0 -15.382 6.9 -15.382 15.382 v 29.044 c 0 8.482 6.9 15.382 15.382 15.382 s 15.382 -6.9 15.382 -15.382 V 15.382 C 60.382 6.9 53.481 0 45 0 z" style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: rgb(62,150,255); fill-rule: nonzero; opacity: 1;" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" />
            <path d="M 69.245 38.312 c -1.104 0 -2 0.896 -2 2 v 6.505 c 0 12.266 -9.979 22.244 -22.245 22.244 s -22.245 -9.979 -22.245 -22.244 v -6.505 c 0 -1.104 -0.896 -2 -2 -2 s -2 0.896 -2 2 v 6.505 c 0 13.797 10.705 25.134 24.245 26.16 V 86 h -9.126 c -1.104 0 -2 0.896 -2 2 s 0.896 2 2 2 h 22.252 c 1.104 0 2 -0.896 2 -2 s -0.896 -2 -2 -2 H 47 V 72.978 c 13.54 -1.026 24.245 -12.363 24.245 -26.16 v -6.505 C 71.245 39.208 70.35 38.312 69.245 38.312 z" style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: rgb(62,150,255); fill-rule: nonzero; opacity: 1;" transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" />
        </g>
        </svg>
    `       

    if (!SpeechRecognition) {
        console.error('SpeechRecognition is not supported in this browser.');
        return;
    } else {
        console.log('SpeechRecognition is supported!');
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
            console.log('Speech recognition button clicked');
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
            button.innerHTML = recordingSVG;   
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
            console.log('No speech detected for 3 seconds, stopping recognition.');
            stopRecognition();
        }, 3000);
    }

    function revertToIdleState(button: HTMLButtonElement) {
        button.innerHTML = startSVG;
    }

    recognition.addEventListener('speechend', () => {
        console.log('Speech recognition has ended.');
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

        console.log('Final transcript:', finalTranscript);
        console.log('Interim transcript:', interimTranscript);

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