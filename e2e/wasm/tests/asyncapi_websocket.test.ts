/**
 * AsyncAPI WebSocket tests
 * @generated
 */

import { readFileSync } from "node:fs";
import path from "node:path";
import { TestClient } from "@spikard/wasm/node";
import { describe, expect, test } from "vitest";
import {
	ChatMessageMessageSchema,
	createAppWebsocketChat,
	UserJoinedMessageSchema,
	UserLeftMessageSchema,
} from "../app/main.ts";

const ROOT_DIR = path.resolve(__dirname, "../../..");
const WEBSOCKET_FIXTURE_ROOT = path.join(ROOT_DIR, "testing_data", "websockets");

function loadFixtureExamples(name: string): string[] {
	const fixturePath = path.join(WEBSOCKET_FIXTURE_ROOT, `${name}.json`);
	const data = JSON.parse(readFileSync(fixturePath, "utf-8"));
	const examples = Array.isArray(data.examples) ? data.examples : [];
	if (examples.length === 0) {
		return [JSON.stringify({})];
	}
	return examples.map((example) => JSON.stringify(example));
}

describe("asyncapi_websocket", () => {
	test("WebSocket /chat", async () => {
		const app = createAppWebsocketChat();
		const client = new TestClient(app);
		const ws = await client.websocketConnect("/chat");
		const fixtures = [
			{ name: "chatMessage", schema: ChatMessageMessageSchema },
			{ name: "userLeft", schema: UserLeftMessageSchema },
			{ name: "userJoined", schema: UserJoinedMessageSchema },
		];
		for (const { name, schema } of fixtures) {
			const payload = schema.parse(JSON.parse(loadFixtureExamples(name)[0] ?? "{}"));
			await ws.sendJson(payload);
			const response = await ws.receiveJson();
			expect(response.validated).toBe(true);
			for (const [key, value] of Object.entries(payload)) {
				expect(response[key]).toEqual(value);
			}
		}
		await ws.close();
	});
});
