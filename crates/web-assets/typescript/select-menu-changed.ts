export const modelChanged = () => {
    // Select the target element you want to observe
    const targetElement = document.getElementById('model-selector'); // Replace with your specific selector

    // Helper function to get the 'data-value' from the first child with the 'selected-option' class
    const getSelectedOptionValue = () => {
        return targetElement?.getAttribute('data-value')
    };

    // Set the ID on page load if 'data-value' is present
    const initialVal = getSelectedOptionValue();
    const hiddenField = document.getElementById('prompt-form-prompt-id');
    if (hiddenField instanceof HTMLInputElement && initialVal) {
        console.log('Setting prompt form value on page load');
        hiddenField.value = initialVal;
    }

    // Create a callback function to execute when mutations are observed
    const callback = (mutationsList: MutationRecord[]) => {
        for (const mutation of mutationsList) {
            // Check if the 'data-value' attribute was modified
            if (mutation.type === 'attributes' && mutation.attributeName === 'data-value') {
                const val = getSelectedOptionValue();
                console.log('data-value attribute changed:', val);

                if (hiddenField instanceof HTMLInputElement && val) {
                    console.log('Updating prompt form');
                    hiddenField.value = val;
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
        subtree: true, // Include child elements in the observation
    };

    // Start observing the target element for configured mutations
    if (targetElement) {
        observer.observe(targetElement, config);
    }
};
