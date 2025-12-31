# Spikard gRPC Binding for PHP

This document describes the PHP gRPC binding implementation for Spikard, enabling PHP code to implement gRPC handlers and connect to Spikard's gRPC runtime.

## Overview

The gRPC binding provides:

- **Request/Response Objects**: Type-safe representation of gRPC messages
- **Handler Interface**: Contract for implementing gRPC service handlers
- **Service Registry**: Manage multiple gRPC service handlers
- **Facade API**: Convenient static methods for common operations
- **Protocol Buffer Integration**: Works seamlessly with `google/protobuf` package

## Installation

The gRPC module is included with Spikard. Ensure you have the Spikard extension installed and the `google/protobuf` package:

```bash
composer require google/protobuf:^4.33
```

## Architecture

The PHP gRPC binding follows Spikard's language-agnostic handler pattern:

```
┌─────────────────────────────────────────────────────────┐
│  PHP gRPC Handler (implements HandlerInterface)         │
├─────────────────────────────────────────────────────────┤
│  1. Receives GrpcRequest with serialized protobuf       │
│  2. Deserializes using google/protobuf                  │
│  3. Processes business logic                            │
│  4. Serializes response to protobuf                     │
│  5. Returns GrpcResponse                                │
└─────────────────────────────────────────────────────────┘
                          ↑
                          │
                          │ (FFI Bridge)
                          │
┌─────────────────────────────────────────────────────────┐
│  Rust gRPC Runtime (Tonic)                              │
├─────────────────────────────────────────────────────────┤
│  - HTTP/2 multiplexing                                  │
│  - Compression support                                  │
│  - Metadata handling                                    │
│  - Stream management                                    │
└─────────────────────────────────────────────────────────┘
```

## Basic Usage

### 1. Create a Handler

Implement the `HandlerInterface` to handle gRPC requests:

```php
<?php

use Spikard\Grpc\HandlerInterface;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;

class UserServiceHandler implements HandlerInterface
{
    public function handleRequest(Request $request): Response
    {
        // Deserialize request using google/protobuf
        $getUserRequest = new \Example\GetUserRequest();
        $getUserRequest->mergeFromString($request->payload);

        // Process the request
        $userId = $getUserRequest->getId();
        $user = $this->getUserFromDatabase($userId);

        // Serialize response
        return new Response(
            payload: $user->serializeToString()
        );
    }

    private function getUserFromDatabase(int $id): \Example\User
    {
        // Your database logic here
        $user = new \Example\User();
        $user->setId($id);
        $user->setName('John Doe');
        return $user;
    }
}
```

### 2. Register Handlers

Use the Service registry to manage multiple handlers:

```php
<?php

use Spikard\Grpc;

// Create a service registry
$service = Grpc::createService();

// Register handlers
$service->registerHandler('example.UserService', new UserServiceHandler());
$service->registerHandler('example.PostService', new PostServiceHandler());
$service->registerHandler('example.CommentService', new CommentServiceHandler());

// Check registered services
echo "Registered services: " . implode(', ', $service->getServiceNames());
```

### 3. Handle Requests

The framework automatically routes requests to the appropriate handler:

```php
<?php

use Spikard\Grpc;

// Create a request
$request = Grpc::createRequest(
    serviceName: 'example.UserService',
    methodName: 'GetUser',
    payload: $serializedProtobufData,
    metadata: ['authorization' => 'Bearer token123']
);

// Get the handler and process
$handler = $service->getHandler('example.UserService');
if ($handler) {
    $response = $handler->handleRequest($request);
    // Response contains serialized protobuf payload and metadata
}
```

## API Reference

### Request Class

Represents an incoming gRPC request.

```php
final class Request
{
    public readonly string $serviceName;      // e.g., "example.UserService"
    public readonly string $methodName;       // e.g., "GetUser"
    public readonly string $payload;          // Binary protobuf data
    public readonly array $metadata;          // gRPC headers

    public function getMetadata(string $key): ?string;
    public function hasMetadata(string $key): bool;
    public function getPayloadSize(): int;
    public function getAllMetadata(): array;
}
```

### Response Class

Represents a gRPC response to be sent back.

```php
final class Response
{
    public readonly string $payload;          // Binary protobuf data
    public readonly array $metadata;          // Response headers

    public static function error(string $message, array $metadata = []): self;
    public function getPayloadSize(): int;
    public function getMetadata(string $key): ?string;
    public function hasMetadata(string $key): bool;
    public function getAllMetadata(): array;
}
```

### HandlerInterface

Contract for implementing gRPC service handlers.

```php
interface HandlerInterface
{
    /**
     * Handle a gRPC request and return a response.
     *
     * @param Request $request The incoming gRPC request
     * @return Response The response with serialized payload
     * @throws Exception If the request cannot be processed
     */
    public function handleRequest(Request $request): Response;
}
```

### Service Class

Registry for managing multiple gRPC handlers.

```php
final class Service
{
    public function registerHandler(string $serviceName, HandlerInterface $handler): self;
    public function getHandler(string $serviceName): ?HandlerInterface;
    public function hasHandler(string $serviceName): bool;
    public function getServiceNames(): array;
    public function getHandlerCount(): int;
    public function handleRequest(Request $request): Response;
    public function getAllHandlers(): array;
    public function clear(): self;
}
```

### Grpc Facade

Static helper methods for common operations.

```php
final class Grpc
{
    public static function createService(): Service;
    public static function createRequest(
        string $serviceName,
        string $methodName,
        string $payload,
        array $metadata = []
    ): Request;
    public static function createResponse(string $payload, array $metadata = []): Response;
    public static function createErrorResponse(string $message, array $metadata = []): Response;
}
```

## Advanced Usage

### Error Handling

Return error responses for exceptional cases:

```php
<?php

class UserServiceHandler implements HandlerInterface
{
    public function handleRequest(Request $request): Response
    {
        try {
            $req = new \Example\GetUserRequest();
            $req->mergeFromString($request->payload);

            if ($req->getId() <= 0) {
                return Response::error('Invalid user ID');
            }

            $user = $this->getUserFromDatabase($req->getId());
            if (!$user) {
                return Response::error('User not found');
            }

            return new Response($user->serializeToString());
        } catch (\Exception $e) {
            return Response::error('Internal server error: ' . $e->getMessage());
        }
    }
}
```

### Metadata Handling

Access and set gRPC metadata (headers):

```php
<?php

class AuthorizedServiceHandler implements HandlerInterface
{
    public function handleRequest(Request $request): Response
    {
        // Check authorization metadata
        $token = $request->getMetadata('authorization');
        if (!$token || !$this->validateToken($token)) {
            return Response::error(
                'Unauthorized',
                ['grpc-status' => 'UNAUTHENTICATED']
            );
        }

        // Extract user ID from token
        $userId = $this->extractUserIdFromToken($token);

        // Process request...
        $response = new Response($payload);

        // Add response metadata
        $response = new Response(
            $payload,
            ['x-user-id' => (string)$userId]
        );

        return $response;
    }
}
```

### Request ID Correlation

Implement request tracing with metadata:

```php
<?php

class TracedServiceHandler implements HandlerInterface
{
    public function handleRequest(Request $request): Response
    {
        // Get or generate request ID
        $requestId = $request->getMetadata('x-request-id') ?? bin2hex(random_bytes(8));

        // Log the request
        error_log("Processing request $requestId for {$request->methodName}");

        try {
            // Process the request
            $payload = $this->processRequest($request);

            // Return response with request ID
            return new Response($payload, ['x-request-id' => $requestId]);
        } catch (\Exception $e) {
            error_log("Error in request $requestId: " . $e->getMessage());
            throw $e;
        }
    }
}
```

### Dependency Injection

Work with Spikard's DI container:

```php
<?php

class UserServiceHandler implements HandlerInterface
{
    public function __construct(
        private UserRepository $userRepository,
        private Logger $logger,
    ) {}

    public function handleRequest(Request $request): Response
    {
        $this->logger->info('Processing gRPC request', [
            'service' => $request->serviceName,
            'method' => $request->methodName,
        ]);

        $req = new \Example\GetUserRequest();
        $req->mergeFromString($request->payload);

        $user = $this->userRepository->findById($req->getId());

        // ... process ...

        return new Response($user->serializeToString());
    }
}
```

## Protocol Buffer Integration

The gRPC binding works seamlessly with the `google/protobuf` package:

1. **Serialization**: Protobuf messages are serialized to binary strings
2. **Request Payload**: Contains the serialized protobuf request message
3. **Response Payload**: Should contain the serialized protobuf response message
4. **Metadata**: String key-value pairs (gRPC headers)

Example with generated protobuf classes:

```php
<?php

use Example\GetUserRequest;
use Example\User;

class UserServiceHandler implements HandlerInterface
{
    public function handleRequest(Request $request): Response
    {
        // Deserialize request
        $getUserRequest = new GetUserRequest();
        $getUserRequest->mergeFromString($request->payload);

        // Create response
        $user = new User();
        $user->setId($getUserRequest->getId());
        $user->setName('John Doe');
        $user->setEmail('john@example.com');

        // Serialize response
        return new Response($user->serializeToString());
    }
}
```

## Testing

Test your handlers using PHPUnit:

```php
<?php

use PHPUnit\Framework\TestCase;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;

class UserServiceHandlerTest extends TestCase
{
    private UserServiceHandler $handler;

    protected function setUp(): void
    {
        $this->handler = new UserServiceHandler();
    }

    public function testGetUser(): void
    {
        // Create a request
        $userRequest = new GetUserRequest();
        $userRequest->setId(123);

        $request = new Request(
            serviceName: 'example.UserService',
            methodName: 'GetUser',
            payload: $userRequest->serializeToString()
        );

        // Handle the request
        $response = $this->handler->handleRequest($request);

        // Verify the response
        $user = new User();
        $user->mergeFromString($response->payload);

        $this->assertSame(123, $user->getId());
        $this->assertSame('John Doe', $user->getName());
    }
}
```

## Best Practices

1. **Always Deserialize**: Use `mergeFromString()` to deserialize request payloads
2. **Always Serialize**: Use `serializeToString()` to serialize response payloads
3. **Handle Errors**: Return appropriate error responses for exceptional cases
4. **Validate Input**: Validate request data before processing
5. **Log Requests**: Log important request details for debugging
6. **Use Type Hints**: Leverage PHP type system for safety
7. **Test Handlers**: Write unit tests for all handler implementations
8. **Document API**: Document your gRPC services with comments

## Performance Considerations

- **Payload Size**: The gRPC runtime has configurable max message size (default 4MB)
- **Streaming**: Server-streaming RPCs are handled by the gRPC runtime
- **Async**: Handlers run in the same async context as other Spikard handlers
- **Metadata**: Metadata is passed as string key-value pairs; avoid large metadata values

## Troubleshooting

### Handler Not Called

- Ensure service name is fully qualified (contains a dot)
- Verify handler is registered before requests arrive
- Check service name matches exactly in request routing

### Serialization Errors

- Ensure payloads are valid protobuf binary data
- Use `mergeFromString()` not `parseFromString()`
- Check protobuf library version matches generated code

### Metadata Issues

- Metadata keys are case-sensitive
- Metadata values must be valid ASCII strings
- Use lowercase keys following gRPC conventions

## Compatibility

- **PHP**: 8.1+
- **Protobuf**: google/protobuf ^4.33
- **Spikard**: 0.7.5+

## See Also

- [gRPC Protocol Documentation](https://grpc.io/)
- [Protocol Buffers Documentation](https://developers.google.com/protocol-buffers)
- [google/protobuf PHP Package](https://packagist.org/packages/google/protobuf)
- [Spikard Documentation](../README.md)
