/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcUserAgentAndClientInfoMetadata, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: User-Agent and client info metadata", async () => {
  // Tests User-Agent header handling and client identification. Validates proper user-agent parsing and logging.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
    "user-agent": "grpc-client/1.2.3 (linux; amd64)",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ClientService",
    methodName: "Identify",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcUserAgentAndClientInfoMetadata(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ client_type: "grpc-client", client_version: "1.2.3" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});