/**
 * Parametrized tests for gRPC streaming fixtures.
 *
 * This module runs all fixtures from testing_data/protobuf/streaming/
 * as parametrized tests against the running gRPC server.
 *
 * Architecture:
 *     1. Fixtures are validated by scripts/validate_fixtures.py (schema enforcement)
 *     2. Fixtures are loaded during test discovery
 *     3. Tests are parametrized by fixture category (server/client/bidirectional/errors)
 *     4. GrpcTestClient executes RPCs against running server
 *     5. Responses are validated against expected_response in fixtures
 *
 * Adding new fixtures:
 *     - Add JSON file to testing_data/protobuf/streaming/{category}/
 *     - Run: task validate:fixtures
 *     - Tests automatically discover and run new fixtures
 *
 * Stream generation:
 *     - Fixtures with "stream_generator" are automatically expanded
 *     - See generateStream() for generation logic
 */

import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { afterAll, beforeAll, describe, expect, it } from "vitest";

import { GrpcTestClient } from "./grpc_test_client";

/**
 * Load all fixtures from a category directory.
 *
 * @param category - The fixture category name (e.g., 'server', 'client')
 * @returns List of tuples [fixture_name, fixture_data]
 */
function loadFixturesByCategory(category: string): Array<[string, Record<string, unknown>]> {
	const fixturesDir = join(__dirname, "../../../testing_data/protobuf/streaming");
	const categoryDir = join(fixturesDir, category);

	try {
		const files = readdirSync(categoryDir);
		const fixtures: Array<[string, Record<string, unknown>]> = [];

		for (const file of files.sort()) {
			if (!file.endsWith(".json")) {
				continue;
			}

			const content = readFileSync(join(categoryDir, file), "utf-8");
			const fixture = JSON.parse(content) as Record<string, unknown>;

			// Skip fixtures marked with "skip": true
			if (fixture.skip === true) {
				continue;
			}

			fixtures.push([fixture.name as string, fixture]);
		}

		return fixtures;
	} catch {
		return [];
	}
}

/**
 * Generate stream messages based on generator description.
 *
 * @param streamGenerator - Description of generation logic
 * @param streamSize - Number of messages to generate
 * @returns List of generated messages
 */
function generateStream(streamGenerator: string, streamSize: number): Array<Record<string, unknown>> {
	const lowerGen = streamGenerator.toLowerCase();

	if (lowerGen.includes("sequential") || lowerGen.includes("counter")) {
		// Generate sequential integer messages
		return Array.from({ length: streamSize }, (_, i) => ({
			index: i,
			value: `message_${i}`,
		}));
	}

	if (lowerGen.includes("random")) {
		// Generate messages with random data
		return Array.from({ length: streamSize }, (_, i) => ({
			index: i,
			random_value: Math.floor(Math.random() * 1000),
		}));
	}

	if (lowerGen.includes("timestamp")) {
		// Generate messages with timestamps
		return Array.from({ length: streamSize }, (_, i) => ({
			index: i,
			timestamp: Date.now() / 1000,
		}));
	}

	// Default: simple indexed messages
	return Array.from({ length: streamSize }, (_, i) => ({
		index: i,
		data: `item_${i}`,
	}));
}

/**
 * Extract service name, method name, and method definition from fixture.
 *
 * Fixtures are schema-validated, so we trust the structure exists.
 *
 * @param fixture - Fixture data (schema-validated)
 * @param streamingMode - Expected streaming mode (server_streaming, client_streaming, or undefined)
 * @returns Tuple of [service_name, method_name, method_definition]
 */
function extractServiceMethod(
	fixture: Record<string, unknown>,
	streamingMode?: string,
): [string, string, Record<string, unknown>] {
	const protobuf = fixture.protobuf as Record<string, unknown>;
	const services = protobuf.services as Array<Record<string, unknown>>;
	const service = services[0];
	const serviceName = service.name as string;

	const methods = service.methods as Array<Record<string, unknown>>;

	let method: Record<string, unknown>;
	if (streamingMode) {
		method =
			methods.find(
				(m) =>
					(streamingMode === "server_streaming" && m.server_streaming === true) ||
					(streamingMode === "client_streaming" && m.client_streaming === true),
			) || methods[0];
	} else {
		method = methods[0];
	}

	const methodName = method.name as string;

	return [serviceName, methodName, method];
}

/**
 * Extract and prepare request data from fixture.
 *
 * Handles both single messages and streams, including stream generation.
 *
 * @param fixture - Fixture data (schema-validated)
 * @param isStreaming - Whether this is a streaming request (client or bidirectional)
 * @returns Single message object or list of messages for streaming
 */
function extractRequestData(
	fixture: Record<string, unknown>,
	isStreaming: boolean = false,
): Record<string, unknown> | Array<Record<string, unknown>> {
	const request = fixture.request as Record<string, unknown>;

	if (!isStreaming) {
		// Server streaming or unary: single message
		return request.message as Record<string, unknown>;
	}

	// Client or bidirectional streaming: stream of messages
	if (request.stream) {
		return request.stream as Array<Record<string, unknown>>;
	}

	// Generate stream if using stream_generator
	if (request.stream_generator) {
		const streamGenerator = request.stream_generator as string;
		const streamSize = request.stream_size as number;
		return generateStream(streamGenerator, streamSize);
	}

	// Fallback: empty stream
	return [];
}

/**
 * Validate streaming response against expected response.
 *
 * @param responses - Actual response messages received
 * @param expectedResponse - Expected response from fixture
 * @throws AssertionError if responses don't match expectations
 */
function validateStreamResponse(
	responses: Array<Record<string, unknown>>,
	expectedResponse: Record<string, unknown>,
): void {
	const expectedMessages = expectedResponse.stream as Array<Record<string, unknown>> | undefined;

	if (expectedMessages === undefined) {
		// No specific stream expectations, just verify non-null
		expect(responses).toBeDefined();
		return;
	}

	// Validate stream length
	expect(responses).toHaveLength(expectedMessages.length);

	// Validate each message
	for (let i = 0; i < responses.length; i++) {
		expect(responses[i]).toEqual(expectedMessages[i]);
	}
}

/**
 * Validate single response message against expected response.
 *
 * @param response - Actual response message received
 * @param expectedResponse - Expected response from fixture
 * @throws AssertionError if response doesn't match expectations
 */
function validateSingleResponse(response: Record<string, unknown>, expectedResponse: Record<string, unknown>): void {
	const expectedMessage = expectedResponse.message;

	if (expectedMessage === undefined || expectedMessage === null) {
		// No specific message expectations
		expect(response).toBeDefined();
		return;
	}

	// Skip string descriptions (used for documentation)
	if (typeof expectedMessage === "string") {
		return;
	}

	// Validate message content
	expect(response).toEqual(expectedMessage);
}

/**
 * Validate gRPC error against expected error.
 *
 * @param error - Captured error from try/catch
 * @param expectedResponse - Expected response from fixture with error field
 * @throws AssertionError if error doesn't match expectations
 */
function validateErrorResponse(error: unknown, expectedResponse: Record<string, unknown>): void {
	const expectedError = expectedResponse.error as Record<string, unknown>;
	const expectedCode = expectedError.code as string | number | undefined;
	const expectedMessage = expectedError.message as string | undefined;

	// Cast error to check code (gRPC errors have code property)
	const grpcError = error as Record<string, unknown>;

	// Validate error code
	if (typeof expectedCode === "string") {
		expect(grpcError.code).toBe(expectedCode);
	} else if (typeof expectedCode === "number") {
		expect(grpcError.code).toBe(expectedCode);
	}

	// Validate error message if specified
	if (expectedMessage) {
		const errorDetails = String(grpcError.details || "");
		expect(errorDetails).toContain(expectedMessage);
	}
}

// Load fixtures by category
const serverStreamingFixtures = loadFixturesByCategory("server");
const clientStreamingFixtures = loadFixturesByCategory("client");
const bidirectionalFixtures = loadFixturesByCategory("bidirectional");
const errorFixtures = loadFixturesByCategory("errors");

describe("gRPC Server Streaming Fixtures", () => {
	let client: GrpcTestClient;

	beforeAll(() => {
		client = new GrpcTestClient("localhost:50051");
	});

	afterAll(() => {
		// Cleanup if needed
	});

	for (const [fixtureName, fixture] of serverStreamingFixtures) {
		it(`should pass fixture: ${fixtureName}`, async () => {
			// Extract service and method
			const [serviceName, methodName] = extractServiceMethod(fixture, "server_streaming");

			// Extract request data
			const requestMessage = extractRequestData(fixture, false);

			// Extract metadata and timeout
			const requestObj = fixture.request as Record<string, unknown>;
			const metadata = requestObj.metadata as Record<string, string> | undefined;
			const handler = fixture.handler as Record<string, unknown> | undefined;
			const timeoutMs = handler?.timeout_ms as number | undefined;

			// Execute RPC
			const responses = await client.executeServerStreaming(
				serviceName,
				methodName,
				requestMessage as Record<string, unknown>,
				metadata,
				timeoutMs ? timeoutMs / 1000 : undefined,
			);

			// Validate response
			const expectedResponse = fixture.expected_response as Record<string, unknown>;
			validateStreamResponse(responses, expectedResponse);
		});
	}
});

describe("gRPC Client Streaming Fixtures", () => {
	let client: GrpcTestClient;

	beforeAll(() => {
		client = new GrpcTestClient("localhost:50051");
	});

	afterAll(() => {
		// Cleanup if needed
	});

	for (const [fixtureName, fixture] of clientStreamingFixtures) {
		it(`should pass fixture: ${fixtureName}`, async () => {
			// Extract service and method
			const [serviceName, methodName] = extractServiceMethod(fixture, "client_streaming");

			// Extract request data (stream of messages)
			const requestMessages = extractRequestData(fixture, true);

			// Extract metadata and timeout
			const requestObj = fixture.request as Record<string, unknown>;
			const metadata = requestObj.metadata as Record<string, string> | undefined;
			const handler = fixture.handler as Record<string, unknown> | undefined;
			const timeoutMs = handler?.timeout_ms as number | undefined;

			// Execute RPC
			const response = await client.executeClientStreaming(
				serviceName,
				methodName,
				requestMessages as Array<Record<string, unknown>>,
				metadata,
				timeoutMs ? timeoutMs / 1000 : undefined,
			);

			// Validate response
			const expectedResponse = fixture.expected_response as Record<string, unknown>;
			validateSingleResponse(response, expectedResponse);
		});
	}
});

describe("gRPC Bidirectional Streaming Fixtures", () => {
	let client: GrpcTestClient;

	beforeAll(() => {
		client = new GrpcTestClient("localhost:50051");
	});

	afterAll(() => {
		// Cleanup if needed
	});

	for (const [fixtureName, fixture] of bidirectionalFixtures) {
		it(`should pass fixture: ${fixtureName}`, async () => {
			// Extract service and method
			const [serviceName, methodName] = extractServiceMethod(fixture);

			// Extract request data (stream of messages)
			const requestMessages = extractRequestData(fixture, true);

			// Extract metadata and timeout
			const requestObj = fixture.request as Record<string, unknown>;
			const metadata = requestObj.metadata as Record<string, string> | undefined;
			const handler = fixture.handler as Record<string, unknown> | undefined;
			const timeoutMs = handler?.timeout_ms as number | undefined;

			// Execute RPC
			const responses = await client.executeBidirectional(
				serviceName,
				methodName,
				requestMessages as Array<Record<string, unknown>>,
				metadata,
				timeoutMs ? timeoutMs / 1000 : undefined,
			);

			// Validate response
			const expectedResponse = fixture.expected_response as Record<string, unknown>;
			validateStreamResponse(responses, expectedResponse);
		});
	}
});

describe("gRPC Error Handling Fixtures", () => {
	let client: GrpcTestClient;

	beforeAll(() => {
		client = new GrpcTestClient("localhost:50051");
	});

	afterAll(() => {
		// Cleanup if needed
	});

	for (const [fixtureName, fixture] of errorFixtures) {
		it(`should pass fixture: ${fixtureName}`, async () => {
			// Extract service and method
			const [serviceName, methodName, method] = extractServiceMethod(fixture);

			// Determine streaming mode from method
			const isClientStreaming = (method.client_streaming as boolean) || false;
			const isServerStreaming = (method.server_streaming as boolean) || false;

			// Extract request data
			const isStreaming = isClientStreaming || (isClientStreaming && isServerStreaming);
			const requestData = extractRequestData(fixture, isStreaming);

			// Extract metadata and timeout
			const requestObj = fixture.request as Record<string, unknown>;
			const metadata = requestObj.metadata as Record<string, string> | undefined;
			const handler = fixture.handler as Record<string, unknown> | undefined;
			const timeoutMs = handler?.timeout_ms as number | undefined;

			// Execute RPC and expect error
			let error: unknown;

			try {
				if (isServerStreaming && !isClientStreaming) {
					// Server streaming
					await client.executeServerStreaming(
						serviceName,
						methodName,
						requestData as Record<string, unknown>,
						metadata,
						timeoutMs ? timeoutMs / 1000 : undefined,
					);
				} else if (isClientStreaming && !isServerStreaming) {
					// Client streaming
					await client.executeClientStreaming(
						serviceName,
						methodName,
						requestData as Array<Record<string, unknown>>,
						metadata,
						timeoutMs ? timeoutMs / 1000 : undefined,
					);
				} else {
					// Bidirectional or unary
					await client.executeBidirectional(
						serviceName,
						methodName,
						requestData as Array<Record<string, unknown>>,
						metadata,
						timeoutMs ? timeoutMs / 1000 : undefined,
					);
				}

				// If we get here, the call should have failed
				throw new Error("Expected gRPC error but none was raised");
			} catch (err) {
				error = err;
			}

			// Validate error
			const expectedResponse = fixture.expected_response as Record<string, unknown>;
			validateErrorResponse(error, expectedResponse);
		});
	}
});
