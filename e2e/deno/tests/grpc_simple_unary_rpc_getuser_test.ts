it("should handle gRPC request: Simple unary RPC - GetUser", async () => {
  // Tests basic unary gRPC call with scalar types (int32, string). Covers fundamental request-response pattern.

  const metadata: Record<string, string> = {
    "authorization": "Bearer test-token",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.UserService",
    methodName: "GetUser",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcSimpleUnaryRpcGetuser(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ id: 123, name: "Alice Johnson", email: "alice@example.com" })));
  expect(response.metadata).toBeDefined();
});
