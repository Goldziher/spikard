it("should handle gRPC request: gRPC DEADLINE_EXCEEDED status 4", async () => {
	// Tests DEADLINE_EXCEEDED gRPC status code. Returned when the RPC does not complete within the specified time limit.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.TimeoutService",
		methodName: "SlowOp",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcGrpcDeadlineExceededStatus4(request);

	// Verify response
	expect(response.statusCode).toBe("DEADLINE_EXCEEDED");
	expect(response.metadata).toBeDefined();
});
