/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingValidationFailureMidStream, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming - validation failure mid-stream", async () => {
  // Tests client streaming RPC where a message fails validation in the middle of the stream. Server rejects the stream and returns error.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ValidationService",
    methodName: "ValidateUsers",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingValidationFailureMidStream(request);

  // Verify response
  assertEquals(response.statusCode, "INVALID_ARGUMENT");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ processed: 2, status: "VALIDATION_FAILED", error_message: "Invalid email format at message index 2: invalid-email" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});