/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingTransformToUppercase, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming - transform to uppercase", async () => {
  // Tests bidirectional streaming RPC where server transforms incoming messages to uppercase.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.TransformService",
    methodName: "Transform",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingTransformToUppercase(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assert(response.metadata !== undefined && response.metadata !== null);
});