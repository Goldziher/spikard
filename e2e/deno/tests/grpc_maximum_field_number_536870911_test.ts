/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcMaximumFieldNumber536870911, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Maximum field number 536870911", async () => {
  // Tests protobuf messages using the maximum allowed field number (536870911). Validates proper field number encoding in varint format.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.MaxFieldService",
    methodName: "TestMaxField",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcMaximumFieldNumber536870911(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ id: 42, received_max: "Testing maximum field number" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});