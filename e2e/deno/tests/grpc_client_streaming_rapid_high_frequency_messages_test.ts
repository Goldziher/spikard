/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingRapidHighFrequencyMessages, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming - rapid high-frequency messages", async () => {
  // Tests client streaming RPC with rapid-fire message delivery. Server handles 50 messages in quick succession and returns aggregated metrics.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.MetricsService",
    methodName: "ProcessEvents",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingRapidHighFrequencyMessages(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ event_id: "rapid-batch-001", event_count: 50, min_value: 0.1, max_value: 5.0, avg_value: 2.55, throughput_mps: 500.0, status: "PROCESSED" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});