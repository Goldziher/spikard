/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcNestedMessages, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Nested messages", async () => {
  // Tests nested message types with complex field hierarchies. Covers nested message definitions and serialization.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.UserService",
    methodName: "CreateUser",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcNestedMessages(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ user_id: 456, name: "Bob Smith", email: "bob@example.com", address: { street: "123 Main St", city: "Springfield", zip_code: "12345" } })));
  assert(response.metadata !== undefined && response.metadata !== null);
});