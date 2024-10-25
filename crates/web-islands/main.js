async function loadWasm() {
    try {
        const response = await fetch('pkg/web_islands_bg.wasm'); // Update with the correct path to your WASM file
        if (!response.ok) {
            throw new Error(`Failed to fetch .wasm file: ${response.statusText}`);
        }

        // Create an empty imports object if no imports are required
        const imports = {};

        // Use WebAssembly.instantiateStreaming to instantiate the module
        const { instance } = await WebAssembly.instantiateStreaming(response, imports);

        console.log('WASM loaded successfully', instance);
    } catch (err) {
        console.error('Error loading WASM:', err);
    }
}

loadWasm();
