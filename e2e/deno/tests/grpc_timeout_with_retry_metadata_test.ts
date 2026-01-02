it("should handle gRPC request: Timeout with retry metadata", async () => {
  // Tests DEADLINE_EXCEEDED status code with retry metadata in response trailers. Indicates whether client should retry.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.RetryService",
    methodName: "RetryableOperation",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcTimeoutWithRetryMetadata(request);

  // Verify response
  expect(response.statusCode).toBe("DEADLINE_EXCEEDED");
  expect(response.metadata).toBeDefined();
});
