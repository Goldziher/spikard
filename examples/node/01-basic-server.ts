/**
 * Basic Server Example
 *
 * The simplest possible Spikard application.
 * Starts a server on port 8000 with a single route that returns "Hello, World!".
 */

import { get, Spikard } from "@spikard/node";

const app = new Spikard({
	port: 8000,
});

/**
 * Simple GET handler returning plain text
 */
@get("/")
async function handleRoot() {
	return "Hello, World!";
}

/**
 * Health check endpoint
 */
@get("/health")
async function handleHealth() {
	return {
		status: "ok",
		timestamp: new Date().toISOString(),
	};
}

// Register handlers
app.registerHandler(handleRoot);
app.registerHandler(handleHealth);

console.log("Starting Spikard Node.js server on http://127.0.0.1:8000");
console.log("Press Ctrl+C to stop\n");

// Run the server
app.listen().catch((error) => {
	console.error("Server error:", error);
	process.exit(1);
});
