#!/usr/bin/env node

/**
 * Test application generated from AsyncAPI specification
 */

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import WebSocket from "ws";

// ES modules compatibility
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Load test fixtures
const FIXTURES_DIR = join(__dirname, "..", "..", "testing_data", "websockets");

function loadFixture(name: string): any {
	const fixturePath = join(FIXTURES_DIR, `${name}.json`);
	const content = readFileSync(fixturePath, "utf-8");
	return JSON.parse(content);
}

function validateMessage(message: any, fixtureName: string): boolean {
	try {
		const fixture = loadFixture(fixtureName);
		const schema = fixture.schema || {};
		const required = schema.required || [];

		// Basic validation - check required fields
		for (const field of required) {
			if (!(field in message)) {
				console.log(`❌ Missing required field: ${field}`);
				return false;
			}
		}

		console.log(`✓ Message validated against ${fixtureName}`);
		return true;
	} catch (error) {
		console.log(`❌ Validation error: ${error}`);
		return false;
	}
}

async function handleWebSocket(uri: string): Promise<void> {
	console.log(`Connecting to ${uri}...`);

	return new Promise((resolve, reject) => {
		const ws = new WebSocket(uri);

		ws.on("open", () => {
			console.log("✓ Connected");

			// Send example messages
			const fixture_userLeft = loadFixture("userLeft");
			const example_userLeft = fixture_userLeft.examples[0];
			console.log("Sending userLeft message...");
			ws.send(JSON.stringify(example_userLeft));

			const fixture_chatMessage = loadFixture("chatMessage");
			const example_chatMessage = fixture_chatMessage.examples[0];
			console.log("Sending chatMessage message...");
			ws.send(JSON.stringify(example_chatMessage));

			const fixture_userJoined = loadFixture("userJoined");
			const example_userJoined = fixture_userJoined.examples[0];
			console.log("Sending userJoined message...");
			ws.send(JSON.stringify(example_userJoined));
		});

		ws.on("message", (data: WebSocket.Data) => {
			const message = JSON.parse(data.toString());
			const msgType = message.type || "unknown";
			console.log(`Received message type: ${msgType}`);

			// Validate based on message type
			if (msgType === "userLeft") {
				validateMessage(message, "userLeft");
			}
			if (msgType === "chatMessage") {
				validateMessage(message, "chatMessage");
			}
			if (msgType === "userJoined") {
				validateMessage(message, "userJoined");
			}
		});

		ws.on("close", () => {
			console.log("Connection closed");
			resolve();
		});

		ws.on("error", (error) => {
			console.error("WebSocket error:", error);
			reject(error);
		});
	});
}

async function main(): Promise<void> {
	// Default WebSocket URI - override with environment variable WS_URI
	const uri = process.env.WS_URI || "ws://localhost:8000/chat";
	await handleWebSocket(uri);
}

// Run main function
main().catch(console.error);
