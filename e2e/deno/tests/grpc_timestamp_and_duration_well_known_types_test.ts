/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcTimestampAndDurationWellKnownTypes, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Timestamp and Duration well-known types", async () => {
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
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ event_id: "event-001", processed_at: "2024-01-15T10:31:45.123Z", processing_time_ms: 1000 })));
  assert(response.metadata !== undefined && response.metadata !== null);
});