/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcInvalidArgumentStatus3, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC INVALID_ARGUMENT status 3", async () => {
  // Tests INVALID_ARGUMENT gRPC status code. Indicates that the client provided an invalid or malformed argument.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ArgService",
    methodName: "Validate",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcInvalidArgumentStatus3(request);

  // Verify response
  assertEquals(response.statusCode, "INVALID_ARGUMENT");
  assert(response.metadata !== undefined && response.metadata !== null);
});