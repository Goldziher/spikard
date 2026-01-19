/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcErrorHandlingUnimplementedMethod, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Error handling - unimplemented method", async () => {
  // Tests unary RPC calling an unimplemented method. Validates that UNIMPLEMENTED status is returned when the server does not support the requested RPC method. This fixture ensures proper error handling for feature requests that are not yet available in the current server implementation.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ErrorTestService",
    methodName: "FutureFeature",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingUnimplementedMethod(request);

  // Verify response
  assertEquals(response.statusCode, "UNIMPLEMENTED");
  assert(response.metadata !== undefined && response.metadata !== null);
});