/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcErrorHandlingResourceExhausted, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Error handling - resource exhausted", async () => {
  // Tests bidirectional streaming RPC exceeding rate limits. Expects RESOURCE_EXHAUSTED status when client attempts to send 100 messages in rapid succession, exceeding the 100 requests/second rate limit threshold.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ErrorTestService",
    methodName: "RateLimitedChat",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingResourceExhausted(request);

  // Verify response
  assertEquals(response.statusCode, "RESOURCE_EXHAUSTED");
  assert(response.metadata !== undefined && response.metadata !== null);
});