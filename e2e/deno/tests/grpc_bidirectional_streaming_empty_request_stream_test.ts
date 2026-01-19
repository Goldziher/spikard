/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingEmptyRequestStream, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming - empty request stream", async () => {
  // Tests bidirectional streaming RPC with empty request stream but server sends response.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.EmptyService",
    methodName: "HandleEmpty",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingEmptyRequestStream(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assert(response.metadata !== undefined && response.metadata !== null);
});