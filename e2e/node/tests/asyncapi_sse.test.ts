import { createAppSseNotifications } from "../app/main.js";

/**
 * AsyncAPI SSE tests
 * @generated
 */

import { TestClient } from "@spikard/node";
import { describe, expect, test } from "vitest";

describe("asyncapi_sse", () => {
	test("SSE /notifications", async () => {
		const app = createAppSseNotifications();
		const client = new TestClient(app);
		const response = await client.get("/notifications");
		expect(response.statusCode).toBe(200);
		const normalized = response.text().replace(/\r\n/g, "\n");
		const events = normalized
			.split("\n\n")
			.filter((chunk) => chunk.startsWith("data:"))
			.map((chunk) => chunk.slice(5).trim());
		const expected = [
			'{"level":"example_level","message":"example_message","source":"example_source","timestamp":"2024-01-15T10:30:00Z","type":"system_alert"}',
			'{"body":"example_body","priority":"example_priority","timestamp":"2024-01-15T10:30:00Z","title":"example_title","type":"user_notification","userId":"example_userId"}',
			'{"message":"example_message","metadata":{},"service":"example_service","status":"example_status","timestamp":"2024-01-15T10:30:00Z","type":"status_update"}',
		];
		expect(events.length).toBe(expected.length);
		events.forEach((payload, index) => {
			expect(JSON.parse(payload)).toEqual(JSON.parse(expected[index]));
		});
	});
});
