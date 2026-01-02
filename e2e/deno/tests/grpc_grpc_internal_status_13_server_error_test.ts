/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcInternalStatus13ServerError, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC INTERNAL status 13 - server error", async () => {
  // Tests INTERNAL gRPC status code. Returned when an internal server error occurs.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.InternalService",
    methodName: "Fail",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcInternalStatus13ServerError(request);

  // Verify response
  assertEquals(response.statusCode, "INTERNAL");
  assert(response.metadata !== undefined && response.metadata !== null);
});