#!/usr/bin/env node

/**
 * Test application generated from AsyncAPI specification
 */

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import fetch from "node-fetch";

// ES modules compatibility
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Load test fixtures
const FIXTURES_DIR = join(__dirname, "..", "..", "testing_data", "sse");

function loadFixture(name: string): any {
	const fixturePath = join(FIXTURES_DIR, `${name}.json`);
	const content = readFileSync(fixturePath, "utf-8");
	return JSON.parse(content);
}

async function handleSSE(url: string): Promise<void> {
	console.log(`Connecting to ${url}...`);

	const response = await fetch(url);
	console.log("✓ Connected");

	const reader = response.body?.getReader();
	const decoder = new TextDecoder();

	while (reader) {
		const { done, value } = await reader.read();
		if (done) break;

		const chunk = decoder.decode(value);
		const lines = chunk.split("\n");

		for (const line of lines) {
			if (line.startsWith("data:")) {
				const data = line.slice(5).trim();
				try {
					const message = JSON.parse(data);
					console.log("Received event:", message);
				} catch {
					console.log("Received:", data);
				}
			}
		}
	}
}

async function main(): Promise<void> {
	// Default SSE URI - override with environment variable SSE_URI
	const url = process.env.SSE_URI || "http://localhost:8000/notifications";
	await handleSSE(url);
}

// Run main function
main().catch(console.error);
