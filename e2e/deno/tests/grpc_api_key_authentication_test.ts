it("should handle gRPC request: API key authentication", async () => {
  // Tests API key authentication via gRPC metadata. Validates that API keys are properly validated and associated with clients.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
    "x-api-key": "sk_live_abc123def456",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ApiService",
    methodName: "FetchResource",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcApiKeyAuthentication(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ data: "resource_data", client_id: "client-api-001" })));
  expect(response.metadata).toBeDefined();
});
