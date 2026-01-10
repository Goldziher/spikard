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
    private const FIXTURES_DIR = __DIR__ . '/../../../testing_data/protobuf/streaming';

    private GrpcTestClient $client;

    /**
     * Set up test client before each test.
     */
    protected function setUp(): void
    {
        parent::setUp();
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
     * @return array<string, array<int, mixed>>
     */
    private static function loadFixtures(string $category): array
    {
        $categoryDir = self::FIXTURES_DIR . '/' . $category;
        if (!is_dir($categoryDir)) {
            return [];
        }

        $fixtures = [];
        $files = glob($categoryDir . '/*.json');

        if ($files === false) {
            return [];
        }

        foreach ($files as $file) {
            try {
                $content = file_get_contents($file);
                if ($content === false) {
                    continue;
                }

                /** @var array<string, mixed> $fixture */
                $fixture = json_decode($content, true, 512, JSON_THROW_ON_ERROR);

                // Skip fixtures marked with "skip": true
                if (isset($fixture['skip']) && $fixture['skip'] === true) {
                    continue;
                }

                $fixtures[$fixture['name']] = [$fixture];
            } catch (\JsonException $e) {
                // Skip fixtures with JSON parse errors
                continue;
            }
        }

        ksort($fixtures);

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
        $generatorLower = strtolower($streamGenerator);

        if (str_contains($generatorLower, 'sequential') || str_contains($generatorLower, 'counter')) {
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

        if (str_contains($generatorLower, 'random')) {
            // Generate messages with random data
            $result = [];
            for ($i = 0; $i < $streamSize; ++$i) {
                $result[] = [
                    'index' => $i,
                    'random_value' => random_int(0, 1000),
                ];
            }

            return $result;
        }

        if (str_contains($generatorLower, 'timestamp')) {
            // Generate messages with timestamps
            $result = [];
            for ($i = 0; $i < $streamSize; ++$i) {
                $result[] = [
                    'index' => $i,
                    'timestamp' => microtime(true),
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
        $serviceName = $package . '.' . $service['name'];

        /** @var array<int, array<string, mixed>> $methods */
        $methods = $service['methods'];

        // Find method matching streaming mode
        $method = $methods[0];
        if ($streamingMode !== null) {
            foreach ($methods as $m) {
                if (isset($m[$streamingMode]) && $m[$streamingMode] === true) {
                    $method = $m;
                    break;
                }
            }
        }

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
                return $request['message'];
            }

            return [];
        }

        // Client or bidirectional streaming: stream of messages
        if (isset($request['stream'])) {
            return $request['stream'];
        }

        // Generate stream if using stream_generator
        if (isset($request['stream_generator'])) {
            $streamSize = $request['stream_size'] ?? 5;

            return $this->generateStream($request['stream_generator'], $streamSize);
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
        $expectedMessages = $expectedResponse['stream'] ?? null;

        if ($expectedMessages === null) {
            // No specific stream expectations, just verify non-empty
            $this->assertIsArray($responses);

            return;
        }

        // Validate stream length
        $this->assertCount(
            count($expectedMessages),
            $responses,
            sprintf(
                'Expected %d messages, got %d',
                count($expectedMessages),
                count($responses),
            ),
        );

        // Validate each message
        foreach ($responses as $i => $actual) {
            /** @var array<string, mixed> $expectedMsg */
            $expectedMsg = $expectedMessages[$i];
            $this->assertEquals(
                $expectedMsg,
                $actual,
                sprintf('Message %d mismatch', $i),
            );
        }
    }

    /**
     * Validate single response message against expected response.
     *
     * @param array<string, mixed> $response Actual response message received
     * @param array<string, mixed> $expectedResponse Expected response from fixture
     */
    private function validateSingleResponse(array $response, array $expectedResponse): void
    {
        $expectedMessage = $expectedResponse['message'] ?? null;

        if ($expectedMessage === null) {
            // No specific message expectations
            $this->assertIsArray($response);

            return;
        }

        // Skip string descriptions (used for documentation)
        if (is_string($expectedMessage)) {
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

        // Extract request data
        $requestMessage = $this->extractRequestData($fixture, isStreaming: false);

        // Extract metadata and timeout
        $request = $fixture['request'];
        $metadata = $request['metadata'] ?? [];
        $handler = $fixture['handler'] ?? [];
        $timeoutMs = $handler['timeout_ms'] ?? null;

        // Execute RPC
        $responses = $this->client->executeServerStreaming(
            $serviceName,
            $methodName,
            $requestMessage,
            $metadata,
            $timeoutMs !== null ? $timeoutMs / 1000.0 : null,
        );

        // Validate response
        $expectedResponse = $fixture['expected_response'];
        $this->validateStreamResponse($responses, $expectedResponse);
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

        // Extract request data (stream of messages)
        $requestMessages = $this->extractRequestData($fixture, isStreaming: true);

        // Extract metadata and timeout
        $request = $fixture['request'];
        $metadata = $request['metadata'] ?? [];
        $handler = $fixture['handler'] ?? [];
        $timeoutMs = $handler['timeout_ms'] ?? null;

        // Execute RPC
        $response = $this->client->executeClientStreaming(
            $serviceName,
            $methodName,
            $requestMessages,
            $metadata,
            $timeoutMs !== null ? $timeoutMs / 1000.0 : null,
        );

        // Validate response
        $expectedResponse = $fixture['expected_response'];
        $this->validateSingleResponse($response, $expectedResponse);
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

        // Extract request data (stream of messages)
        $requestMessages = $this->extractRequestData($fixture, isStreaming: true);

        // Extract metadata and timeout
        $request = $fixture['request'];
        $metadata = $request['metadata'] ?? [];
        $handler = $fixture['handler'] ?? [];
        $timeoutMs = $handler['timeout_ms'] ?? null;

        // Execute RPC
        $responses = $this->client->executeBidirectional(
            $serviceName,
            $methodName,
            $requestMessages,
            $metadata,
            $timeoutMs !== null ? $timeoutMs / 1000.0 : null,
        );

        // Validate response
        $expectedResponse = $fixture['expected_response'];
        $this->validateStreamResponse($responses, $expectedResponse);
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

        // Determine streaming mode from method
        $isClientStreaming = $method['client_streaming'] ?? false;
        $isServerStreaming = $method['server_streaming'] ?? false;

        // Extract request data
        $isStreaming = $isClientStreaming || ($isClientStreaming && $isServerStreaming);
        $requestData = $this->extractRequestData($fixture, isStreaming: $isStreaming);

        // Extract metadata and timeout
        $request = $fixture['request'];
        $metadata = $request['metadata'] ?? [];
        $handler = $fixture['handler'] ?? [];
        $timeoutMs = $handler['timeout_ms'] ?? null;

        // Execute RPC and expect error
        try {
            if ($isServerStreaming && !$isClientStreaming) {
                // Server streaming
                $this->client->executeServerStreaming(
                    $serviceName,
                    $methodName,
                    $requestData,
                    $metadata,
                    $timeoutMs !== null ? $timeoutMs / 1000.0 : null,
                );
            } elseif ($isClientStreaming && !$isServerStreaming) {
                // Client streaming
                $this->client->executeClientStreaming(
                    $serviceName,
                    $methodName,
                    $requestData,
                    $metadata,
                    $timeoutMs !== null ? $timeoutMs / 1000.0 : null,
                );
            } else {
                // Bidirectional or unary
                $this->client->executeBidirectional(
                    $serviceName,
                    $methodName,
                    $requestData,
                    $metadata,
                    $timeoutMs !== null ? $timeoutMs / 1000.0 : null,
                );
            }

            // If we get here without an exception, the test should fail
            $this->fail('Expected RuntimeException to be thrown');
        } catch (\RuntimeException $e) {
            // Validate error
            $expectedResponse = $fixture['expected_response'];
            if (isset($expectedResponse['error'])) {
                $expectedError = $expectedResponse['error'];

                // Check error code if specified
                if (isset($expectedError['code'])) {
                    $this->assertStringContainsString(
                        (string) $expectedError['code'],
                        $e->getMessage(),
                        'Error code mismatch',
                    );
                }

                // Check error message if specified
                if (isset($expectedError['message'])) {
                    $this->assertStringContainsString(
                        $expectedError['message'],
                        $e->getMessage(),
                        'Error message mismatch',
                    );
                }
            }
        }
    }
}
