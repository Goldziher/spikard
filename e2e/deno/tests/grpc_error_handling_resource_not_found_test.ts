/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcErrorHandlingResourceNotFound, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Error handling - resource not found", async () => {
  // Tests NOT_FOUND gRPC status code. Returned when the requested resource does not exist. Validates unary RPC requesting non-existent resource.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ErrorTestService",
    methodName: "GetResource",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingResourceNotFound(request);

  // Verify response
  assertEquals(response.statusCode, "NOT_FOUND");
  assert(response.metadata !== undefined && response.metadata !== null);
});