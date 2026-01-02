/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcMapFieldHandlingMapStringMessage, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Map field handling - Map string Message", async () => {
  // Tests protobuf map fields with string keys and message values. Validates proper key-value pair serialization and access patterns.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.MapService",
    methodName: "ProcessMap",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcMapFieldHandlingMapStringMessage(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ id: "map-test-001", map_count: 3, keys: ["key1", "key2", "key3"] })));
  assert(response.metadata !== undefined && response.metadata !== null);
});