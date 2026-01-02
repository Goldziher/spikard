/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcErrorHandlingGrpcStatusCodes, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Error handling - gRPC status codes", async () => {
  // Tests gRPC error status codes and error responses. Covers NOT_FOUND, INVALID_ARGUMENT, INTERNAL, and other gRPC status codes.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ProductService",
    methodName: "GetProduct",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingGrpcStatusCodes(request);

  // Verify response
  assertEquals(response.statusCode, "INVALID_ARGUMENT");
  assert(response.metadata !== undefined && response.metadata !== null);
});