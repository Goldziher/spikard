/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcLargeRepeatedFieldWith10000Items, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Large repeated field with 10 000 items", async () => {
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
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ series_id: "metrics-large-series", point_count: 10000, min_value: 10.5, max_value: 99.9 })));
  assert(response.metadata !== undefined && response.metadata !== null);
});