/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcLargeRepeatedFieldWith10000Items } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Large repeated field with 10 000 items", async () => {
		// Tests handling of repeated fields containing thousands of elements. Validates efficient serialization and deserialization of large arrays without memory bloat.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.MetricsService",
			methodName: "IngestTimeSeries",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcLargeRepeatedFieldWith10000Items(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({ series_id: "metrics-large-series", point_count: 10000, min_value: 10.5, max_value: 99.9 }),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
