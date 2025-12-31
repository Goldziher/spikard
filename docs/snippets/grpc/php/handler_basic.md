# PHP gRPC Handler

Complete PHP handler implementation for UserService with GetUser and CreateUser methods.

```php
<?php

declare(strict_types=1);

use Spikard\Grpc\HandlerInterface;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;
use Userservice\GetUserRequest;
use Userservice\CreateUserRequest;
use Userservice\User;

class UserServiceHandler implements HandlerInterface
{
    public function __construct(
        private UserRepository $userRepository,
    ) {}

    public function handleRequest(Request $request): Response
    {
        // Route based on method name
        return match ($request->methodName) {
            'GetUser' => $this->getUser($request),
            'CreateUser' => $this->createUser($request),
            default => Response::error("Unknown method: {$request->methodName}"),
        };
    }

    private function getUser(Request $request): Response
    {
        try {
            // 1. Deserialize request
            $req = new GetUserRequest();
            $req->mergeFromString($request->payload);

            // 2. Validate input
            if ($req->getId() <= 0) {
                return Response::error('User ID must be positive');
            }

            // 3. Business logic
            $user = $this->userRepository->findById($req->getId());
            if (!$user) {
                return Response::error("User {$req->getId()} not found");
            }

            // 4. Build response
            $responseUser = new User();
            $responseUser->setId($user->getId());
            $responseUser->setName($user->getName());
            $responseUser->setEmail($user->getEmail());
            $responseUser->setCreatedAt($user->getCreatedAt()->format('c'));

            // 5. Serialize and return
            return new Response(
                payload: $responseUser->serializeToString(),
                metadata: ['x-user-found' => 'true']
            );

        } catch (\Exception $e) {
            return Response::error("Error: {$e->getMessage()}");
        }
    }

    private function createUser(Request $request): Response
    {
        try {
            // 1. Deserialize request
            $req = new CreateUserRequest();
            $req->mergeFromString($request->payload);

            // 2. Validate input
            if (empty($req->getName()) || empty($req->getEmail())) {
                return Response::error('Name and email are required');
            }

            // 3. Check authorization from metadata
            $authToken = $request->getMetadata('authorization');
            if (!$authToken) {
                return Response::error(
                    'Authentication required',
                    ['grpc-status' => 'UNAUTHENTICATED']
                );
            }

            // 4. Business logic
            $user = $this->userRepository->create(
                name: $req->getName(),
                email: $req->getEmail()
            );

            // 5. Build response
            $responseUser = new User();
            $responseUser->setId($user->getId());
            $responseUser->setName($user->getName());
            $responseUser->setEmail($user->getEmail());
            $responseUser->setCreatedAt((new \DateTime())->format('c'));

            // 6. Serialize with metadata
            return new Response(
                payload: $responseUser->serializeToString(),
                metadata: [
                    'x-user-id' => (string)$user->getId(),
                    'x-created' => 'true',
                ]
            );

        } catch (\Exception $e) {
            return Response::error("Error: {$e->getMessage()}");
        }
    }
}
```

## Key Patterns

- **Synchronous**: PHP handlers are synchronous
- **`mergeFromString()`**: Deserializes binary protobuf (use merge, not parse)
- **`serializeToString()`**: Serializes protobuf to binary
- **Getters/Setters**: PHP protobuf uses getter/setter methods
- **Error responses**: Return `Response::error()` instead of throwing
- **Named arguments**: PHP 8.0+ named arguments for clarity
- **Type hints**: Leverage PHP type system for safety

## Registration

```php
<?php

declare(strict_types=1);

use Spikard\Grpc;

// Create service registry
$service = Grpc::createService();

// Register handler
$handler = new UserServiceHandler(
    userRepository: new UserRepository()
);

$service->registerHandler('userservice.UserService', $handler);

// Service ready to handle requests
```
