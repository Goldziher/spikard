it("should handle gRPC request: Validation error - INVALID_ARGUMENT with details", async () => {
	// Tests INVALID_ARGUMENT status code with detailed validation error information. Demonstrates how validation failures are communicated.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.ValidationService",
		methodName: "ValidateInput",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcValidationErrorInvalidArgumentWithDetails(request);

	// Verify response
	expect(response.statusCode).toBe("INVALID_ARGUMENT");
	expect(response.metadata).toBeDefined();
});
