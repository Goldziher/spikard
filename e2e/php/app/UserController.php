<?php

declare(strict_types=1);

namespace SpikardE2E;

use Spikard\Attributes\Delete;
use Spikard\Attributes\Get;
use Spikard\Attributes\Patch;
use Spikard\Attributes\Post;
use Spikard\Attributes\Put;
use Spikard\Http\Params\Body;
use Spikard\Http\Params\Query;
use Spikard\Http\Response;

/**
 * Example controller demonstrating route attributes.
 *
 * This controller showcases various HTTP methods with different parameter types:
 * - GET with query parameters
 * - POST with body
 * - PUT/PATCH with path parameters and body
 * - DELETE with path parameters
 *
 * Usage:
 * ```php
 * $app = new App();
 * $app = $app->registerController(UserController::class);
 * ```
 */
final class UserController
{
    /** @var array<string, array<string, mixed>> */
    private array $users = [];

    public function __construct()
    {
        // Initialize with some sample data
        $this->users = [
            '1' => ['id' => '1', 'name' => 'Alice', 'email' => 'alice@example.com', 'age' => 30],
            '2' => ['id' => '2', 'name' => 'Bob', 'email' => 'bob@example.com', 'age' => 25],
            '3' => ['id' => '3', 'name' => 'Charlie', 'email' => 'charlie@example.com', 'age' => 35],
        ];
    }

    /**
     * List all users with optional filtering.
     *
     * Query parameters:
     * - limit: Maximum number of users to return (default: 10)
     * - offset: Number of users to skip (default: 0)
     *
     * @return array<string, mixed>
     */
    #[Get('/users')]
    public function list(
        ?int $limit = 10,
        ?int $offset = 0
    ): array {
        $users = array_values($this->users);
        $total = count($users);

        $offset = $offset ?? 0;
        $limit = $limit ?? 10;

        $users = array_slice($users, $offset, $limit);

        return [
            'users' => $users,
            'total' => $total,
            'limit' => $limit,
            'offset' => $offset,
        ];
    }

    /**
     * Get a single user by ID.
     *
     * Path parameters:
     * - id: User ID
     *
     * @return array<string, mixed>|Response
     */
    #[Get('/users/:id')]
    public function get(string $id): array|Response
    {
        if (!isset($this->users[$id])) {
            return new Response(
                statusCode: 404,
                body: ['error' => 'User not found', 'id' => $id],
                headers: ['Content-Type' => 'application/json'],
            );
        }

        return ['user' => $this->users[$id]];
    }

    /**
     * Create a new user.
     *
     * Body should contain:
     * - name: User name (required)
     * - email: User email (required)
     * - age: User age (optional)
     *
     * Note: Parameters named 'data', 'body', or 'payload' automatically receive request body
     *
     * @param array<string, mixed> $data
     * @return array<string, mixed>|Response
     */
    #[Post('/users')]
    public function create(array $data = new Body()): array|Response
    {
        // Validate required fields
        if (!isset($data['name']) || !isset($data['email'])) {
            return new Response(
                statusCode: 400,
                body: [
                    'error' => 'Validation failed',
                    'message' => 'name and email are required',
                ],
                headers: ['Content-Type' => 'application/json'],
            );
        }

        // Generate new ID
        $newId = (string) (count($this->users) + 1);
        $user = [
            'id' => $newId,
            'name' => $data['name'],
            'email' => $data['email'],
            'age' => $data['age'] ?? null,
        ];

        $this->users[$newId] = $user;

        return [
            'user' => $user,
            'created' => true,
        ];
    }

    /**
     * Update a user (full replacement).
     *
     * Path parameters:
     * - id: User ID
     *
     * Body should contain:
     * - name: User name (required)
     * - email: User email (required)
     * - age: User age (optional)
     *
     * @param array<string, mixed> $data
     * @return array<string, mixed>|Response
     */
    #[Put('/users/:id')]
    public function update(string $id, array $data = new Body()): array|Response
    {
        if (!isset($this->users[$id])) {
            return new Response(
                statusCode: 404,
                body: ['error' => 'User not found', 'id' => $id],
                headers: ['Content-Type' => 'application/json'],
            );
        }

        // Validate required fields
        if (!isset($data['name']) || !isset($data['email'])) {
            return new Response(
                statusCode: 400,
                body: [
                    'error' => 'Validation failed',
                    'message' => 'name and email are required',
                ],
                headers: ['Content-Type' => 'application/json'],
            );
        }

        $user = [
            'id' => $id,
            'name' => $data['name'],
            'email' => $data['email'],
            'age' => $data['age'] ?? null,
        ];

        $this->users[$id] = $user;

        return [
            'user' => $user,
            'updated' => true,
        ];
    }

    /**
     * Partially update a user.
     *
     * Path parameters:
     * - id: User ID
     *
     * Body can contain any of:
     * - name: User name
     * - email: User email
     * - age: User age
     *
     * @param array<string, mixed> $data
     * @return array<string, mixed>|Response
     */
    #[Patch('/users/:id')]
    public function patch(string $id, array $data = new Body()): array|Response
    {
        if (!isset($this->users[$id])) {
            return new Response(
                statusCode: 404,
                body: ['error' => 'User not found', 'id' => $id],
                headers: ['Content-Type' => 'application/json'],
            );
        }

        // Merge with existing data
        $user = array_merge($this->users[$id], $data);
        $user['id'] = $id; // Ensure ID doesn't change

        $this->users[$id] = $user;

        return [
            'user' => $user,
            'updated' => true,
        ];
    }

    /**
     * Delete a user.
     *
     * Path parameters:
     * - id: User ID
     *
     * @return array<string, mixed>|Response
     */
    #[Delete('/users/:id')]
    public function delete(string $id): array|Response
    {
        if (!isset($this->users[$id])) {
            return new Response(
                statusCode: 404,
                body: ['error' => 'User not found', 'id' => $id],
                headers: ['Content-Type' => 'application/json'],
            );
        }

        unset($this->users[$id]);

        return [
            'deleted' => true,
            'id' => $id,
        ];
    }

    /**
     * Search users by name.
     *
     * Query parameters:
     * - q: Search query string
     *
     * @return array<string, mixed>
     */
    #[Get('/users/search')]
    public function search(?string $q = ''): array
    {
        $query = strtolower($q ?? '');
        $results = [];

        foreach ($this->users as $user) {
            if (str_contains(strtolower($user['name']), $query)) {
                $results[] = $user;
            }
        }

        return [
            'users' => $results,
            'query' => $q,
            'total' => count($results),
        ];
    }
}
