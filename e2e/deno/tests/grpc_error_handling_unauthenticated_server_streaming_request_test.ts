/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcErrorHandlingUnauthenticatedServerStreamingRequest, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Error handling - unauthenticated server streaming request", async () => {
  // Tests server streaming RPC without required auth metadata. Expects UNAUTHENTICATED status when authorization header is missing.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ErrorTestService",
    methodName: "SecureStream",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingUnauthenticatedServerStreamingRequest(request);

  // Verify response
  assertEquals(response.statusCode, "UNAUTHENTICATED");
  assert(response.metadata !== undefined && response.metadata !== null);
});