/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcErrorHandlingPermissionDeniedClientStreaming, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Error handling - permission denied client streaming", async () => {
  // Tests client streaming RPC accessing unauthorized resource. Expects PERMISSION_DENIED status when client sends restricted access level requests. Demonstrates permission validation on streaming upload operations.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ErrorTestService",
    methodName: "UploadRestricted",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingPermissionDeniedClientStreaming(request);

  // Verify response
  assertEquals(response.statusCode, "PERMISSION_DENIED");
  assert(response.metadata !== undefined && response.metadata !== null);
});