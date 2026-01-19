/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingErrorMidStream, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming - error mid-stream", async () => {
  // Tests bidirectional streaming RPC where server returns error after processing some messages.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ErrorService",
    methodName: "ProcessWithError",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingErrorMidStream(request);

  // Verify response
  assertEquals(response.statusCode, "INTERNAL");
  assertEquals(response.payload, Buffer.from(JSON.stringify("Error after processing 2 messages")));
  assert(response.metadata !== undefined && response.metadata !== null);
});