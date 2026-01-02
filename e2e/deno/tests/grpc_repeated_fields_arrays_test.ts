it("should handle gRPC request: Repeated fields arrays", async () => {
  // Tests arrays/repeated fields for primitive types and messages. Covers repeated field serialization and deserialization.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.BlogService",
    methodName: "CreatePost",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcRepeatedFieldsArrays(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ id: 789, title: "Getting Started with gRPC", content: "This is a comprehensive guide to gRPC...", tags: [{ id: 1, name: "gRPC" }, { id: 2, name: "Protocol Buffers" }, { id: 3, name: "RPC" }], categories: ["tutorial", "programming", "networking"] })));
  expect(response.metadata).toBeDefined();
});
