it("should handle gRPC request: Bidirectional streaming RPC", async () => {
  // Tests bidirectional streaming where both client and server send multiple messages. Covers duplex communication patterns.

  const metadata: Record<string, string> = {
    "authorization": "Bearer user-token",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ChatService",
    methodName: "Chat",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingRpc(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.metadata).toBeDefined();
});
