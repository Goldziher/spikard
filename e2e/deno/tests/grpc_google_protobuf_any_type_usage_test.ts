/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGoogleProtobufAnyTypeUsage, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Google protobuf Any type usage", async () => {
  // Tests usage of google.protobuf.Any for storing arbitrary message types. Validates type URL encoding and message packing.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.AnyService",
    methodName: "ProcessAny",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGoogleProtobufAnyTypeUsage(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ request_id: "any-test-001", type_name: "example.v1.Container", success: true })));
  assert(response.metadata !== undefined && response.metadata !== null);
});