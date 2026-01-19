/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingFilterValidMessages, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming - filter valid messages", async () => {
  // Tests bidirectional streaming RPC where server filters out invalid messages during streaming.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.FilterService",
    methodName: "FilterValid",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingFilterValidMessages(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assert(response.metadata !== undefined && response.metadata !== null);
});