/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcErrorHandlingInvalidRequestPayload, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Error handling - invalid request payload", async () => {
  // Tests server streaming RPC with invalid request payload. Validates that INVALID_ARGUMENT status is returned when required field is missing from the request message. The server should reject the malformed payload before beginning the stream.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ErrorTestService",
    methodName: "ValidateRequest",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingInvalidRequestPayload(request);

  // Verify response
  assertEquals(response.statusCode, "INVALID_ARGUMENT");
  assert(response.metadata !== undefined && response.metadata !== null);
});