/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingUnicodeStringAggregation, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming - Unicode string aggregation", async () => {
  // Tests client streaming RPC with Unicode strings that are concatenated. Validates proper UTF-8 handling across multiple messages.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.TextService",
    methodName: "ConcatenateStrings",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingUnicodeStringAggregation(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ fragment_id: "unicode-001", result: "Hello, 世界! Привет 🌍", fragment_count: 4, total_length: 26, status: "CONCATENATED" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});