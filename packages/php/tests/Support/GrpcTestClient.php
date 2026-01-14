<?php

declare(strict_types=1);

namespace Spikard\Tests\Support;

use Exception;
use RuntimeException;

/**
 * Wrapper for gRPC call operations to provide proper type safety.
 * The gRPC PHP extension's call objects are not statically typed,
 * so we use method checks to safely invoke operations.
 */
final class GrpcCallWrapper
{
    /**
     */
    private object $call;

    /**
     * @param object $call The actual gRPC call object
     */
    public function __construct(object $call)
    {
        $this->call = $call;
    }

    /**
     * @param array<string, string> $metadata
     */
    public function sendMetadata(array $metadata): void
    {
        if (\method_exists($this->call, 'sendMetadata')) {
            $this->call->sendMetadata($metadata);
        }
    }

    /**
     */
    public function write(string $message): void
    {
        if (\method_exists($this->call, 'write')) {
            $this->call->write($message);
        }
    }

    public function writesDone(): void
    {
        if (\method_exists($this->call, 'writesDone')) {
            $this->call->writesDone();
        }
    }

    /**
     * Read a message from the call.
     *
     * @return array{0: string|null, 1: array<string, string>}
     */
    public function read(): array
    {
        if (\method_exists($this->call, 'read')) {
            /** @var array{0: string|null, 1: array<string, string>} */
            return $this->call->read();
        }
        return [null, []];
    }

    /**
     * Fetch final gRPC status when available.
     *
     * @return array{code: int, details: string, metadata: array<string, mixed>}
     */
    public function getStatus(): array
    {
        if (\method_exists($this->call, 'getStatus')) {
            /**  */
            $status = $this->call->getStatus();

            if (\is_array($status)) {
                return [
                    'code' => \is_numeric($status['code'] ?? null) ? (int) $status['code'] : 0,
                    'details' => \is_string($status['details'] ?? null) ? (string) $status['details'] : '',
                    'metadata' => \is_array($status['metadata'] ?? null) ? (array) $status['metadata'] : [],
                ];
            }

            if (\is_object($status)) {
                $code = 0;
                $details = '';
                $metadata = [];

                if (\method_exists($status, 'getCode')) {
                    $codeVal = $status->getCode();
                    if (\is_numeric($codeVal)) {
                        $code = (int) $codeVal;
                    }
                } elseif (\property_exists($status, 'code') && \is_numeric($status->code)) {
                    $code = (int) $status->code;
                }

                if (\method_exists($status, 'getDetails')) {
                    $detailsVal = $status->getDetails();
                    if (\is_string($detailsVal)) {
                        $details = $detailsVal;
                    }
                } elseif (\property_exists($status, 'details') && \is_string($status->details)) {
                    $details = $status->details;
                }

                if (\method_exists($status, 'getMetadata')) {
                    $metaVal = $status->getMetadata();
                    if (\is_array($metaVal)) {
                        $metadata = $metaVal;
                    }
                } elseif (\property_exists($status, 'metadata') && \is_array($status->metadata)) {
                    $metadata = $status->metadata;
                }

                return [
                    'code' => $code,
                    'details' => $details,
                    'metadata' => $metadata,
                ];
            }
        }

        return ['code' => 0, 'details' => '', 'metadata' => []];
    }
}

/**
 * gRPC Test Client for executing fixtures against running gRPC server.
 *
 * This class provides a wrapper for executing gRPC streaming fixtures
 * in integration tests with support for:
 * - All four streaming modes (unary, server, client, bidirectional)
 * - Metadata headers (authentication, tracing, etc.)
 * - Timeouts per request
 * - JSON-encoded messages (compatible with Spikard's gRPC implementation)
 *
 * Usage:
 *     $client = new GrpcTestClient('localhost:50051');
 *     $responses = $client->executeServerStreaming(
 *         'example.v1.StreamService',
 *         'GetStream',
 *         ['request_id' => 'test-001'],
 *         ['authorization' => 'Bearer token'],
 *         5.0
 *     );
 */
/**
 * Wrapper for gRPC channel to provide safe method access.
 */
final class GrpcChannelWrapper
{
    /**
     */
    private object $channel;

    /**
     */
    public function __construct(object $channel)
    {
        $this->channel = $channel;
    }

    /**
     * Create a call object for an RPC method.
     *
     * @param string $method Method path (e.g., "/service/Method")
     * @param float $timeout Timeout in seconds
     * @return object A call object
     */
    public function createCall(string $method, float $timeout): object
    {
        if (\method_exists($this->channel, 'createCall')) {
            return $this->channel->createCall($method, $timeout);
        }

        throw new RuntimeException('gRPC channel does not support createCall()');
    }
}

final class GrpcTestClient
{
    private string $serverAddress;

    /**
     */
    private ?GrpcChannelWrapper $channel = null;

    /**
     * Initialize gRPC test client.
     *
     * @param string $serverAddress Server address in format "host:port"
     */
    public function __construct(string $serverAddress = 'localhost:50051')
    {
        $this->serverAddress = $serverAddress;
    }

    private function grpcCodeName(int $code): string
    {
        $map = [
            0 => 'OK',
            1 => 'CANCELLED',
            2 => 'UNKNOWN',
            3 => 'INVALID_ARGUMENT',
            4 => 'DEADLINE_EXCEEDED',
            5 => 'NOT_FOUND',
            6 => 'ALREADY_EXISTS',
            7 => 'PERMISSION_DENIED',
            8 => 'RESOURCE_EXHAUSTED',
            9 => 'FAILED_PRECONDITION',
            10 => 'ABORTED',
            11 => 'OUT_OF_RANGE',
            12 => 'UNIMPLEMENTED',
            13 => 'INTERNAL',
            14 => 'UNAVAILABLE',
            15 => 'DATA_LOSS',
            16 => 'UNAUTHENTICATED',
        ];

        return $map[$code] ?? "UNKNOWN({$code})";
    }

    private function assertOkStatus(GrpcCallWrapper $call, string $context): void
    {
        $status = $call->getStatus();
        $code = $status['code'];
        $details = $status['details'];

        if ($code !== 0) {
            $codeName = $this->grpcCodeName($code);
            throw new RuntimeException(\sprintf('%s gRPC error %s (%d): %s', $context, $codeName, $code, $details));
        }
    }

    /**
     * Connect to the gRPC server.
     *
     * @throws RuntimeException If connection fails
     */
    public function connect(): void
    {
        if ($this->channel !== null) {
            return;
        }

        // Create insecure channel to the gRPC server
        // In production, you would use grpc_channel_create with credentials
        // For testing, we use a simple TCP connection wrapper
        $parts = \explode(':', $this->serverAddress);
        if (\count($parts) !== 2) {
            throw new RuntimeException(
                \sprintf('Invalid server address format: %s', $this->serverAddress)
            );
        }

        [$host, $port] = $parts;
        $portInt = (int) $port;

        // Verify gRPC extension is loaded
        if (!\extension_loaded('grpc')) {
            throw new RuntimeException(
                'gRPC PHP extension not loaded. Install with: pecl install grpc'
            );
        }

        // Create the channel using grpc_channel_create (gRPC PHP extension)
        // The gRPC PHP extension returns a resource-like object that we treat as GrpcChannelInterface for type safety
        if (\function_exists('grpc_channel_create')) {
            /** @var object|false $channelResult */
            $channelResult = grpc_channel_create($host . ':' . $portInt, []);
            if ($channelResult === false) {
                throw new RuntimeException(
                    \sprintf('Failed to create gRPC channel to %s', $this->serverAddress)
                );
            }
            $this->channel = new GrpcChannelWrapper($channelResult);
        } else {
            throw new RuntimeException(
                'grpc_channel_create function not available (gRPC PHP extension issue)'
            );
        }
    }

    /**
     * Close the connection to the gRPC server.
     */
    public function disconnect(): void
    {
        // Channels are automatically cleaned up in PHP
        $this->channel = null;
    }

    /**
     * Prepare metadata from dictionary to gRPC format.
     *
     * @param array<string, string> $metadata Metadata dictionary from fixture
     *
     * @return array<string, string>
     */
    private function prepareMetadata(array $metadata): array
    {
        // gRPC metadata is passed as array of string key => string value
        $prepared = [];
        foreach ($metadata as $key => $value) {
            $prepared[(string) $key] = (string) $value;
        }

        return $prepared;
    }

    /**
     * Execute unary RPC from fixture.
     *
     * @param string $serviceName Fully qualified service name (e.g., "example.v1.Service")
     * @param string $methodName Method name
     * @param array<string, mixed> $request Request data as array
     * @param array<string, string> $metadata Optional metadata headers
     * @param float|null $timeout Optional timeout in seconds
     *
     * @return array<string, mixed> Response data as array
     *
     * @throws RuntimeException If RPC fails
     */
    public function executeUnary(
        string $serviceName,
        string $methodName,
        array $request,
        array $metadata = [],
        ?float $timeout = null,
    ): array {
        $this->connect();

        if ($this->channel === null) {
            throw new RuntimeException('Channel not initialized');
        }

        try {
            $method = '/' . $serviceName . '/' . $methodName;
            $preparedMetadata = $this->prepareMetadata($metadata);

            // Serialize request as JSON
            $requestPayload = \json_encode($request, JSON_THROW_ON_ERROR);

            // Create a call object (gRPC PHP extension not statically typed)
            $rawCall = $this->channel->createCall($method, $timeout ?? 5.0);
            $call = new GrpcCallWrapper($rawCall);

            // Send request and receive response
            $call->sendMetadata($preparedMetadata);
            $call->write($requestPayload);
            $call->writesDone();

            [$responsePayload, $_metadata] = $call->read();

            if ($responsePayload === null) {
                $this->assertOkStatus($call, 'Unary RPC failed');
                throw new RuntimeException('No response received from server');
            }

            $this->assertOkStatus($call, 'Unary RPC failed');

            // Deserialize response
            /** @var array<string, mixed> */
            return \json_decode($responsePayload, true, 512, JSON_THROW_ON_ERROR);
        } catch (Exception $e) {
            throw new RuntimeException(
                \sprintf('Unary RPC failed: %s', $e->getMessage()),
                0,
                $e,
            );
        }
    }

    /**
     * Execute server streaming RPC from fixture.
     *
     * @param string $serviceName Fully qualified service name
     * @param string $methodName Method name
     * @param array<string, mixed> $request Request data as array
     * @param array<string, string> $metadata Optional metadata headers
     * @param float|null $timeout Optional timeout in seconds
     *
     * @return array<int, array<string, mixed>> List of response messages
     *
     * @throws RuntimeException If RPC fails
     */
    public function executeServerStreaming(
        string $serviceName,
        string $methodName,
        array $request,
        array $metadata = [],
        ?float $timeout = null,
    ): array {
        $this->connect();

        if ($this->channel === null) {
            throw new RuntimeException('Channel not initialized');
        }

        try {
            $method = '/' . $serviceName . '/' . $methodName;
            $preparedMetadata = $this->prepareMetadata($metadata);

            // Serialize request as JSON
            $requestPayload = \json_encode($request, JSON_THROW_ON_ERROR);

            // Create a call object (gRPC PHP extension not statically typed)
            $rawCall = $this->channel->createCall($method, $timeout ?? 5.0);
            $call = new GrpcCallWrapper($rawCall);

            // Send request and start reading responses
            $call->sendMetadata($preparedMetadata);
            $call->write($requestPayload);
            $call->writesDone();

            // Read all response messages
            $responses = [];
            while (true) {
                [$message, $_metadata] = $call->read();

                if ($message === null) {
                    break;
                }

                // Deserialize message
                /** @var array<string, mixed> */
                $decoded = \json_decode($message, true, 512, JSON_THROW_ON_ERROR);
                $responses[] = $decoded;
            }

            $this->assertOkStatus($call, 'Server streaming RPC failed');

            return $responses;
        } catch (Exception $e) {
            throw new RuntimeException(
                \sprintf('Server streaming RPC failed: %s', $e->getMessage()),
                0,
                $e,
            );
        }
    }

    /**
     * Execute client streaming RPC from fixture.
     *
     * @param string $serviceName Fully qualified service name
     * @param string $methodName Method name
     * @param array<int, array<string, mixed>> $requests List of request messages
     * @param array<string, string> $metadata Optional metadata headers
     * @param float|null $timeout Optional timeout in seconds
     *
     * @return array<string, mixed> Response data as array
     *
     * @throws RuntimeException If RPC fails
     */
    public function executeClientStreaming(
        string $serviceName,
        string $methodName,
        array $requests,
        array $metadata = [],
        ?float $timeout = null,
    ): array {
        $this->connect();

        if ($this->channel === null) {
            throw new RuntimeException('Channel not initialized');
        }

        try {
            $method = '/' . $serviceName . '/' . $methodName;
            $preparedMetadata = $this->prepareMetadata($metadata);

            // Create a call object (gRPC PHP extension not statically typed)
            $rawCall = $this->channel->createCall($method, $timeout ?? 5.0);
            $call = new GrpcCallWrapper($rawCall);

            // Send metadata and all request messages
            $call->sendMetadata($preparedMetadata);

            foreach ($requests as $request) {
                $payload = \json_encode($request, JSON_THROW_ON_ERROR);
                $call->write($payload);
            }

            $call->writesDone();

            // Read single response
            [$responsePayload, $_metadata] = $call->read();

            if ($responsePayload === null) {
                $this->assertOkStatus($call, 'Client streaming RPC failed');
                throw new RuntimeException('No response received from server');
            }

            $this->assertOkStatus($call, 'Client streaming RPC failed');

            // Deserialize response
            /** @var array<string, mixed> */
            return \json_decode($responsePayload, true, 512, JSON_THROW_ON_ERROR);
        } catch (Exception $e) {
            throw new RuntimeException(
                \sprintf('Client streaming RPC failed: %s', $e->getMessage()),
                0,
                $e,
            );
        }
    }

    /**
     * Execute bidirectional streaming RPC from fixture.
     *
     * @param string $serviceName Fully qualified service name
     * @param string $methodName Method name
     * @param array<int, array<string, mixed>> $requests List of request messages
     * @param array<string, string> $metadata Optional metadata headers
     * @param float|null $timeout Optional timeout in seconds
     *
     * @return array<int, array<string, mixed>> List of response messages
     *
     * @throws RuntimeException If RPC fails
     */
    public function executeBidirectional(
        string $serviceName,
        string $methodName,
        array $requests,
        array $metadata = [],
        ?float $timeout = null,
    ): array {
        $this->connect();

        if ($this->channel === null) {
            throw new RuntimeException('Channel not initialized');
        }

        try {
            $method = '/' . $serviceName . '/' . $methodName;
            $preparedMetadata = $this->prepareMetadata($metadata);

            // Create a call object (gRPC PHP extension not statically typed)
            $rawCall = $this->channel->createCall($method, $timeout ?? 5.0);
            $call = new GrpcCallWrapper($rawCall);

            // Send metadata
            $call->sendMetadata($preparedMetadata);

            // Send all request messages
            foreach ($requests as $request) {
                $payload = \json_encode($request, JSON_THROW_ON_ERROR);
                $call->write($payload);
            }

            $call->writesDone();

            // Read all response messages
            $responses = [];
            while (true) {
                [$message, $_metadata] = $call->read();

                if ($message === null) {
                    break;
                }

                // Deserialize message
                /** @var array<string, mixed> */
                $decoded = \json_decode($message, true, 512, JSON_THROW_ON_ERROR);
                $responses[] = $decoded;
            }

            $this->assertOkStatus($call, 'Bidirectional RPC failed');

            return $responses;
        } catch (Exception $e) {
            throw new RuntimeException(
                \sprintf('Bidirectional streaming RPC failed: %s', $e->getMessage()),
                0,
                $e,
            );
        }
    }

    /**
     * Destructor to ensure channel is cleaned up.
     */
    public function __destruct()
    {
        $this->disconnect();
    }
}
