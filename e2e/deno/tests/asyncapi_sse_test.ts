/**
 * AsyncAPI SSE tests
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assertEquals } from "jsr:@std/assert@1";
import { join, resolve } from "jsr:@std/path@1";
import {
	createAppSseNotifications,
	NotificationBatchMessageSchema,
	StatusUpdateMessageSchema,
	SystemAlertMessageSchema,
	UserNotificationMessageSchema,
} from "../app/main.ts";

const ROOT_DIR = resolve(new URL(".", import.meta.url).pathname, "../../..");
const SSE_FIXTURE_ROOT = join(ROOT_DIR, "testing_data", "sse");

function loadFixtureExamples(name: string): string[] {
	const fixturePath = join(SSE_FIXTURE_ROOT, `${name}.json`);
	const data = JSON.parse(Deno.readTextFileSync(fixturePath));
	const examples = Array.isArray(data.examples) ? data.examples : [];
	if (examples.length === 0) {
		return [JSON.stringify({})];
	}
	return examples.map((example) => JSON.stringify(example));
}

Deno.test("asyncapi_sse: SSE /notifications", async () => {
	const app = createAppSseNotifications();
	const client = new TestClient(app);
	const response = await client.get("/notifications");
	assertEquals(response.statusCode, 200);
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
	assertEquals(events.length, expected.length);
	events.forEach((payload, index) => {
		assertEquals(JSON.parse(payload), expected[index]);
	});
});
