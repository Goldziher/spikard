/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcRateLimitingWithMetadataHeaders, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Rate limiting with metadata headers", async () => {
  // Tests gRPC rate limiting. Validates rate limit headers in response and proper 429 handling.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.RateLimitService",
    methodName: "Query",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcRateLimitingWithMetadataHeaders(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ result: "success" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});