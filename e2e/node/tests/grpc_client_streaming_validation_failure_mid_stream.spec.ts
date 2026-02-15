/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingValidationFailureMidStream } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming - validation failure mid-stream", async () => {
		// Tests client streaming RPC where a message fails validation in the middle of the stream. Server rejects the stream and returns error.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ValidationService",
			methodName: "ValidateUsers",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingValidationFailureMidStream(request);

		// Verify response
		expect(response.statusCode).toBe("INVALID_ARGUMENT");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					processed: 2,
					status: "VALIDATION_FAILED",
					error_message: "Invalid email format at message index 2: invalid-email",
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
