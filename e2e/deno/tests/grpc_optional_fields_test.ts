/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcOptionalFields, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Optional fields", async () => {
  // Tests optional field handling with presence semantics. Covers optional fields with and without values.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.UserService",
    methodName: "UpdateProfile",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcOptionalFields(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ user_id: 42, username: "charlie_dev", bio: "Software engineer and gRPC enthusiast", updated_at: 1704067200000 })));
  assert(response.metadata !== undefined && response.metadata !== null);
});