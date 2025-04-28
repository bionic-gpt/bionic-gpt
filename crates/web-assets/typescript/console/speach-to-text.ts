/**
 * Main function to attach STT listeners to an element with the id #speech-to-text-button.
 * Clicking the button will start speech recognition, and clicking again will stop it.
 * The recognized text can be displayed or processed as needed.
 */
export const speechToText = (): void => {
    const button = document.querySelector<HTMLButtonElement>('#speech-to-text-button');
    const textArea = document.querySelector<HTMLTextAreaElement>('.pt-3.auto-expand');
  
    const SpeechRecognitionConstructor =
      (window as unknown as WindowWithSpeechRecognition).SpeechRecognition ||
      (window as unknown as WindowWithSpeechRecognition).webkitSpeechRecognition;
  
    if (!SpeechRecognitionConstructor) {
      console.log('SpeechRecognition is not supported in this browser.');
      return;
    } else {
      console.log('SpeechRecognition is supported!');
      if (button) {
        const suffixImage = button.children.item(1) as HTMLElement | null;
        if (suffixImage) {
          suffixImage.style.display = 'none';
        }
        button.classList.remove('hidden');
      }
    }
  
    const recognition = new SpeechRecognitionConstructor();
    recognition.lang = 'en-US';
    recognition.interimResults = true;
    recognition.maxAlternatives = 1;
    recognition.continuous = true;
  
    let textAreaValue = '';
    let finalTranscript = '';
    let isListening = false;
    let silenceTimeout: number | undefined;
  
    if (button) {
      button.addEventListener('click', handleClick);
    }
  
    function handleClick() {
      if (!isListening) {
        startRecognition();
      } else {
        stopRecognition();
      }
    }
  
    function startRecognition() {
      if (textArea) {
        textAreaValue = textArea.value;
      }
      finalTranscript = '';
      if (button) {
        toggleButtonImages('listening');
      }
  
      try {
        recognition.start();
        resetSilenceTimeout();
        isListening = true; // move this here after successful start
      } catch (error) {
        console.error('Error starting speech recognition:', error);
        if (button) {
          revertToIdleState();
        }
      }
    }
  
    function stopRecognition() {
      recognition.stop();
      clearTimeout(silenceTimeout);
      // do not touch isListening here! let the 'end' event handle it
    }
  
    function resetSilenceTimeout() {
      clearTimeout(silenceTimeout);
      silenceTimeout = window.setTimeout(() => {
        stopRecognition();
      }, 3000);
    }
  
    function revertToIdleState() {
      toggleButtonImages('idle');
    }
  
    function toggleButtonImages(state: 'idle' | 'listening') {
      if (!button) return;
  
      const prefixImage = button.children.item(0) as HTMLElement | null;
      const suffixImage = button.children.item(1) as HTMLElement | null;
  
      if (state === 'idle') {
        if (prefixImage) prefixImage.style.display = '';
        if (suffixImage) suffixImage.style.display = 'none';
      } else {
        if (prefixImage) prefixImage.style.display = 'none';
        if (suffixImage) suffixImage.style.display = '';
      }
    }
  
    recognition.addEventListener('speechend', () => {
      clearTimeout(silenceTimeout);
    });
  
    recognition.addEventListener('end', () => {
      isListening = false;
      if (button) {
        revertToIdleState();
      }
    });
  
    recognition.addEventListener('error', (event: Event) => {
      console.error('Speech recognition error:', (event as any).error);
      stopRecognition(); // will automatically call 'end'
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
  
  // --- Custom interfaces for Web Speech API support ---
  
  interface WindowWithSpeechRecognition extends Window {
    SpeechRecognition?: SpeechRecognitionConstructor;
    webkitSpeechRecognition?: SpeechRecognitionConstructor;
  }
  
  interface SpeechRecognitionConstructor {
    new (): SpeechRecognition;
  }
  
  interface SpeechRecognition extends EventTarget {
    lang: string;
    continuous: boolean;
    interimResults: boolean;
    maxAlternatives: number;
    start(): void;
    stop(): void;
    abort(): void;
    addEventListener<K extends keyof SpeechRecognitionEventMap>(
      type: K,
      listener: (this: SpeechRecognition, ev: SpeechRecognitionEventMap[K]) => any,
      options?: boolean | AddEventListenerOptions
    ): void;
  }
  
  interface SpeechRecognitionEventMap {
    result: SpeechRecognitionEvent;
    error: Event;
    end: Event;
    start: Event;
    speechend: Event;
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
  