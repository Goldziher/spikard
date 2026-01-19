<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;
use Spikard\Tests\Support\GrpcTestClient;

/**
 * Parametrized tests for gRPC streaming fixtures.
 *
 * This test suite runs all fixtures from testing_data/protobuf/streaming/
 * as parametrized tests against the running gRPC server.
 *
 * Architecture:
 *     1. Fixtures are validated by scripts/validate_fixtures.py (schema enforcement)
 *     2. Fixtures are loaded by this test class (discovery & parsing)
 *     3. Tests are parametrized by fixture category (server/client/bidirectional/errors)
 *     4. GrpcTestClient executes RPCs against running server
 *     5. Responses are validated against expected_response in fixtures
 *
 * Adding new fixtures:
 *     - Add JSON file to testing_data/protobuf/streaming/{category}/
 *     - Run: task validate:fixtures
 *     - Tests automatically discover and run new fixtures
 */
final class GrpcFixturesTest extends TestCase
{
    private const GRPC_CODE_MAP = [
        'OK' => 0,
        'CANCELLED' => 1,
        'UNKNOWN' => 2,
        'INVALID_ARGUMENT' => 3,
        'DEADLINE_EXCEEDED' => 4,
        'NOT_FOUND' => 5,
        'ALREADY_EXISTS' => 6,
        'PERMISSION_DENIED' => 7,
        'RESOURCE_EXHAUSTED' => 8,
        'FAILED_PRECONDITION' => 9,
        'ABORTED' => 10,
        'OUT_OF_RANGE' => 11,
        'UNIMPLEMENTED' => 12,
        'INTERNAL' => 13,
        'UNAVAILABLE' => 14,
        'DATA_LOSS' => 15,
        'UNAUTHENTICATED' => 16,
    ];
    private const FIXTURES_DIR = __DIR__ . '/../../../testing_data/protobuf/streaming';

    private GrpcTestClient $client;

    /**
     * Set up test client before each test.
     */
    protected function setUp(): void
    {
        parent::setUp();
        if (!\extension_loaded('grpc')) {
            $this->markTestSkipped('gRPC extension not loaded.');
        }
        $this->client = new GrpcTestClient('localhost:50051');
    }

    /**
     * Clean up test client after each test.
     */
    protected function tearDown(): void
    {
        $this->client->disconnect();
        parent::tearDown();
    }

    /**
     * Load all fixtures from a category directory.
     *
     * @param string $category The fixture category name (e.g., 'server', 'client')
     *
     * @return array<string, array<int, array<string, mixed>>>
     */
    private static function loadFixtures(string $category): array
    {
        $categoryDir = self::FIXTURES_DIR . '/' . $category;
        if (!\is_dir($categoryDir)) {
            return [];
        }

        $fixtures = [];
        $files = \glob($categoryDir . '/*.json');

        if ($files === false) {
            return [];
        }

        foreach ($files as $file) {
            try {
                $content = \file_get_contents($file);
                if ($content === false) {
                    continue;
                }

                /** @var array<string, mixed> $fixture */
                $fixture = \json_decode($content, true, 512, JSON_THROW_ON_ERROR);

                // Skip fixtures marked with "skip": true
                if (isset($fixture['skip']) && $fixture['skip'] === true) {
                    continue;
                }

                // Ensure fixture has a valid name
                /**  */
                $fixtureNameVal = $fixture['name'] ?? null;
                if (!\is_string($fixtureNameVal)) {
                    continue;
                }

                $fixtures[$fixtureNameVal] = [$fixture];
            } catch (\JsonException $e) {
                // Skip fixtures with JSON parse errors
                continue;
            }
        }

        \ksort($fixtures);

        return $fixtures;
    }

    /**
     * Generate stream messages based on generator description.
     *
     * @param string $streamGenerator Description of generation logic
     * @param int $streamSize Number of messages to generate
     *
     * @return array<int, array<string, mixed>>
     */
    private function generateStream(string $streamGenerator, int $streamSize): array
    {
        $generatorLower = \strtolower($streamGenerator);

        if (\str_contains($generatorLower, 'sequential') || \str_contains($generatorLower, 'counter')) {
            // Generate sequential integer messages
            $result = [];
            for ($i = 0; $i < $streamSize; ++$i) {
                $result[] = [
                    'index' => $i,
                    'value' => 'message_' . $i,
                ];
            }

            return $result;
        }

        if (\str_contains($generatorLower, 'random')) {
            // Generate messages with random data
            $result = [];
            for ($i = 0; $i < $streamSize; ++$i) {
                $result[] = [
                    'index' => $i,
                    'random_value' => \random_int(0, 1000),
                ];
            }

            return $result;
        }

        if (\str_contains($generatorLower, 'timestamp')) {
            // Generate messages with timestamps
            $result = [];
            for ($i = 0; $i < $streamSize; ++$i) {
                $result[] = [
                    'index' => $i,
                    'timestamp' => \microtime(true),
                ];
            }

            return $result;
        }

        // Default: simple indexed messages
        $result = [];
        for ($i = 0; $i < $streamSize; ++$i) {
            $result[] = [
                'index' => $i,
                'data' => 'item_' . $i,
            ];
        }

        return $result;
    }

    /**
     * Extract service name, method name, and method definition from fixture.
     *
     * @param array<string, mixed> $fixture Fixture data (schema-validated)
     * @param string|null $streamingMode Expected streaming mode (server_streaming, client_streaming, or null for any)
     *
     * @return array<int, string|array<string, mixed>>
     */
    private function extractServiceMethod(array $fixture, ?string $streamingMode = null): array
    {
        /** @var array<string, mixed> $protobuf */
        $protobuf = $fixture['protobuf'];

        // Build fully qualified service name: "package.ServiceName"
        /** @var string $package */
        $package = $protobuf['package'];

        /** @var array<int, array<string, mixed>> $services */
        $services = $protobuf['services'];

        /** @var array<string, mixed> $service */
        $service = $services[0];

        /** @var string $name */
        $name = $service['name'];
        $serviceName = $package . '.' . $name;

        /** @var array<int, array<string, mixed>> $methods */
        $methods = $service['methods'];

        // Find method matching streaming mode
        /** @var array<string, mixed> $method */
        $method = $methods[0];

        if ($streamingMode !== null) {
            foreach ($methods as $m) {
                if (isset($m[$streamingMode]) && $m[$streamingMode] === true) {
                    $method = $m;
                    break;
                }
            }
        }

        /** @var string $methodName */
        $methodName = $method['name'];

        return [$serviceName, $methodName, $method];
    }

    /**
     * Extract and prepare request data from fixture.
     *
     * @param array<string, mixed> $fixture Fixture data (schema-validated)
     * @param bool $isStreaming Whether this is a streaming request (client or bidirectional)
     *
     * @return array<string, mixed>|array<int, array<string, mixed>>
     */
    private function extractRequestData(array $fixture, bool $isStreaming = false): array
    {
        /** @var array<string, mixed> $request */
        $request = $fixture['request'];

        if (!$isStreaming) {
            // Server streaming or unary: single message
            if (isset($request['message'])) {
                /** @var array<string, mixed> $message */
                $message = $request['message'];
                return $message;
            }

            return [];
        }

        // Client or bidirectional streaming: stream of messages
        if (isset($request['stream'])) {
            /** @var array<int, array<string, mixed>> $stream */
            $stream = $request['stream'];
            return $stream;
        }

        // Generate stream if using stream_generator
        if (isset($request['stream_generator'])) {
            /** @var string $generator */
            $generator = $request['stream_generator'];

            /** @var int $size */
            $size = $request['stream_size'] ?? 5;

            return $this->generateStream($generator, $size);
        }

        // Fallback: empty stream
        return [];
    }

    /**
     * Validate streaming response against expected response.
     *
     * @param array<int, array<string, mixed>> $responses Actual response messages received
     * @param array<string, mixed> $expectedResponse Expected response from fixture
     */
    private function validateStreamResponse(array $responses, array $expectedResponse): void
    {
        /**  */
        $expectedMessages = $expectedResponse['stream'] ?? null;

        if ($expectedMessages !== null) {
            /** @var array<int, array<string, mixed>> $expectedMessages */

            // Validate stream length
            $this->assertCount(
                \count($expectedMessages),
                $responses,
                \sprintf(
                    'Expected %d messages, got %d',
                    \count($expectedMessages),
                    \count($responses),
                ),
            );

            // Validate each message
            foreach ($responses as $i => $actual) {
                /** @var array<string, mixed> $expectedMsg */
                $expectedMsg = $expectedMessages[$i];
                $this->assertEquals(
                    $expectedMsg,
                    $actual,
                    \sprintf('Message %d mismatch', $i),
                );
            }
            return;
        }

        /**  */
        $expectedCountValue = $expectedResponse['stream_count'] ?? null;

        if ($expectedCountValue !== null && \is_numeric($expectedCountValue)) {
            $expectedCount = (int) $expectedCountValue;
            $this->assertCount(
                $expectedCount,
                $responses,
                \sprintf('Expected %d messages, got %d', $expectedCount, \count($responses)),
            );

            /**  */
            $expectedSamples = $expectedResponse['stream_sample'] ?? null;
            if (\is_array($expectedSamples)) {
                foreach ($expectedSamples as $sample) {
                    if (!\is_array($sample)) {
                        continue;
                    }
                    $found = false;
                    foreach ($responses as $response) {
                        if ($response == $sample) {
                            $found = true;
                            break;
                        }
                    }
                    $this->assertTrue($found, 'Expected sample message not found in stream');
                }
            }
            return;
        }

    }

    private function grpcCodeFromName(mixed $statusCode): ?int
    {
        if (!\is_string($statusCode)) {
            return null;
        }

        $normalized = \strtoupper($statusCode);
        return self::GRPC_CODE_MAP[$normalized] ?? null;
    }

    /**
     * Validate single response message against expected response.
     *
     * @param array<string, mixed> $response Actual response message received
     * @param array<string, mixed> $expectedResponse Expected response from fixture
     */
    private function validateSingleResponse(array $response, array $expectedResponse): void
    {
        /**  */
        $expectedMessage = $expectedResponse['message'] ?? null;

        if ($expectedMessage === null) {
            // No specific message expectations
            return;
        }

        // Skip string descriptions (used for documentation)
        if (\is_string($expectedMessage)) {
            return;
        }

        // Validate message content
        $this->assertEquals($expectedMessage, $response, 'Response mismatch');
    }

    /**
     * Data provider for server streaming fixtures.
     *
     * @return array<string, array<int, mixed>>
     */
    public static function serverStreamingFixturesProvider(): array
    {
        return self::loadFixtures('server');
    }

    /**
     * Data provider for client streaming fixtures.
     *
     * @return array<string, array<int, mixed>>
     */
    public static function clientStreamingFixturesProvider(): array
    {
        return self::loadFixtures('client');
    }

    /**
     * Data provider for bidirectional streaming fixtures.
     *
     * @return array<string, array<int, mixed>>
     */
    public static function bidirectionalStreamingFixturesProvider(): array
    {
        return self::loadFixtures('bidirectional');
    }

    /**
     * Data provider for error handling fixtures.
     *
     * @return array<string, array<int, mixed>>
     */
    public static function errorFixturesProvider(): array
    {
        return self::loadFixtures('errors');
    }

    // ======================== Server Streaming Tests ========================

    /**
     * Test server streaming RPC against fixture.
     *
     * @param array<string, mixed> $fixture Fixture data (schema-validated)
     */
    #[Test]
    #[DataProvider('serverStreamingFixturesProvider')]
    public function testServerStreamingFixture(array $fixture): void
    {
        // Extract service and method
        [$serviceName, $methodName, $method] = $this->extractServiceMethod($fixture, 'server_streaming');
        \assert(\is_string($serviceName));
        \assert(\is_string($methodName));

        // Extract request data (return type ensures it's an array)
        $requestMessageMixed = $this->extractRequestData($fixture, isStreaming: false);
        /** @var array<string, mixed> $requestMessage */
        $requestMessage = $requestMessageMixed;

        // Extract metadata and timeout
        /**  */
        $request = $fixture['request'];
        \assert(\is_array($request));

        /** @var array<string, string> $metadata */
        $metadata = \is_array($request['metadata'] ?? null) ? (array) $request['metadata'] : [];

        /**  */
        $handler = $request['handler'] ?? [];
        \assert(\is_array($handler));

        /**  */
        $timeoutMs = $handler['timeout_ms'] ?? null;
        /** @var float|null $timeout */
        $timeout = $timeoutMs !== null && \is_numeric($timeoutMs) ? (float) $timeoutMs / 1000.0 : null;

        /**  */
        $expectedResponse = $fixture['expected_response'];
        /** @var array<string, mixed> $expectedResponseArray */
        $expectedResponseArray = \is_array($expectedResponse) ? $expectedResponse : [];
        $expectedError = $expectedResponseArray['error'] ?? null;
        $expectedStatusCode = $this->grpcCodeFromName($expectedResponseArray['status_code'] ?? null);
        $expectsError = \is_array($expectedError);

        if ($expectsError) {
            /** @var array{responses: array<int, array<string, mixed>>, status: array{code: int, details: string, metadata: array<string, mixed>}} $result */
            $result = $this->client->executeServerStreamingWithStatus(
                $serviceName,
                $methodName,
                $requestMessage,
                $metadata,
                $timeout,
            );
            $responses = $result['responses'];
            $status = $result['status'];

            $this->validateStreamResponse($responses, $expectedResponseArray);
            $expectedCode = null;
            if (\is_array($expectedError) && \is_numeric($expectedError['code'] ?? null)) {
                $expectedCode = (int) $expectedError['code'];
            } elseif (\is_int($expectedStatusCode)) {
                $expectedCode = $expectedStatusCode;
            }
            if (\is_int($expectedCode)) {
                $this->assertSame($expectedCode, $status['code']);
            }
            $expectedMessage = null;
            if (\is_array($expectedError)) {
                $expectedMessage = $expectedError['message'] ?? null;
            }
            if ($expectedMessage === null) {
                $expectedMessage = $expectedResponseArray['message'] ?? null;
            }
            if (\is_string($expectedMessage) && $expectedMessage !== '') {
                $this->assertStringContainsString($expectedMessage, $status['details']);
            }
            return;
        }

        $responses = $this->client->executeServerStreaming(
            $serviceName,
            $methodName,
            $requestMessage,
            $metadata,
            $timeout,
        );
        $this->validateStreamResponse($responses, $expectedResponseArray);
    }

    // ======================== Client Streaming Tests ========================

    /**
     * Test client streaming RPC against fixture.
     *
     * @param array<string, mixed> $fixture Fixture data (schema-validated)
     */
    #[Test]
    #[DataProvider('clientStreamingFixturesProvider')]
    public function testClientStreamingFixture(array $fixture): void
    {
        // Extract service and method
        [$serviceName, $methodName, $method] = $this->extractServiceMethod($fixture, 'client_streaming');
        \assert(\is_string($serviceName));
        \assert(\is_string($methodName));

        // Extract request data (stream of messages, return type ensures it's an array)
        $requestMessagesMixed = $this->extractRequestData($fixture, isStreaming: true);
        /** @var array<int, array<string, mixed>> $requestMessages */
        $requestMessages = $requestMessagesMixed;

        // Extract metadata and timeout
        /**  */
        $request = $fixture['request'];
        \assert(\is_array($request));

        /** @var array<string, string> $metadata */
        $metadata = \is_array($request['metadata'] ?? null) ? (array) $request['metadata'] : [];

        /**  */
        $handler = $request['handler'] ?? [];
        \assert(\is_array($handler));

        /**  */
        $timeoutMs = $handler['timeout_ms'] ?? null;
        /** @var float|null $timeout */
        $timeout = $timeoutMs !== null && \is_numeric($timeoutMs) ? (float) $timeoutMs / 1000.0 : null;

        // Execute RPC
        $response = $this->client->executeClientStreaming(
            $serviceName,
            $methodName,
            $requestMessages,
            $metadata,
            $timeout,
        );

        // Validate response
        /**  */
        $expectedResponse = $fixture['expected_response'];
        /** @var array<string, mixed> $expectedResponseArray */
        $expectedResponseArray = \is_array($expectedResponse) ? $expectedResponse : [];
        $this->validateSingleResponse($response, $expectedResponseArray);
    }

    // ======================== Bidirectional Streaming Tests ========================

    /**
     * Test bidirectional streaming RPC against fixture.
     *
     * @param array<string, mixed> $fixture Fixture data (schema-validated)
     */
    #[Test]
    #[DataProvider('bidirectionalStreamingFixturesProvider')]
    public function testBidirectionalFixture(array $fixture): void
    {
        // Extract service and method
        [$serviceName, $methodName, $method] = $this->extractServiceMethod($fixture);
        \assert(\is_string($serviceName));
        \assert(\is_string($methodName));

        // Extract request data (stream of messages, return type ensures it's an array)
        $requestMessagesMixed = $this->extractRequestData($fixture, isStreaming: true);
        /** @var array<int, array<string, mixed>> $requestMessages */
        $requestMessages = $requestMessagesMixed;

        // Extract metadata and timeout
        /**  */
        $request = $fixture['request'];
        \assert(\is_array($request));

        /** @var array<string, string> $metadata */
        $metadata = \is_array($request['metadata'] ?? null) ? (array) $request['metadata'] : [];

        /**  */
        $handler = $request['handler'] ?? [];
        \assert(\is_array($handler));

        /**  */
        $timeoutMs = $handler['timeout_ms'] ?? null;
        /** @var float|null $timeout */
        $timeout = $timeoutMs !== null && \is_numeric($timeoutMs) ? (float) $timeoutMs / 1000.0 : null;

        /**  */
        $expectedResponse = $fixture['expected_response'];
        /** @var array<string, mixed> $expectedResponseArray */
        $expectedResponseArray = \is_array($expectedResponse) ? $expectedResponse : [];
        $expectedError = $expectedResponseArray['error'] ?? null;
        $expectedStatusCode = $this->grpcCodeFromName($expectedResponseArray['status_code'] ?? null);
        $expectsError = \is_array($expectedError);

        if ($expectsError) {
            /** @var array{responses: array<int, array<string, mixed>>, status: array{code: int, details: string, metadata: array<string, mixed>}} $result */
            $result = $this->client->executeBidirectionalWithStatus(
                $serviceName,
                $methodName,
                $requestMessages,
                $metadata,
                $timeout,
            );
            $responses = $result['responses'];
            $status = $result['status'];

            $this->validateStreamResponse($responses, $expectedResponseArray);
            $expectedCode = null;
            if (\is_array($expectedError) && \is_numeric($expectedError['code'] ?? null)) {
                $expectedCode = (int) $expectedError['code'];
            } elseif (\is_int($expectedStatusCode)) {
                $expectedCode = $expectedStatusCode;
            }
            if (\is_int($expectedCode)) {
                $this->assertSame($expectedCode, $status['code']);
            }
            $expectedMessage = null;
            if (\is_array($expectedError)) {
                $expectedMessage = $expectedError['message'] ?? null;
            }
            if ($expectedMessage === null) {
                $expectedMessage = $expectedResponseArray['message'] ?? null;
            }
            if (\is_string($expectedMessage) && $expectedMessage !== '') {
                $this->assertStringContainsString($expectedMessage, $status['details']);
            }
            return;
        }

        $responses = $this->client->executeBidirectional(
            $serviceName,
            $methodName,
            $requestMessages,
            $metadata,
            $timeout,
        );
        $this->validateStreamResponse($responses, $expectedResponseArray);
    }

    // ======================== Error Handling Tests ========================

    /**
     * Test error cases from fixtures.
     *
     * @param array<string, mixed> $fixture Fixture data (schema-validated)
     */
    #[Test]
    #[DataProvider('errorFixturesProvider')]
    public function testErrorHandlingFixture(array $fixture): void
    {
        // Extract service and method
        [$serviceName, $methodName, $method] = $this->extractServiceMethod($fixture);
        \assert(\is_string($serviceName));
        \assert(\is_string($methodName));

        // Determine streaming mode from method
        /**  */
        $isClientStreamingVal = $method['client_streaming'] ?? false;
        $isClientStreaming = (bool) $isClientStreamingVal;

        /**  */
        $isServerStreamingVal = $method['server_streaming'] ?? false;
        $isServerStreaming = (bool) $isServerStreamingVal;

        // Extract request data
        $isStreaming = $isClientStreaming || $isServerStreaming;
        $requestDataMixed = $this->extractRequestData($fixture, isStreaming: $isStreaming);

        // Extract metadata and timeout
        /**  */
        $request = $fixture['request'];
        \assert(\is_array($request));

        /** @var array<string, string> $metadata */
        $metadata = \is_array($request['metadata'] ?? null) ? (array) $request['metadata'] : [];

        /**  */
        $handler = $request['handler'] ?? [];
        \assert(\is_array($handler));

        /**  */
        $timeoutMs = $handler['timeout_ms'] ?? null;
        /** @var float|null $timeout */
        $timeout = $timeoutMs !== null && \is_numeric($timeoutMs) ? (float) $timeoutMs / 1000.0 : null;

        // Execute RPC and expect error
        try {
            if ($isServerStreaming && !$isClientStreaming) {
                // Server streaming
                /** @var array<string, mixed> $requestData */
                $requestData = $requestDataMixed;
                $this->client->executeServerStreaming(
                    $serviceName,
                    $methodName,
                    $requestData,
                    $metadata,
                    $timeout,
                );
            } elseif ($isClientStreaming && !$isServerStreaming) {
                // Client streaming
                /** @var array<int, array<string, mixed>> $requestData */
                $requestData = $requestDataMixed;
                $this->client->executeClientStreaming(
                    $serviceName,
                    $methodName,
                    $requestData,
                    $metadata,
                    $timeout,
                );
            } else {
                // Bidirectional or unary
                /** @var array<int, array<string, mixed>> $requestData */
                $requestData = $requestDataMixed;
                $this->client->executeBidirectional(
                    $serviceName,
                    $methodName,
                    $requestData,
                    $metadata,
                    $timeout,
                );
            }

            // If we get here without an exception, the test should fail
            $this->fail('Expected RuntimeException to be thrown');
        } catch (\RuntimeException $e) {
            // Validate error
            /**  */
            $expectedResponse = $fixture['expected_response'];
            \assert(\is_array($expectedResponse));

            /**  */
            $error = $expectedResponse['error'] ?? null;
            if (\is_array($error)) {
                /**  */
                $code = $error['code'] ?? null;
                if ($code !== null && (\is_string($code) || \is_int($code))) {
                    $this->assertStringContainsString(
                        (string) $code,
                        $e->getMessage(),
                        'Error code mismatch',
                    );
                }

                /**  */
                $message = $error['message'] ?? null;
                if (\is_string($message)) {
                    $this->assertStringContainsString(
                        $message,
                        $e->getMessage(),
                        'Error message mismatch',
                    );
                }
            }
        }
    }
}
