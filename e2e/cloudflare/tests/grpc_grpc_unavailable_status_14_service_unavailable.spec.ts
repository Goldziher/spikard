it("should handle gRPC request: gRPC UNAVAILABLE status 14 - service unavailable", async () => {
	// Tests UNAVAILABLE gRPC status code. Returned when the service is temporarily unavailable.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.UnavailService",
		methodName: "Request",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcGrpcUnavailableStatus14ServiceUnavailable(request);

	// Verify response
	expect(response.statusCode).toBe("UNAVAILABLE");
	expect(response.metadata).toBeDefined();
});
