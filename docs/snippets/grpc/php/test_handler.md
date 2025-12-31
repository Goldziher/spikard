# PHP gRPC Handler Tests

Comprehensive test examples for gRPC handlers using PHPUnit.

```php
<?php
// tests/UserServiceHandlerTest.php

declare(strict_types=1);

namespace Tests;

use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\MockObject\MockObject;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;
use Userservice\GetUserRequest;
use Userservice\CreateUserRequest;
use Userservice\User;

class UserServiceHandlerTest extends TestCase
{
    private UserServiceHandler $handler;
    private MockObject $userRepository;

    protected function setUp(): void
    {
        $this->userRepository = $this->createMock(UserRepository::class);
        $this->handler = new UserServiceHandler($this->userRepository);
    }

    public function testGetUserSuccess(): void
    {
        // Setup mock data
        $mockUser = new \stdClass();
        $mockUser->id = 1;
        $mockUser->name = 'Alice';
        $mockUser->email = 'alice@example.com';
        $mockUser->createdAt = new \DateTime();

        $this->userRepository
            ->expects($this->once())
            ->method('findById')
            ->with(1)
            ->willReturn($mockUser);

        // Create request
        $req = new GetUserRequest();
        $req->setId(1);

        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'GetUser',
            payload: $req->serializeToString()
        );

        // Call handler
        $response = $this->handler->handleRequest($grpcRequest);

        // Deserialize response
        $userResponse = new User();
        $userResponse->mergeFromString($response->payload);

        // Assertions
        $this->assertEquals(1, $userResponse->getId());
        $this->assertEquals('Alice', $userResponse->getName());
        $this->assertEquals('alice@example.com', $userResponse->getEmail());
        $this->assertEquals('true', $response->getMetadata('x-user-found'));
    }

    public function testGetUserNotFound(): void
    {
        $this->userRepository
            ->expects($this->once())
            ->method('findById')
            ->with(999)
            ->willReturn(null);

        // Create request for non-existent user
        $req = new GetUserRequest();
        $req->setId(999);

        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'GetUser',
            payload: $req->serializeToString()
        );

        // Call handler
        $response = $this->handler->handleRequest($grpcRequest);

        // Should return error response
        $this->assertTrue($response->isError());
        $this->assertStringContainsString('not found', $response->errorMessage);
    }

    public function testGetUserInvalidId(): void
    {
        $req = new GetUserRequest();
        $req->setId(0);

        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'GetUser',
            payload: $req->serializeToString()
        );

        $response = $this->handler->handleRequest($grpcRequest);

        $this->assertTrue($response->isError());
        $this->assertStringContainsString('must be positive', $response->errorMessage);
    }

    public function testCreateUserSuccess(): void
    {
        $mockUser = new \stdClass();
        $mockUser->id = 3;
        $mockUser->name = 'Charlie';
        $mockUser->email = 'charlie@example.com';

        $this->userRepository
            ->expects($this->once())
            ->method('create')
            ->willReturn($mockUser);

        // Create request
        $req = new CreateUserRequest();
        $req->setName('Charlie');
        $req->setEmail('charlie@example.com');

        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'CreateUser',
            payload: $req->serializeToString(),
            metadata: ['authorization' => 'Bearer valid-token']
        );

        // Call handler
        $response = $this->handler->handleRequest($grpcRequest);

        // Deserialize response
        $userResponse = new User();
        $userResponse->mergeFromString($response->payload);

        // Assertions
        $this->assertEquals(3, $userResponse->getId());
        $this->assertEquals('Charlie', $userResponse->getName());
        $this->assertEquals('charlie@example.com', $userResponse->getEmail());
        $this->assertEquals('3', $response->getMetadata('x-user-id'));
        $this->assertEquals('true', $response->getMetadata('x-created'));
    }

    public function testCreateUserValidationError(): void
    {
        // Create request with missing email
        $req = new CreateUserRequest();
        $req->setName('Test User');
        $req->setEmail('');

        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'CreateUser',
            payload: $req->serializeToString(),
            metadata: ['authorization' => 'Bearer token']
        );

        $response = $this->handler->handleRequest($grpcRequest);

        $this->assertTrue($response->isError());
        $this->assertStringContainsString('required', $response->errorMessage);
    }

    public function testCreateUserRequiresAuthentication(): void
    {
        $req = new CreateUserRequest();
        $req->setName('Test');
        $req->setEmail('test@example.com');

        // Request without authorization header
        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'CreateUser',
            payload: $req->serializeToString()
        );

        $response = $this->handler->handleRequest($grpcRequest);

        $this->assertTrue($response->isError());
        $this->assertStringContainsString('Authentication required', $response->errorMessage);
        $this->assertEquals('UNAUTHENTICATED', $response->getMetadata('grpc-status'));
    }

    public function testUnknownMethod(): void
    {
        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'DeleteUser',
            payload: ''
        );

        $response = $this->handler->handleRequest($grpcRequest);

        $this->assertTrue($response->isError());
        $this->assertStringContainsString('Unknown method', $response->errorMessage);
    }
}
```

## Test Patterns

### Using Data Providers

```php
<?php

class UserServiceHandlerTest extends TestCase
{
    /**
     * @dataProvider invalidUserIdProvider
     */
    public function testGetUserWithInvalidIds(int $invalidId, string $expectedError): void
    {
        $req = new GetUserRequest();
        $req->setId($invalidId);

        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'GetUser',
            payload: $req->serializeToString()
        );

        $response = $this->handler->handleRequest($grpcRequest);

        $this->assertTrue($response->isError());
        $this->assertStringContainsString($expectedError, $response->errorMessage);
    }

    public static function invalidUserIdProvider(): array
    {
        return [
            'zero id' => [0, 'must be positive'],
            'negative id' => [-1, 'must be positive'],
        ];
    }
}
```

### Testing Error Handling

```php
<?php

class UserServiceHandlerTest extends TestCase
{
    public function testHandlesMalformedPayload(): void
    {
        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'GetUser',
            payload: 'invalid protobuf data'
        );

        $response = $this->handler->handleRequest($grpcRequest);

        // Should return error, not throw exception
        $this->assertTrue($response->isError());
        $this->assertStringContainsString('Error', $response->errorMessage);
    }

    public function testHandlesRepositoryException(): void
    {
        $this->userRepository
            ->method('findById')
            ->willThrowException(new \RuntimeException('Database connection failed'));

        $req = new GetUserRequest();
        $req->setId(1);

        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'GetUser',
            payload: $req->serializeToString()
        );

        $response = $this->handler->handleRequest($grpcRequest);

        $this->assertTrue($response->isError());
        $this->assertStringContainsString('Database connection failed', $response->errorMessage);
    }
}
```

### Testing with Metadata

```php
<?php

class UserServiceHandlerTest extends TestCase
{
    public function testHandlerReadsCustomMetadata(): void
    {
        $mockUser = new \stdClass();
        $mockUser->id = 1;
        $mockUser->name = 'Alice';
        $mockUser->email = 'alice@example.com';
        $mockUser->createdAt = new \DateTime();

        $this->userRepository->method('findById')->willReturn($mockUser);

        $req = new GetUserRequest();
        $req->setId(1);

        $grpcRequest = new Request(
            serviceName: 'userservice.v1.UserService',
            methodName: 'GetUser',
            payload: $req->serializeToString(),
            metadata: [
                'x-request-id' => 'abc-123',
                'x-trace-id' => 'trace-456',
            ]
        );

        $response = $this->handler->handleRequest($grpcRequest);

        $this->assertFalse($response->isError());
    }
}
```

## Running Tests

```bash
# Run all tests
./vendor/bin/phpunit

# Run with verbose output
./vendor/bin/phpunit --verbose

# Run specific test file
./vendor/bin/phpunit tests/UserServiceHandlerTest.php

# Run specific test method
./vendor/bin/phpunit --filter testGetUserSuccess

# Run with coverage
./vendor/bin/phpunit --coverage-html coverage/
```
