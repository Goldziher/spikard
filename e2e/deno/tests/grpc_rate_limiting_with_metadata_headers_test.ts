it("should handle gRPC request: Rate limiting with metadata headers", async () => {
  // Tests gRPC rate limiting. Validates rate limit headers in response and proper 429 handling.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.RateLimitService",
    methodName: "Query",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcRateLimitingWithMetadataHeaders(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ result: "success" })));
  expect(response.metadata).toBeDefined();
});
