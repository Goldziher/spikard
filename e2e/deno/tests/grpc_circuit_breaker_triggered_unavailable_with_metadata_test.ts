/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcCircuitBreakerTriggeredUnavailableWithMetadata, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Circuit breaker triggered - UNAVAILABLE with metadata", async () => {
  // Tests UNAVAILABLE status code with circuit breaker metadata. Indicates service degradation and when to retry.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.DownstreamService",
    methodName: "Query",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcCircuitBreakerTriggeredUnavailableWithMetadata(request);

  // Verify response
  assertEquals(response.statusCode, "UNAVAILABLE");
  assert(response.metadata !== undefined && response.metadata !== null);
});