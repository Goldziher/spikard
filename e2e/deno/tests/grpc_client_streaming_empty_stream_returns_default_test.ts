/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingEmptyStreamReturnsDefault, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming - empty stream returns default", async () => {
  // Tests client streaming RPC where client sends no messages (empty stream). Server gracefully handles empty input and returns default response.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.OptionalService",
    methodName: "ProcessOptional",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingEmptyStreamReturnsDefault(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ request_id: "empty-stream-req", message_count: 0, result: "DEFAULT_RESULT", is_default: true })));
  assert(response.metadata !== undefined && response.metadata !== null);
});