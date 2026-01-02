/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcProto3DefaultValueBehavior, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Proto3 default value behavior", async () => {
  // Tests how proto3 handles implicit default values. When fields are omitted from the request, response should reflect appropriate defaults.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.DefaultService",
    methodName: "CheckDefaults",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcProto3DefaultValueBehavior(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ id: 1, name: "", active: false, has_id: true })));
  assert(response.metadata !== undefined && response.metadata !== null);
});