it("should handle gRPC request: Special characters unicode and emoji in strings", async () => {
	// Tests handling of unicode characters, emojis, and special characters in protobuf string fields. Validates proper UTF-8 encoding/decoding.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.EchoService",
		methodName: "EchoSpecial",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcSpecialCharactersUnicodeAndEmojiInStrings(request);

	// Verify response
	expect(response.statusCode).toBe("OK");
	expect(response.payload).toEqual(Buffer.from(JSON.stringify({ echo: "Hello 世界 Привет שלום مرحبا" })));
	expect(response.metadata).toBeDefined();
});
