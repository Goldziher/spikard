/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcValidationErrorInvalidArgumentWithDetails, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Validation error - INVALID_ARGUMENT with details", async () => {
  // Tests INVALID_ARGUMENT status code with detailed validation error information. Demonstrates how validation failures are communicated.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ValidationService",
    methodName: "ValidateInput",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcValidationErrorInvalidArgumentWithDetails(request);

  // Verify response
  assertEquals(response.statusCode, "INVALID_ARGUMENT");
  assert(response.metadata !== undefined && response.metadata !== null);
});