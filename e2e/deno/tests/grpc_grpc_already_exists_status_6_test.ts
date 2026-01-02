/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcAlreadyExistsStatus6, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC ALREADY_EXISTS status 6", async () => {
  // Tests ALREADY_EXISTS gRPC status code. Returned when trying to create a resource that already exists.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.CreateService",
    methodName: "Create",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcAlreadyExistsStatus6(request);

  // Verify response
  assertEquals(response.statusCode, "ALREADY_EXISTS");
  assert(response.metadata !== undefined && response.metadata !== null);
});