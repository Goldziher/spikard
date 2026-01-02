/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcOneofFieldHandling, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Oneof field handling", async () => {
  // Tests oneof fields where only one field in the group can be set at a time. Validates proper mutual exclusivity and serialization.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.OneofService",
    methodName: "ProcessOneof",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcOneofFieldHandling(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ received_type: "text_data", data_present: true })));
  assert(response.metadata !== undefined && response.metadata !== null);
});