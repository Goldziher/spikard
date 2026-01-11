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
     * @var object
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
        if (method_exists($this->call, 'sendMetadata')) {
            $this->call->sendMetadata($metadata);
        }
    }

    /**
     * @param string $message
     */
    public function write(string $message): void
    {
        if (method_exists($this->call, 'write')) {
            $this->call->write($message);
        }
    }

    public function writesDone(): void
    {
        if (method_exists($this->call, 'writesDone')) {
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
        if (method_exists($this->call, 'read')) {
            /** @var array{0: string|null, 1: array<string, string>} */
            return $this->call->read();
        }
        return [null, []];
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
 * Interface for gRPC channel (from PHP gRPC extension).
 * This is a stub to provide type safety for the channel object.
 */
interface GrpcChannelInterface
{
    /**
     * Create a call object for an RPC method.
     *
     * @param string $method Method path (e.g., "/service/Method")
     * @param float $timeout Timeout in seconds
     * @return object A call object
     */
    public function createCall(string $method, float $timeout): object;
}

final class GrpcTestClient
{
    private string $serverAddress;

    /**
     * @var GrpcChannelInterface|null
     */
    private ?GrpcChannelInterface $channel = null;

    /**
     * Initialize gRPC test client.
     *
     * @param string $serverAddress Server address in format "host:port"
     */
    public function __construct(string $serverAddress = 'localhost:50051')
    {
        $this->serverAddress = $serverAddress;
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
        $parts = explode(':', $this->serverAddress);
        if (count($parts) !== 2) {
            throw new RuntimeException(
                sprintf('Invalid server address format: %s', $this->serverAddress)
            );
        }

        [$host, $port] = $parts;
        $portInt = (int) $port;

        // Verify gRPC extension is loaded
        if (!extension_loaded('grpc')) {
            throw new RuntimeException(
                'gRPC PHP extension not loaded. Install with: pecl install grpc'
            );
        }

        // Create the channel using grpc_channel_create (gRPC PHP extension)
        // The gRPC PHP extension returns a resource-like object that we treat as GrpcChannelInterface for type safety
        if (function_exists('grpc_channel_create')) {
            /** @var mixed $channelResult */
            $channelResult = grpc_channel_create($host . ':' . $portInt, []);
            if ($channelResult === false) {
                throw new RuntimeException(
                    sprintf('Failed to create gRPC channel to %s', $this->serverAddress)
                );
            }
            // Cast to GrpcChannelInterface for type safety (gRPC resource implements required methods)
            assert(is_object($channelResult) || is_resource($channelResult), 'gRPC channel must be object or resource');
            /** @var GrpcChannelInterface $channel */
            $channel = $channelResult instanceof GrpcChannelInterface ? $channelResult : (object) $channelResult;
            // We cast this assuming the gRPC extension's channel object is compatible
            $this->channel = $channel;
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
            $requestPayload = json_encode($request, JSON_THROW_ON_ERROR);

            // Create a call object (gRPC PHP extension not statically typed)
            $rawCall = $this->channel->createCall($method, $timeout ?? 5.0);
            $call = new GrpcCallWrapper($rawCall);

            // Send request and receive response
            $call->sendMetadata($preparedMetadata);
            $call->write($requestPayload);
            $call->writesDone();

            [$responsePayload, $_metadata] = $call->read();

            if ($responsePayload === null) {
                throw new RuntimeException('No response received from server');
            }

            // Deserialize response
            /** @var array<string, mixed> */
            return json_decode($responsePayload, true, 512, JSON_THROW_ON_ERROR);
        } catch (Exception $e) {
            throw new RuntimeException(
                sprintf('Unary RPC failed: %s', $e->getMessage()),
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
            $requestPayload = json_encode($request, JSON_THROW_ON_ERROR);

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
                $decoded = json_decode($message, true, 512, JSON_THROW_ON_ERROR);
                $responses[] = $decoded;
            }

            return $responses;
        } catch (Exception $e) {
            throw new RuntimeException(
                sprintf('Server streaming RPC failed: %s', $e->getMessage()),
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
                $payload = json_encode($request, JSON_THROW_ON_ERROR);
                $call->write($payload);
            }

            $call->writesDone();

            // Read single response
            [$responsePayload, $_metadata] = $call->read();

            if ($responsePayload === null) {
                throw new RuntimeException('No response received from server');
            }

            // Deserialize response
            /** @var array<string, mixed> */
            return json_decode($responsePayload, true, 512, JSON_THROW_ON_ERROR);
        } catch (Exception $e) {
            throw new RuntimeException(
                sprintf('Client streaming RPC failed: %s', $e->getMessage()),
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
                $payload = json_encode($request, JSON_THROW_ON_ERROR);
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
                $decoded = json_decode($message, true, 512, JSON_THROW_ON_ERROR);
                $responses[] = $decoded;
            }

            return $responses;
        } catch (Exception $e) {
            throw new RuntimeException(
                sprintf('Bidirectional streaming RPC failed: %s', $e->getMessage()),
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
