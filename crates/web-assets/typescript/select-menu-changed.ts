export const modelChanged = () => {
    // Select the target element you want to observe
    const targetElement = document.getElementById('model-selector'); // Replace with your specific selector

    // Create a callback function to execute when mutations are observed
    const callback = (mutationsList) => {
        for (const mutation of mutationsList) {
            // Check if the 'data-value' attribute was modified
            if (mutation.type === 'attributes' && mutation.attributeName === 'data-value') {
                let val = targetElement?.getAttribute('data-value')
                console.log('data-value attribute changed:', val);
                
                let hiddenField = document.getElementById('prompt-form-prompt-id')
                if(hiddenField instanceof HTMLInputElement && val) {
                    console.log('Updating prompt form')
                    hiddenField.value = val
                }
            }
        }
    };

    // Create an observer instance linked to the callback function
    const observer = new MutationObserver(callback);

    // Options for the observer (which mutations to observe)
    const config = {
        attributes: true, // Observe changes to attributes
        attributeFilter: ['data-value'], // Only observe changes to the 'data-value' attribute
    };

    // Start observing the target element for configured mutations
    if (targetElement) {
        observer.observe(targetElement, config);
    }
}
