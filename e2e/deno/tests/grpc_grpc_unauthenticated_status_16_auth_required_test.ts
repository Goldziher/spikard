/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcUnauthenticatedStatus16AuthRequired, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC UNAUTHENTICATED status 16 - auth required", async () => {
  // Tests UNAUTHENTICATED gRPC status code. Returned when the request lacks valid authentication credentials.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.AuthService",
    methodName: "SecureOp",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcUnauthenticatedStatus16AuthRequired(request);

  // Verify response
  assertEquals(response.statusCode, "UNAUTHENTICATED");
  assert(response.metadata !== undefined && response.metadata !== null);
});