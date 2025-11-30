/**
 * AsyncAPI SSE tests
 * @generated
 */

import { readFileSync } from "node:fs";
import path from "node:path";
import { TestClient } from "spikard-wasm/node";
import { describe, expect, test } from "vitest";
import {
	createAppSseNotifications,
	NotificationBatchMessageSchema,
	StatusUpdateMessageSchema,
	SystemAlertMessageSchema,
	UserNotificationMessageSchema,
} from "../app/main.ts";

const ROOT_DIR = path.resolve(__dirname, "../../..");
const SSE_FIXTURE_ROOT = path.join(ROOT_DIR, "testing_data", "sse");

function loadFixtureExamples(name: string): string[] {
	const fixturePath = path.join(SSE_FIXTURE_ROOT, `${name}.json`);
	const data = JSON.parse(readFileSync(fixturePath, "utf-8"));
	const examples = Array.isArray(data.examples) ? data.examples : [];
	if (examples.length === 0) {
		return [JSON.stringify({})];
	}
	return examples.map((example) => JSON.stringify(example));
}

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
		const fixtures = [
			{ name: "systemAlert", schema: SystemAlertMessageSchema },
			{ name: "notificationBatch", schema: NotificationBatchMessageSchema },
			{ name: "userNotification", schema: UserNotificationMessageSchema },
			{ name: "statusUpdate", schema: StatusUpdateMessageSchema },
		];
		const expected = fixtures.flatMap(({ name, schema }) =>
			loadFixtureExamples(name).map((payload) => schema.parse(JSON.parse(payload))),
		);
		expect(events.length).toBe(expected.length);
		events.forEach((payload, index) => {
			expect(JSON.parse(payload)).toEqual(expected[index]);
		});
	});
});
