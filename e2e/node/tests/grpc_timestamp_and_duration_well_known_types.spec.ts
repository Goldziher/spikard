/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcTimestampAndDurationWellKnownTypes } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Timestamp and Duration well-known types", async () => {
		// Tests usage of google.protobuf.Timestamp and Duration types. Validates RFC 3339 timestamp serialization and duration calculations.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.EventService",
			methodName: "LogEvent",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcTimestampAndDurationWellKnownTypes(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({ event_id: "event-001", processed_at: "2024-01-15T10:31:45.123Z", processing_time_ms: 1000 }),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
