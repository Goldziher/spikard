it("should handle gRPC request: gRPC OUT_OF_RANGE status 11", async () => {
	// Tests OUT_OF_RANGE gRPC status code. Returned when a value is outside the acceptable range.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.RangeService",
		methodName: "Check",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcGrpcOutOfRangeStatus11(request);

	// Verify response
	expect(response.statusCode).toBe("OUT_OF_RANGE");
	expect(response.metadata).toBeDefined();
});
