export const modelChanged = () => {
    // Select the target element you want to observe
    const targetElement = document.getElementById('model-selector'); // Replace with your specific selector

    // Helper function to get the 'data-value' from the target element
    const getSelectedOptionValue = () => {
        return targetElement?.getAttribute('data-value');
    };

    // Reference to all hidden input fields with the class 'set-my-prompt-id'
    const hiddenFields = document.querySelectorAll<HTMLInputElement>('input.set-my-prompt-id[type="hidden"]');

    // Function to update all hidden fields
    const updateHiddenFields = () => {
        const val = getSelectedOptionValue();
        console.log('data-value attribute changed:', val);

        if (val) {
            console.log('Updating hidden fields');
            hiddenFields.forEach((field) => {
                field.value = val;
            });
        }
    };

    // Set the initial value on page load if 'data-value' is present
    updateHiddenFields();

    // Callback function to execute when mutations are observed
    const callback = (mutationsList: MutationRecord[]) => {
        for (const mutation of mutationsList) {
            // Check if the 'data-value' attribute was modified
            if (mutation.type === 'attributes' && mutation.attributeName === 'data-value') {
                updateHiddenFields();
            }
        }
    };

    // Create an observer instance linked to the callback function
    const observer = new MutationObserver(callback);

    // Options for the observer (which mutations to observe)
    const config = {
        attributes: true,               // Observe changes to attributes
        attributeFilter: ['data-value'], // Only observe changes to the 'data-value' attribute
        subtree: true,                  // Include child elements in the observation
    };

    // Start observing the target element for configured mutations
    if (targetElement) {
        observer.observe(targetElement, config);
    }
};
