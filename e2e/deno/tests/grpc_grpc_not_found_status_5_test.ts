/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcNotFoundStatus5, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC NOT_FOUND status 5", async () => {
  // Tests NOT_FOUND gRPC status code. Returned when a requested resource (e.g., user, file) does not exist.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ResourceService",
    methodName: "Get",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcNotFoundStatus5(request);

  // Verify response
  assertEquals(response.statusCode, "NOT_FOUND");
  assert(response.metadata !== undefined && response.metadata !== null);
});