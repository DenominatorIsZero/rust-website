// Shared WASM Demo Initialization
// Handles loading and error handling for all AI demos

// Initialize WASM demo by importing the interactive module
async function initializeDemo() {
    try {
        // Determine the path to the demo's interactive.js file
        // This script is loaded from /static/js/ but needs to find the demo's interactive.js
        const currentPath = window.location.pathname;
        const demoMatch = currentPath.match(/\/static\/wasm\/([^\/]+)\/index\.html/);
        
        if (!demoMatch) {
            throw new Error('Could not determine demo path');
        }
        
        const demoName = demoMatch[1];
        const interactivePath = `/static/wasm/${demoName}/interactive.js`;
        
        // Import the demo-specific interactive.js module
        const init = (await import(interactivePath)).default;
        await init();
    } catch (error) {
        // Handle specific wasm-bindgen control flow exception
        if (!error.message.startsWith("Using exceptions for control flow")) {
            console.error("Failed to initialize WASM demo:", error);
            
            // Display user-friendly error message
            document.body.innerHTML = `
                <div class="demo-error">
                    <h2>Demo Failed to Load</h2>
                    <p>Error: ${error.message}</p>
                    <p class="demo-error-hint">
                        Please refresh the page to try again.
                    </p>
                </div>
            `;
        }
    }
}

// Start initialization when the module loads
initializeDemo();