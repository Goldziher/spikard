it("should handle gRPC request: Server streaming RPC", async () => {
	// Tests server streaming where the server sends multiple responses. Covers streaming response patterns.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.ItemService",
		methodName: "ListItems",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcServerStreamingRpc(request);

	// Verify response
	expect(response.statusCode).toBe("OK");
	expect(response.metadata).toBeDefined();
});
