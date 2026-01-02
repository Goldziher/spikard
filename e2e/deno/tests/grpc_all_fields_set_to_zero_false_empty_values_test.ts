/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcAllFieldsSetToZeroFalseEmptyValues, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: All fields set to zero false empty values", async () => {
  // Tests proto3 default value behavior when all fields are explicitly set to zero, false, empty string. Validates that zero values are transmitted correctly.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ZeroValueService",
    methodName: "ProcessZeros",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcAllFieldsSetToZeroFalseEmptyValues(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ success: true, fields_received: 5 })));
  assert(response.metadata !== undefined && response.metadata !== null);
});