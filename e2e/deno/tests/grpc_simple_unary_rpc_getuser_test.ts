/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcSimpleUnaryRpcGetuser, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Simple unary RPC - GetUser", async () => {
  // Tests basic unary gRPC call with scalar types (int32, string). Covers fundamental request-response pattern.

  const metadata: Record<string, string> = {
    "authorization": "Bearer test-token",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.UserService",
    methodName: "GetUser",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcSimpleUnaryRpcGetuser(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ id: 123, name: "Alice Johnson", email: "alice@example.com" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});