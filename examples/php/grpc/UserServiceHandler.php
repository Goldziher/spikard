<?php

/**
 * Example gRPC handler for UserService.
 *
 * This example demonstrates how to implement a gRPC handler in PHP using Spikard.
 * It shows:
 * - Implementing the HandlerInterface
 * - Deserializing protobuf request payloads
 * - Processing business logic
 * - Serializing protobuf response payloads
 * - Handling errors appropriately
 *
 * @example
 * To use this handler:
 * 1. Define your protobuf messages in a .proto file
 * 2. Generate PHP classes using protobuf compiler
 * 3. Implement the handler as shown below
 * 4. Register the handler with Spikard
 *
 * ```php
 * use Spikard\Grpc;
 *
 * $service = Grpc::createService();
 * $service->registerHandler(
 *     'example.UserService',
 *     new UserServiceHandler($userRepository)
 * );
 * ```
 */

declare(strict_types=1);

namespace Example\Grpc;

use Spikard\Grpc\HandlerInterface;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;

/**
 * Handles gRPC requests for the UserService.
 *
 * This handler demonstrates a complete gRPC service implementation with:
 * - Input validation
 * - Error handling
 * - Metadata access
 * - Logging
 */
final class UserServiceHandler implements HandlerInterface
{
    /**
     * @param UserRepository $repository User data repository
     */
    public function __construct(
        private UserRepository $repository,
    ) {}

    /**
     * Handle a gRPC request for UserService.
     *
     * Supports the following methods:
     * - GetUser: Retrieve a single user by ID
     * - ListUsers: List all users with pagination
     * - CreateUser: Create a new user
     * - UpdateUser: Update an existing user
     * - DeleteUser: Delete a user
     *
     * @param Request $request The incoming gRPC request
     * @return Response The response with serialized payload
     */
    public function handleRequest(Request $request): Response
    {
        try {
            return match ($request->methodName) {
                'GetUser' => $this->handleGetUser($request),
                'ListUsers' => $this->handleListUsers($request),
                'CreateUser' => $this->handleCreateUser($request),
                'UpdateUser' => $this->handleUpdateUser($request),
                'DeleteUser' => $this->handleDeleteUser($request),
                default => Response::error("Unknown method: {$request->methodName}"),
            };
        } catch (\Exception $e) {
            return Response::error("Internal error: " . $e->getMessage());
        }
    }

    /**
     * Handle GetUser RPC.
     *
     * Request: GetUserRequest { int32 id = 1; }
     * Response: User { int32 id = 1; string name = 2; string email = 3; }
     */
    private function handleGetUser(Request $request): Response
    {
        try {
            // Deserialize request
            $getUserRequest = new \Example\GetUserRequest();
            $getUserRequest->mergeFromString($request->payload);

            // Validate input
            if ($getUserRequest->getId() <= 0) {
                return Response::error("Invalid user ID");
            }

            // Get user from repository
            $user = $this->repository->findById($getUserRequest->getId());
            if (!$user) {
                return Response::error("User not found");
            }

            // Serialize response
            return new Response(
                payload: $user->serializeToString(),
                metadata: [
                    'x-user-found' => 'true',
                ]
            );
        } catch (\InvalidArgumentException $e) {
            return Response::error("Invalid request: " . $e->getMessage());
        }
    }

    /**
     * Handle ListUsers RPC.
     *
     * Request: ListUsersRequest { int32 limit = 1; int32 offset = 2; }
     * Response: ListUsersResponse { repeated User users = 1; int32 total = 2; }
     */
    private function handleListUsers(Request $request): Response
    {
        try {
            // Deserialize request
            $listRequest = new \Example\ListUsersRequest();
            $listRequest->mergeFromString($request->payload);

            // Get pagination parameters
            $limit = max(1, $listRequest->getLimit());
            $offset = max(0, $listRequest->getOffset());

            // Get users from repository
            $users = $this->repository->findAll($limit, $offset);
            $total = $this->repository->count();

            // Build response
            $response = new \Example\ListUsersResponse();
            foreach ($users as $user) {
                $response->addUsers($user);
            }
            $response->setTotal($total);

            return new Response($response->serializeToString());
        } catch (\Exception $e) {
            return Response::error("Failed to list users: " . $e->getMessage());
        }
    }

    /**
     * Handle CreateUser RPC.
     *
     * Request: CreateUserRequest { string name = 1; string email = 2; }
     * Response: User { int32 id = 1; string name = 2; string email = 3; }
     */
    private function handleCreateUser(Request $request): Response
    {
        try {
            // Deserialize request
            $createRequest = new \Example\CreateUserRequest();
            $createRequest->mergeFromString($request->payload);

            // Validate input
            if (empty($createRequest->getName())) {
                return Response::error("Name is required");
            }
            if (empty($createRequest->getEmail())) {
                return Response::error("Email is required");
            }

            // Create user in repository
            $user = $this->repository->create(
                name: $createRequest->getName(),
                email: $createRequest->getEmail(),
            );

            return new Response(
                payload: $user->serializeToString(),
                metadata: [
                    'x-user-created' => 'true',
                    'x-user-id' => (string)$user->getId(),
                ]
            );
        } catch (\Exception $e) {
            return Response::error("Failed to create user: " . $e->getMessage());
        }
    }

    /**
     * Handle UpdateUser RPC.
     *
     * Request: UpdateUserRequest { int32 id = 1; string name = 2; string email = 3; }
     * Response: User { int32 id = 1; string name = 2; string email = 3; }
     */
    private function handleUpdateUser(Request $request): Response
    {
        try {
            // Deserialize request
            $updateRequest = new \Example\UpdateUserRequest();
            $updateRequest->mergeFromString($request->payload);

            // Validate input
            if ($updateRequest->getId() <= 0) {
                return Response::error("Invalid user ID");
            }

            // Update user in repository
            $user = $this->repository->update(
                id: $updateRequest->getId(),
                name: $updateRequest->getName(),
                email: $updateRequest->getEmail(),
            );

            if (!$user) {
                return Response::error("User not found");
            }

            return new Response($user->serializeToString());
        } catch (\Exception $e) {
            return Response::error("Failed to update user: " . $e->getMessage());
        }
    }

    /**
     * Handle DeleteUser RPC.
     *
     * Request: DeleteUserRequest { int32 id = 1; }
     * Response: google.protobuf.Empty
     */
    private function handleDeleteUser(Request $request): Response
    {
        try {
            // Deserialize request
            $deleteRequest = new \Example\DeleteUserRequest();
            $deleteRequest->mergeFromString($request->payload);

            // Validate input
            if ($deleteRequest->getId() <= 0) {
                return Response::error("Invalid user ID");
            }

            // Delete user from repository
            $deleted = $this->repository->delete($deleteRequest->getId());

            if (!$deleted) {
                return Response::error("User not found");
            }

            // Return empty response
            $empty = new \Google\Protobuf\GPBEmpty();
            return new Response($empty->serializeToString());
        } catch (\Exception $e) {
            return Response::error("Failed to delete user: " . $e->getMessage());
        }
    }
}

/**
 * Example user repository for demonstration.
 * In a real application, this would interact with a database.
 */
class UserRepository
{
    /** @var array<int, \Example\User> */
    private array $users = [];
    private int $nextId = 1;

    public function findById(int $id): ?\Example\User
    {
        return $this->users[$id] ?? null;
    }

    /**
     * @return \Example\User[]
     */
    public function findAll(int $limit, int $offset): array
    {
        return array_slice(
            array_values($this->users),
            $offset,
            $limit
        );
    }

    public function count(): int
    {
        return count($this->users);
    }

    public function create(string $name, string $email): \Example\User
    {
        $user = new \Example\User();
        $user->setId($this->nextId);
        $user->setName($name);
        $user->setEmail($email);

        $this->users[$this->nextId] = $user;
        $this->nextId++;

        return $user;
    }

    public function update(int $id, string $name, string $email): ?\Example\User
    {
        if (!isset($this->users[$id])) {
            return null;
        }

        $user = $this->users[$id];
        if (!empty($name)) {
            $user->setName($name);
        }
        if (!empty($email)) {
            $user->setEmail($email);
        }

        return $user;
    }

    public function delete(int $id): bool
    {
        if (!isset($this->users[$id])) {
            return false;
        }

        unset($this->users[$id]);
        return true;
    }
}
