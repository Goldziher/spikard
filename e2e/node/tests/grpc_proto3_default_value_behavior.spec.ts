/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcProto3DefaultValueBehavior } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Proto3 default value behavior", async () => {
		// Tests how proto3 handles implicit default values. When fields are omitted from the request, response should reflect appropriate defaults.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.DefaultService",
			methodName: "CheckDefaults",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcProto3DefaultValueBehavior(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify({ id: 1, name: "", active: false, has_id: true })));
		expect(response.metadata).toBeDefined();
	});
});
