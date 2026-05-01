/**
 * Smoke tests: verify TestClient surface is exported and has expected methods.
 *
 * These tests do not require a running HTTP server — they only verify that the
 * generated NAPI binding exports the expected types and method names.
 *
 * Run after `napi build --release` (or `napi build` for debug):
 *   pnpm test
 */

import * as assert from "node:assert/strict";
import { describe, it } from "node:test";

// The NAPI binding is loaded from the compiled .node file via index.js.
// Adjust the import path if your project layout differs.
// eslint-disable-next-line @typescript-eslint/no-require-imports
const binding = require("../../crates/spikard-node/index") as Record<string, unknown>;

describe("TestClient surface", () => {
	it("exports TestClient class", () => {
		assert.ok("JsTestClient" in binding || "TestClient" in binding, "TestClient must be exported from binding");
	});

	it("TestClient has HTTP method functions", () => {
		const ClientCls = (binding["JsTestClient"] ?? binding["TestClient"]) as Record<string, unknown> | undefined;
		assert.ok(ClientCls !== undefined, "TestClient class must be defined");
		const proto = (ClientCls as { prototype?: Record<string, unknown> }).prototype ?? {};
		for (const method of ["get", "post", "put", "patch", "delete"]) {
			assert.ok(method in proto, `TestClient must have method: ${method}`);
		}
	});

	it("exports ResponseSnapshot class", () => {
		assert.ok(
			"JsResponseSnapshot" in binding || "ResponseSnapshot" in binding,
			"ResponseSnapshot must be exported from binding",
		);
	});

	it("exports SnapshotError enum", () => {
		assert.ok(
			"JsSnapshotError" in binding || "SnapshotError" in binding,
			"SnapshotError must be exported from binding",
		);
	});

	it("exports GraphQLSubscriptionSnapshot class", () => {
		assert.ok(
			"JsGraphQLSubscriptionSnapshot" in binding || "GraphQLSubscriptionSnapshot" in binding,
			"GraphQLSubscriptionSnapshot must be exported from binding",
		);
	});

	it("exports WebSocketMessage enum", () => {
		assert.ok(
			"JsWebSocketMessage" in binding || "WebSocketMessage" in binding,
			"WebSocketMessage must be exported from binding",
		);
	});
});
