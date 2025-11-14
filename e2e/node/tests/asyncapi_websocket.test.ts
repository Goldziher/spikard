import { createAppWebsocketChat } from "../app/main.js";

/**
 * AsyncAPI WebSocket tests
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";

describe("asyncapi_websocket", () => {
	test("WebSocket /chat", async () => {
		const app = createAppWebsocketChat();
		const client = new TestClient(app);
		const ws = await client.websocketConnect("/chat");

		// Send chatMessage message
		const sent_message_chatmessage = JSON.parse(
			'{"text":"example_text","timestamp":"2024-01-15T10:30:00Z","type":"message","user":"example_user"}',
		);
		await ws.sendJson(sent_message_chatmessage);

		// Receive echo response
		const response_chatmessage = await ws.receiveJson();
		expect(response_chatmessage.validated).toBe(true);

		// Verify echoed fields match sent message
		for (const [key, value] of Object.entries(sent_message_chatmessage)) {
			expect(response_chatmessage[key]).toEqual(value);
		}

		// Send userLeft message
		const sent_message_userleft = JSON.parse(
			'{"timestamp":"2024-01-15T10:30:00Z","type":"userLeft","user":"example_user"}',
		);
		await ws.sendJson(sent_message_userleft);

		// Receive echo response
		const response_userleft = await ws.receiveJson();
		expect(response_userleft.validated).toBe(true);

		// Verify echoed fields match sent message
		for (const [key, value] of Object.entries(sent_message_userleft)) {
			expect(response_userleft[key]).toEqual(value);
		}

		// Send userJoined message
		const sent_message_userjoined = JSON.parse(
			'{"timestamp":"2024-01-15T10:30:00Z","type":"userJoined","user":"example_user"}',
		);
		await ws.sendJson(sent_message_userjoined);

		// Receive echo response
		const response_userjoined = await ws.receiveJson();
		expect(response_userjoined.validated).toBe(true);

		// Verify echoed fields match sent message
		for (const [key, value] of Object.entries(sent_message_userjoined)) {
			expect(response_userjoined[key]).toEqual(value);
		}

		await ws.close();
	});
});
