it("should handle gRPC request: gRPC INVALID_ARGUMENT status 3", async () => {
	// Tests INVALID_ARGUMENT gRPC status code. Indicates that the client provided an invalid or malformed argument.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.ArgService",
		methodName: "Validate",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcGrpcInvalidArgumentStatus3(request);

	// Verify response
	expect(response.statusCode).toBe("INVALID_ARGUMENT");
	expect(response.metadata).toBeDefined();
});
