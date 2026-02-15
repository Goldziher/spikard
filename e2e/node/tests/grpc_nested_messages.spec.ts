/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcNestedMessages } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Nested messages", async () => {
		// Tests nested message types with complex field hierarchies. Covers nested message definitions and serialization.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.UserService",
			methodName: "CreateUser",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcNestedMessages(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					user_id: 456,
					name: "Bob Smith",
					email: "bob@example.com",
					address: { street: "123 Main St", city: "Springfield", zip_code: "12345" },
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
