it("should handle gRPC request: Mutual TLS metadata simulation", async () => {
  // Tests mutual TLS authentication by validating client certificate metadata. Simulates mTLS handshake verification.

  const metadata: Record<string, string> = {
    "x-client-cert-cn": "client.example.com",
    "x-client-cert-fingerprint": "AB:CD:EF:12:34:56:78:90",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.MtlsService",
    methodName: "VerifyClient",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcMutualTlsMetadataSimulation(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ verified: true, client_cn: "client.example.com" })));
  expect(response.metadata).toBeDefined();
});
