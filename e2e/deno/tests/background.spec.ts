/**
 * E2E tests for background
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { assertEquals } from "@std/assert";
import {
	createAppBackgroundBackgroundEventLogging,
	createAppBackgroundBackgroundEventLoggingSecondPayload,
} from "../app/main.ts";
Deno.test("background: Background event logging - second payload", async () => {
	const app = createAppBackgroundBackgroundEventLoggingSecondPayload();
	const client = new TestClient(app);

	const json = { event: "beta" };
	const response = await client.post("/background/events", { json });

	assertEquals(response.statusCode, 202);
	const stateResponse = await client.get("/background/events");
	assertEquals(stateResponse.statusCode, 200);
	expect(stateResponse.json()).toStrictEqual({ events: ["beta"] });
});

Deno.test("background: Background event logging", async () => {
	const app = createAppBackgroundBackgroundEventLogging();
	const client = new TestClient(app);

	const json = { event: "alpha" };
	const response = await client.post("/background/events", { json });

	expect(response.statusCode).toBe(202);
	const stateResponse = await client.get("/background/events");
	expect(stateResponse.statusCode).toBe(200);
	expect(stateResponse.json()).toStrictEqual({ events: ["alpha"] });
});
