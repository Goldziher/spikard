/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingSingleMessageAggregation, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming - single message aggregation", async () => {
  // Tests client streaming RPC where client sends a single message. Server acknowledges and returns aggregated result.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.AggregateService",
    methodName: "AggregateData",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingSingleMessageAggregation(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ count: 1, total: 42, average: 42.0, status: "AGGREGATED" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});