/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcTimeoutWithRetryMetadata, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Timeout with retry metadata", async () => {
  // Tests DEADLINE_EXCEEDED status code with retry metadata in response trailers. Indicates whether client should retry.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.RetryService",
    methodName: "RetryableOperation",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcTimeoutWithRetryMetadata(request);

  // Verify response
  assertEquals(response.statusCode, "DEADLINE_EXCEEDED");
  assert(response.metadata !== undefined && response.metadata !== null);
});