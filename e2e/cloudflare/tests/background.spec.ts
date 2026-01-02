/**
 * E2E tests for background
 * @generated
 */

import { TestClient } from "@spikard/wasm";
import { describe, expect, test } from "vitest";
import {
	createAppBackgroundBackgroundEventLogging,
	createAppBackgroundBackgroundEventLoggingSecondPayload,
} from "../app/main.ts";

describe("background", () => {
	test("Background event logging - second payload", async () => {
		const app = createAppBackgroundBackgroundEventLoggingSecondPayload();
		const client = new TestClient(app);

		const json = { event: "beta" };
		const response = await client.post("/background/events", { json });

		expect(response.statusCode).toBe(202);
		const stateResponse = await client.get("/background/events");
		expect(stateResponse.statusCode).toBe(200);
		expect(stateResponse.json()).toStrictEqual({ events: ["beta"] });
	});

	test("Background event logging", async () => {
		const app = createAppBackgroundBackgroundEventLogging();
		const client = new TestClient(app);

		const json = { event: "alpha" };
		const response = await client.post("/background/events", { json });

		expect(response.statusCode).toBe(202);
		const stateResponse = await client.get("/background/events");
		expect(stateResponse.statusCode).toBe(200);
		expect(stateResponse.json()).toStrictEqual({ events: ["alpha"] });
	});
});
