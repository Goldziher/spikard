/**
 * Basic Server Example
 *
 * The simplest possible Spikard application.
 * Starts a server on port 8000 with a single route that returns "Hello, World!".
 */

import { get, Spikard } from "@spikard/node";

const app = new Spikard();

/**
 * Simple GET handler returning plain text
 */
get("/")(async function handleRoot() {
	return "Hello, World!";
});

/**
 * Health check endpoint
 */
get("/health")(async function handleHealth() {
	return {
		status: "ok",
		timestamp: new Date().toISOString(),
	};
});

console.log("Starting Spikard Node.js server on http://127.0.0.1:8000");
console.log("Press Ctrl+C to stop\n");

// Run the server
app.run({ port: 8000, host: "0.0.0.0" });
