/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcPermissionDeniedStatus7, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC PERMISSION_DENIED status 7", async () => {
  // Tests PERMISSION_DENIED gRPC status code. Returned when the caller does not have sufficient permissions.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.SecureService",
    methodName: "AdminAction",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcPermissionDeniedStatus7(request);

  // Verify response
  assertEquals(response.statusCode, "PERMISSION_DENIED");
  assert(response.metadata !== undefined && response.metadata !== null);
});