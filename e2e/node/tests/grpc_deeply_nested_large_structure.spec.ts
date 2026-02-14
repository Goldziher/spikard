/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcDeeplyNestedLargeStructure } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Deeply nested large structure", async () => {
		// Tests deeply nested protobuf messages with complex hierarchies. Validates that nested message serialization handles proper field numbering and recursive structures.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.RegistryService",
			methodName: "RegisterPerson",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcDeeplyNestedLargeStructure(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					success: true,
					person: { name: "John Doe", address: { street: "123 Main St", city: "Springfield" } },
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
