it("should handle gRPC request: Circuit breaker triggered - UNAVAILABLE with metadata", async () => {
  // Tests UNAVAILABLE status code with circuit breaker metadata. Indicates service degradation and when to retry.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.DownstreamService",
    methodName: "Query",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcCircuitBreakerTriggeredUnavailableWithMetadata(request);

  // Verify response
  expect(response.statusCode).toBe("UNAVAILABLE");
  expect(response.metadata).toBeDefined();
});
