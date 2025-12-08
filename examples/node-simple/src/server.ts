/**
 * Simple Spikard Node.js example
 *
 * Demonstrates basic routing with GET and POST handlers
 */

import { Spikard, get, post, type HandlerFunction } from "@spikard/node";

const app = new Spikard();

/**
 * Root endpoint - returns welcome message
 */
const handleRoot: HandlerFunction = async () => {
	return {
		message: "Hello from Spikard Node!",
		timestamp: new Date().toISOString(),
	};
};
get("/")(handleRoot);

/**
 * Health check endpoint
 */
const handleHealth: HandlerFunction = async () => {
	return {
		status: "healthy",
		uptime: process.uptime(),
	};
};
get("/health")(handleHealth);

/**
 * Get user by ID
 */
const getUserById: HandlerFunction = async (req) => {
	const userId = req.params.id ?? "unknown";
	return {
		user_id: userId,
		name: "Test User",
	};
};
get("/users/:id")(getUserById);

/**
 * Echo endpoint - returns request body
 */
const handleEcho: HandlerFunction = async (req) => {
	const body = req.body ? req.json() : null;
	return {
		echoed: true,
		body,
		receivedAt: new Date().toISOString(),
	};
};
post("/echo")(handleEcho);

console.log("Starting Spikard Node.js server on http://0.0.0.0:8000");
console.log("Press Ctrl+C to stop\n");

app.run({ port: 8000, host: "0.0.0.0" });
