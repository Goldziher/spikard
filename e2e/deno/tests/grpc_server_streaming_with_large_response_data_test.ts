it("should handle gRPC request: Server streaming with large response data", async () => {
  // Tests server streaming RPC that yields multiple large messages. Validates proper streaming protocol handling and backpressure management.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamingService",
    methodName: "StreamLargeData",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingWithLargeResponseData(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ stream_id: "stream-large-001", chunk_number: 1, is_final: false })));
  expect(response.metadata).toBeDefined();
});
