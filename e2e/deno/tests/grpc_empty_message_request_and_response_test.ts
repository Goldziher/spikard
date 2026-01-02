/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcEmptyMessageRequestAndResponse, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Empty message request and response", async () => {
  // Tests handling of empty protobuf messages with no fields. Validates that the protocol correctly handles minimal payloads.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.PingService",
    methodName: "Ping",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcEmptyMessageRequestAndResponse(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({  })));
  assert(response.metadata !== undefined && response.metadata !== null);
});