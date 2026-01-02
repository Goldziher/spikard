it("should handle gRPC request: gRPC compression test - gzip", async () => {
  // Tests gRPC payload compression using gzip. Validates that compressed messages are properly decompressed and that header metadata indicates compression.

  const metadata: Record<string, string> = {
    "grpc-encoding": "gzip",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.CompressionService",
    methodName: "SendCompressed",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcCompressionTestGzip(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ id: "compress-test-001", compressed: true })));
  expect(response.metadata).toBeDefined();
});
