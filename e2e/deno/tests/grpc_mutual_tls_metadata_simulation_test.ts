/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcMutualTlsMetadataSimulation, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Mutual TLS metadata simulation", async () => {
  // Tests mutual TLS authentication by validating client certificate metadata. Simulates mTLS handshake verification.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
    "x-client-cert-cn": "client.example.com",
    "x-client-cert-fingerprint": "AB:CD:EF:12:34:56:78:90",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.MtlsService",
    methodName: "VerifyClient",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcMutualTlsMetadataSimulation(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ verified: true, client_cn: "client.example.com" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});