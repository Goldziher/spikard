/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcErrorHandlingStreamErrorMidTransmission, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Error handling - stream error mid-transmission", async () => {
  // Tests server streaming RPC that errors after yielding 3 messages. The stream opens successfully and delivers 3 messages before encountering an INTERNAL error. Validates that partial stream data is delivered before the error is signaled.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ErrorTestService",
    methodName: "StreamWithError",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingStreamErrorMidTransmission(request);

  // Verify response
  assertEquals(response.statusCode, "INTERNAL");
  assert(response.metadata !== undefined && response.metadata !== null);
});