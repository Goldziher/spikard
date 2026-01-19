/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingLargeBatch100Messages, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming - large batch 100 messages", async () => {
  // Tests client streaming RPC with 100 messages in the stream. Validates performance with large batch aggregation.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.BatchService",
    methodName: "ProcessBatch",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingLargeBatch100Messages(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ batch_id: "batch-large-001", total_items: 100, total_value: 5050, average_value: 50.5, status: "PROCESSED" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});