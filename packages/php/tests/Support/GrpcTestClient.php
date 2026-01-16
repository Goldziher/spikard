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
     * @return array<string, mixed>
     */
    private function normalizeMetadata(mixed $metadata): array
    {
        if (!\is_array($metadata)) {
            return [];
        }

        $normalized = [];
        foreach ($metadata as $key => $value) {
            $normalized[(string) $key] = $value;
        }

        return $normalized;
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
                    'metadata' => $this->normalizeMetadata($status['metadata'] ?? null),
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
                    $metadata = $this->normalizeMetadata($metaVal);
                } elseif (\property_exists($status, 'metadata')) {
                    $metadata = $this->normalizeMetadata($status->metadata);
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
    private string $target;

    /**
     */
    public function __construct(object $channel, string $target)
    {
        $this->channel = $channel;
        $this->target = $target;
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
            $reflection = new \ReflectionMethod($this->channel, 'createCall');
            $required = $reflection->getNumberOfRequiredParameters();
            $args = match (true) {
                $required <= 2 => [$method, $timeout],
                $required === 3 => [$method, $this->target, ['timeout' => $timeout]],
                $required === 4 => [$method, $this->target, ['timeout' => $timeout], []],
                default => [$method, $this->target, ['timeout' => $timeout], [], []],
            };
            $callResult = $reflection->invokeArgs($this->channel, $args);
            if (!\is_object($callResult)) {
                throw new RuntimeException('gRPC channel createCall() did not return an object');
            }
            return $callResult;
        }

        throw new RuntimeException('gRPC channel does not support createCall()');
    }
}

/**
 * JSON wrapper message for gRPC BaseStub calls.
 */
final class GrpcJsonMessage
{
    private string $payload;

    public function __construct(string $payload = '')
    {
        $this->payload = $payload;
    }

    public function serializeToString(): string
    {
        return $this->payload;
    }

    public function mergeFromString(string $payload): void
    {
        $this->payload = $payload;
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return self::normalizeDecoded($this->payload);
    }

    public function getPayload(): string
    {
        return $this->payload;
    }

    /**
     * @return array<string, mixed>
     */
    public static function decodeJson(string $payload): array
    {
        return self::normalizeDecoded($payload);
    }

    /**
     * @return array<string, mixed>
     */
    private static function normalizeDecoded(string $payload): array
    {
        $decoded = \json_decode($payload, true, 512, JSON_THROW_ON_ERROR);
        if (!\is_array($decoded)) {
            return [];
        }

        $normalized = [];
        foreach ($decoded as $key => $value) {
            $normalized[(string) $key] = $value;
        }

        return $normalized;
    }
}

/**
 * JSON-focused gRPC stub that reuses grpc/grpc BaseStub.
 */
final class GrpcJsonStub extends \Grpc\BaseStub
{
    /**
     * @param array<string, mixed> $metadata
     * @param array<string, mixed> $options
     * @return array{0: mixed, 1: mixed}
     */
    public function unaryCall(string $method, string $payload, array $metadata, array $options): array
    {
        return $this->_simpleRequest(
            $method,
            new GrpcJsonMessage($payload),
            [GrpcJsonMessage::class, 'decodeJson'],
            $metadata,
            $options,
        )->wait();
    }

    /**
     * @param array<string, mixed> $metadata
     * @param array<string, mixed> $options
     */
    public function serverStreamCall(string $method, string $payload, array $metadata, array $options): object
    {
        return $this->_serverStreamRequest(
            $method,
            new GrpcJsonMessage($payload),
            [GrpcJsonMessage::class, 'decodeJson'],
            $metadata,
            $options,
        );
    }

    /**
     * @param array<string, mixed> $metadata
     * @param array<string, mixed> $options
     */
    public function clientStreamCall(string $method, array $metadata, array $options): object
    {
        return $this->_clientStreamRequest(
            $method,
            [GrpcJsonMessage::class, 'decodeJson'],
            $metadata,
            $options,
        );
    }

    /**
     * @param array<string, mixed> $metadata
     * @param array<string, mixed> $options
     */
    public function bidiStreamCall(string $method, array $metadata, array $options): object
    {
        return $this->_bidiRequest(
            $method,
            [GrpcJsonMessage::class, 'decodeJson'],
            $metadata,
            $options,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public static function decodeJson(string $payload): array
    {
        /** @var array<string, mixed> $decoded */
        $decoded = \json_decode($payload, true, 512, JSON_THROW_ON_ERROR);
        return $decoded;
    }
}

final class GrpcTestClient
{
    private string $serverAddress;

    /**
     */
    private ?GrpcChannelWrapper $channel = null;
    private ?GrpcJsonStub $stub = null;

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

        // Create the channel using the available gRPC API
        if (\function_exists('grpc_channel_create')) {
            $options = [];
            /** @var object|false $channelResult */
            $channelResult = grpc_channel_create($host . ':' . $portInt, $options);
            if ($channelResult === false) {
                throw new RuntimeException(
                    \sprintf('Failed to create gRPC channel to %s', $this->serverAddress)
                );
            }

            if (\class_exists('\\Grpc\\BaseStub')) {
                $this->stub = $this->createStub($this->serverAddress, $options, $channelResult);
                $this->channel = null;
                return;
            }

            if (\method_exists($channelResult, 'createCall')) {
                $this->channel = new GrpcChannelWrapper($channelResult, $this->serverAddress);
                $this->stub = null;
                return;
            }
        }

        if (\class_exists('\\Grpc\\Channel')) {
            $options = [];
            if (\class_exists('\\Grpc\\ChannelCredentials')) {
                $options['credentials'] = \Grpc\ChannelCredentials::createInsecure();
            }

            /** @var \Grpc\Channel $channelResult */
            $channelResult = new \Grpc\Channel($host . ':' . $portInt, $options);

            if (\class_exists('\\Grpc\\BaseStub')) {
                $this->stub = $this->createStub($this->serverAddress, $options, $channelResult);
                $this->channel = null;
                return;
            }

            if (\method_exists($channelResult, 'createCall')) {
                $this->channel = new GrpcChannelWrapper($channelResult, $this->serverAddress);
                $this->stub = null;
                return;
            }
        }

        throw new RuntimeException(
            'No compatible gRPC channel API available (missing createCall and BaseStub fallback)'
        );
    }

    /**
     * Close the connection to the gRPC server.
     */
    public function disconnect(): void
    {
        // Channels are automatically cleaned up in PHP
        $this->channel = null;
        $this->stub = null;
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
     * @param array<string, string> $metadata
     * @return array<string, array<int, string>>
     */
    private function prepareStubMetadata(array $metadata): array
    {
        $prepared = [];
        foreach ($metadata as $key => $value) {
            $prepared[(string) $key] = [(string) $value];
        }

        return $prepared;
    }

    /**
     * @return array<string, mixed>
     */
    private function buildCallOptions(?float $timeout): array
    {
        if ($timeout === null) {
            return [];
        }

        $micros = (int) \max(0, \round($timeout * 1_000_000));
        return ['timeout' => $micros];
    }

    /**
     * @return array<string, mixed>
     */
    private function normalizeDecodedResponse(mixed $payload): array
    {
        if (\is_array($payload)) {
            if (\array_key_exists(0, $payload) && \array_key_exists(1, $payload)) {
                $payload = $payload[0];
            } else {
                return $this->normalizeResponseKeys($payload);
            }
        }

        if (\is_object($payload)) {
            if (\method_exists($payload, 'toArray')) {
                $decoded = $payload->toArray();
                if (\is_array($decoded)) {
                    return $this->normalizeResponseKeys($decoded);
                }
            }
            if (\method_exists($payload, 'getPayload')) {
                $payload = $payload->getPayload();
            } elseif (\method_exists($payload, 'serializeToString')) {
                $payload = $payload->serializeToString();
            }
        }

        if (\is_string($payload)) {
            /** @var array<string, mixed> $decoded */
            $decoded = \json_decode($payload, true, 512, JSON_THROW_ON_ERROR);
            return $this->normalizeResponseKeys($decoded);
        }

        throw new RuntimeException('Unexpected response payload type');
    }

    /**
     * @param array<mixed, mixed> $payload
     * @return array<string, mixed>
     */
    private function normalizeResponseKeys(array $payload): array
    {
        $normalized = [];
        foreach ($payload as $key => $value) {
            $normalized[(string) $key] = $value;
        }

        return $normalized;
    }

    /**
     * @return array<string, mixed>
     */
    private function normalizeMetadata(mixed $metadata): array
    {
        if (!\is_array($metadata)) {
            return [];
        }

        $normalized = [];
        foreach ($metadata as $key => $value) {
            $normalized[(string) $key] = $value;
        }

        return $normalized;
    }

    /**
     * @return array<int, array<string, mixed>>
     */
    private function collectStreamResponses(object $call): array
    {
        $responses = [];

        if (\method_exists($call, 'responses')) {
            $stream = $call->responses();
            if (\is_iterable($stream)) {
                foreach ($stream as $message) {
                    $responses[] = $this->normalizeDecodedResponse($message);
                }
                return $responses;
            }
        }

        if (\method_exists($call, 'read')) {
            while (true) {
                $message = $call->read();
                if ($message === null) {
                    break;
                }
                $responses[] = $this->normalizeDecodedResponse($message);
            }
        }

        return $responses;
    }

    private function assertStubStatusOk(mixed $status, string $context): void
    {
        if ($status === null) {
            return;
        }

        $normalized = $this->normalizeStubStatus($status);
        $this->assertNormalizedStatusOk($normalized, $context);
    }

    /**
     * @return array{code: int, details: string, metadata: array<string, mixed>}
     */
    private function normalizeStubStatus(mixed $status): array
    {
        $code = 0;
        $details = '';
        $metadata = [];

        if (\is_array($status)) {
            $codeVal = $status['code'] ?? 0;
            $code = \is_numeric($codeVal) ? (int) $codeVal : 0;
            $detailsVal = $status['details'] ?? '';
            $details = \is_string($detailsVal) ? $detailsVal : '';
            $metadata = $this->normalizeMetadata($status['metadata'] ?? null);
        } elseif (\is_object($status)) {
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
                $metadataVal = $status->getMetadata();
                $metadata = $this->normalizeMetadata($metadataVal);
            } elseif (\property_exists($status, 'metadata')) {
                $metadata = $this->normalizeMetadata($status->metadata);
            }
        }

        return [
            'code' => $code,
            'details' => $details,
            'metadata' => $metadata,
        ];
    }

    /**
     * @param array{code: int, details: string} $status
     */
    private function assertNormalizedStatusOk(array $status, string $context): void
    {
        $code = $status['code'];
        $details = $status['details'];

        if ($code !== 0) {
            $codeName = $this->grpcCodeName($code);
            throw new RuntimeException(\sprintf('%s gRPC error %s (%d): %s', $context, $codeName, $code, $details));
        }
    }

    /**
     * @param array<string, mixed> $options
     */
    private function createStub(string $target, array $options, ?object $channel = null): GrpcJsonStub
    {
        $reflection = new \ReflectionClass(GrpcJsonStub::class);
        $constructor = $reflection->getConstructor();
        $args = [$target, $options];

        if ($channel !== null && $constructor !== null && $constructor->getNumberOfParameters() >= 3) {
            $args[] = $channel;
        }

        /** @var GrpcJsonStub $stub */
        $stub = $reflection->newInstanceArgs($args);
        return $stub;
    }

    /**
     * @param array<string, mixed> $request
     * @param array<string, string> $metadata
     * @return array<string, mixed>
     */
    private function executeUnaryWithStub(
        string $serviceName,
        string $methodName,
        array $request,
        array $metadata,
        ?float $timeout,
    ): array {
        if ($this->stub === null) {
            throw new RuntimeException('Stub not initialized');
        }

        $method = '/' . $serviceName . '/' . $methodName;
        $preparedMetadata = $this->prepareStubMetadata($metadata);
        $options = $this->buildCallOptions($timeout);
        $requestPayload = \json_encode($request, JSON_THROW_ON_ERROR);

        [$response, $status] = $this->stub->unaryCall($method, $requestPayload, $preparedMetadata, $options);

        if ($response === null) {
            $this->assertStubStatusOk($status, 'Unary RPC failed');
            throw new RuntimeException('No response received from server');
        }

        $this->assertStubStatusOk($status, 'Unary RPC failed');
        return $this->normalizeDecodedResponse($response);
    }

    /**
     * @param array<string, mixed> $request
     * @param array<string, string> $metadata
     * @return array{responses: array<int, array<string, mixed>>, status: array{code: int, details: string, metadata: array<string, mixed>}}
     */
    private function executeServerStreamingWithStubStatus(
        string $serviceName,
        string $methodName,
        array $request,
        array $metadata,
        ?float $timeout,
    ): array {
        if ($this->stub === null) {
            throw new RuntimeException('Stub not initialized');
        }

        $method = '/' . $serviceName . '/' . $methodName;
        $preparedMetadata = $this->prepareStubMetadata($metadata);
        $options = $this->buildCallOptions($timeout);
        $requestPayload = \json_encode($request, JSON_THROW_ON_ERROR);

        $call = $this->stub->serverStreamCall($method, $requestPayload, $preparedMetadata, $options);
        $responses = $this->collectStreamResponses($call);
        $status = \method_exists($call, 'getStatus') ? $call->getStatus() : null;

        return [
            'responses' => $responses,
            'status' => $this->normalizeStubStatus($status),
        ];
    }

    /**
     * @param array<int, array<string, mixed>> $requests
     * @param array<string, string> $metadata
     * @return array<string, mixed>
     */
    private function executeClientStreamingWithStub(
        string $serviceName,
        string $methodName,
        array $requests,
        array $metadata,
        ?float $timeout,
    ): array {
        if ($this->stub === null) {
            throw new RuntimeException('Stub not initialized');
        }

        $method = '/' . $serviceName . '/' . $methodName;
        $preparedMetadata = $this->prepareStubMetadata($metadata);
        $options = $this->buildCallOptions($timeout);

        $call = $this->stub->clientStreamCall($method, $preparedMetadata, $options);

        foreach ($requests as $request) {
            $payload = \json_encode($request, JSON_THROW_ON_ERROR);
            if (\method_exists($call, 'write')) {
                $call->write(new GrpcJsonMessage($payload));
            }
        }

        if (\method_exists($call, 'writesDone')) {
            $call->writesDone();
        }

        $response = null;
        $status = null;

        if (\method_exists($call, 'wait')) {
            $waitResult = $call->wait();
            if (\is_array($waitResult)) {
                $response = $waitResult[0] ?? null;
                $status = $waitResult[1] ?? null;
            }
        } elseif (\method_exists($call, 'read')) {
            $response = $call->read();
            $status = \method_exists($call, 'getStatus') ? $call->getStatus() : null;
        }

        if ($response === null) {
            $this->assertStubStatusOk($status, 'Client streaming RPC failed');
            throw new RuntimeException('No response received from server');
        }

        $this->assertStubStatusOk($status, 'Client streaming RPC failed');
        return $this->normalizeDecodedResponse($response);
    }

    /**
     * @param array<int, array<string, mixed>> $requests
     * @param array<string, string> $metadata
     * @return array{responses: array<int, array<string, mixed>>, status: array{code: int, details: string, metadata: array<string, mixed>}}
     */
    private function executeBidirectionalWithStubStatus(
        string $serviceName,
        string $methodName,
        array $requests,
        array $metadata,
        ?float $timeout,
    ): array {
        if ($this->stub === null) {
            throw new RuntimeException('Stub not initialized');
        }

        $method = '/' . $serviceName . '/' . $methodName;
        $preparedMetadata = $this->prepareStubMetadata($metadata);
        $options = $this->buildCallOptions($timeout);

        $call = $this->stub->bidiStreamCall($method, $preparedMetadata, $options);

        foreach ($requests as $request) {
            $payload = \json_encode($request, JSON_THROW_ON_ERROR);
            if (\method_exists($call, 'write')) {
                $call->write(new GrpcJsonMessage($payload));
            }
        }

        if (\method_exists($call, 'writesDone')) {
            $call->writesDone();
        }

        $responses = $this->collectStreamResponses($call);
        $status = \method_exists($call, 'getStatus') ? $call->getStatus() : null;

        return [
            'responses' => $responses,
            'status' => $this->normalizeStubStatus($status),
        ];
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

        if ($this->stub !== null) {
            return $this->executeUnaryWithStub($serviceName, $methodName, $request, $metadata, $timeout);
        }

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
     * @return array{responses: array<int, array<string, mixed>>, status: array{code: int, details: string, metadata: array<string, mixed>}}
     *
     * @throws RuntimeException If RPC fails
     */
    public function executeServerStreamingWithStatus(
        string $serviceName,
        string $methodName,
        array $request,
        array $metadata = [],
        ?float $timeout = null,
    ): array {
        $this->connect();

        if ($this->stub !== null) {
            return $this->executeServerStreamingWithStubStatus($serviceName, $methodName, $request, $metadata, $timeout);
        }

        if ($this->channel === null) {
            throw new RuntimeException('Channel not initialized');
        }

        try {
            $method = '/' . $serviceName . '/' . $methodName;
            $preparedMetadata = $this->prepareMetadata($metadata);
            $requestPayload = \json_encode($request, JSON_THROW_ON_ERROR);

            $rawCall = $this->channel->createCall($method, $timeout ?? 5.0);
            $call = new GrpcCallWrapper($rawCall);

            $call->sendMetadata($preparedMetadata);
            $call->write($requestPayload);
            $call->writesDone();

            $responses = [];
            while (true) {
                [$message, $_metadata] = $call->read();
                if ($message === null) {
                    break;
                }
                /** @var array<string, mixed> */
                $decoded = \json_decode($message, true, 512, JSON_THROW_ON_ERROR);
                $responses[] = $decoded;
            }

            return [
                'responses' => $responses,
                'status' => $call->getStatus(),
            ];
        } catch (Exception $e) {
            throw new RuntimeException(
                \sprintf('Server streaming RPC failed: %s', $e->getMessage()),
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
        $result = $this->executeServerStreamingWithStatus($serviceName, $methodName, $request, $metadata, $timeout);
        $this->assertNormalizedStatusOk($result['status'], 'Server streaming RPC failed');
        return $result['responses'];
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

        if ($this->stub !== null) {
            return $this->executeClientStreamingWithStub($serviceName, $methodName, $requests, $metadata, $timeout);
        }

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
     * @return array{responses: array<int, array<string, mixed>>, status: array{code: int, details: string, metadata: array<string, mixed>}}
     *
     * @throws RuntimeException If RPC fails
     */
    public function executeBidirectionalWithStatus(
        string $serviceName,
        string $methodName,
        array $requests,
        array $metadata = [],
        ?float $timeout = null,
    ): array {
        $this->connect();

        if ($this->stub !== null) {
            return $this->executeBidirectionalWithStubStatus($serviceName, $methodName, $requests, $metadata, $timeout);
        }

        if ($this->channel === null) {
            throw new RuntimeException('Channel not initialized');
        }

        try {
            $method = '/' . $serviceName . '/' . $methodName;
            $preparedMetadata = $this->prepareMetadata($metadata);

            $rawCall = $this->channel->createCall($method, $timeout ?? 5.0);
            $call = new GrpcCallWrapper($rawCall);

            $call->sendMetadata($preparedMetadata);

            foreach ($requests as $request) {
                $payload = \json_encode($request, JSON_THROW_ON_ERROR);
                $call->write($payload);
            }

            $call->writesDone();

            $responses = [];
            while (true) {
                [$message, $_metadata] = $call->read();
                if ($message === null) {
                    break;
                }

                /** @var array<string, mixed> */
                $decoded = \json_decode($message, true, 512, JSON_THROW_ON_ERROR);
                $responses[] = $decoded;
            }

            return [
                'responses' => $responses,
                'status' => $call->getStatus(),
            ];
        } catch (Exception $e) {
            throw new RuntimeException(
                \sprintf('Bidirectional streaming RPC failed: %s', $e->getMessage()),
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
        $result = $this->executeBidirectionalWithStatus($serviceName, $methodName, $requests, $metadata, $timeout);
        $this->assertNormalizedStatusOk($result['status'], 'Bidirectional RPC failed');
        return $result['responses'];
    }

    /**
     * Destructor to ensure channel is cleaned up.
     */
    public function __destruct()
    {
        $this->disconnect();
    }
}
