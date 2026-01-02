/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcResourceExhaustedStatus8, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC RESOURCE_EXHAUSTED status 8", async () => {
  // Tests RESOURCE_EXHAUSTED gRPC status code. Returned when the server has run out of resources (disk space, memory, connections, etc.).

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ResourceService",
    methodName: "AllocateMemory",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcResourceExhaustedStatus8(request);

  // Verify response
  assertEquals(response.statusCode, "RESOURCE_EXHAUSTED");
  assert(response.metadata !== undefined && response.metadata !== null);
});