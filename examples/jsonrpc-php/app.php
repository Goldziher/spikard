<?php

declare(strict_types=1);

/**
 * JSON-RPC 2.0 Example Application
 *
 * This example demonstrates how to use Spikard with JSON-RPC 2.0 method metadata
 * to build RPC endpoints with automatic documentation support.
 *
 * Usage:
 *   php app.php
 *
 * Then invoke methods via curl:
 *   curl -X POST http://localhost:8000/rpc \
 *     -H "Content-Type: application/json" \
 *     -d '{"jsonrpc": "2.0", "method": "math.add", "params": {"a": 5, "b": 3}, "id": 1}'
 */

require_once __DIR__ . '/../../packages/php/src/Spikard.php';

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\JsonRpcMethod;
use Spikard\Attributes\Post;
use Spikard\Http\Request;

final class RpcController
{
    private function params(Request $request): array
    {
        if (!\is_array($request->body)) {
            return [];
        }
        $params = $request->body['params'] ?? [];
        return \is_array($params) ? $params : [];
    }

    #[Post('/rpc')]
    #[JsonRpcMethod(
        methodName: 'math.add',
        description: 'Add two numbers and return the result',
        paramsSchema: [
            'type' => 'object',
            'properties' => [
                'a' => ['type' => 'number'],
                'b' => ['type' => 'number'],
            ],
            'required' => ['a', 'b'],
        ],
        resultSchema: ['type' => 'number'],
        tags: ['math', 'arithmetic'],
    )]
    public function add(Request $request): int|float
    {
        $params = $this->params($request);
        return ($params['a'] ?? 0) + ($params['b'] ?? 0);
    }

    #[Post('/rpc')]
    #[JsonRpcMethod(
        methodName: 'math.subtract',
        description: 'Subtract two numbers and return the result',
        paramsSchema: [
            'type' => 'object',
            'properties' => [
                'a' => ['type' => 'number'],
                'b' => ['type' => 'number'],
            ],
            'required' => ['a', 'b'],
        ],
        resultSchema: ['type' => 'number'],
        tags: ['math', 'arithmetic'],
    )]
    public function subtract(Request $request): int|float
    {
        $params = $this->params($request);
        return ($params['a'] ?? 0) - ($params['b'] ?? 0);
    }

    #[Post('/rpc')]
    #[JsonRpcMethod(
        methodName: 'math.multiply',
        description: 'Multiply two numbers and return the result',
        paramsSchema: [
            'type' => 'object',
            'properties' => [
                'a' => ['type' => 'number'],
                'b' => ['type' => 'number'],
            ],
            'required' => ['a', 'b'],
        ],
        resultSchema: ['type' => 'number'],
        tags: ['math', 'arithmetic'],
    )]
    public function multiply(Request $request): int|float
    {
        $params = $this->params($request);
        return ($params['a'] ?? 0) * ($params['b'] ?? 0);
    }

    #[Post('/rpc')]
    #[JsonRpcMethod(
        methodName: 'user.create',
        description: 'Create a new user with email and name',
        paramsSchema: [
            'type' => 'object',
            'properties' => [
                'email' => [
                    'type' => 'string',
                    'format' => 'email',
                ],
                'name' => ['type' => 'string'],
            ],
            'required' => ['email', 'name'],
        ],
        resultSchema: [
            'type' => 'object',
            'properties' => [
                'id' => ['type' => 'integer'],
                'email' => ['type' => 'string'],
                'name' => ['type' => 'string'],
                'created_at' => ['type' => 'string', 'format' => 'date-time'],
            ],
        ],
        tags: ['users', 'admin'],
    )]
    public function createUser(Request $request): array
    {
        $params = $this->params($request);
        return [
            'id' => random_int(1000, 9999),
            'email' => $params['email'] ?? 'unknown@example.com',
            'name' => $params['name'] ?? 'Unknown',
            'created_at' => date('c'),
        ];
    }

    #[Post('/rpc')]
    #[JsonRpcMethod(
        methodName: 'user.getById',
        description: 'Get a user by their ID',
        paramsSchema: [
            'type' => 'object',
            'properties' => [
                'id' => ['type' => 'integer'],
            ],
            'required' => ['id'],
        ],
        resultSchema: [
            'type' => 'object',
            'properties' => [
                'id' => ['type' => 'integer'],
                'email' => ['type' => 'string'],
                'name' => ['type' => 'string'],
            ],
        ],
        tags: ['users'],
    )]
    public function getUser(Request $request): array
    {
        $params = $this->params($request);
        $id = $params['id'] ?? 0;
        return [
            'id' => $id,
            'email' => "user{$id}@example.com",
            'name' => "User {$id}",
        ];
    }
}

final class HealthController
{
    #[Get('/health')]
    public function health(): array
    {
        return ['status' => 'healthy'];
    }
}

// Create the application
$app = (new App())
    ->registerController(new RpcController())
    ->registerController(new HealthController());

// Run the application
echo "Starting Spikard JSON-RPC server on http://localhost:8000\n";
echo "Available JSON-RPC methods:\n";
echo "  - math.add\n";
echo "  - math.subtract\n";
echo "  - math.multiply\n";
echo "  - user.create\n";
echo "  - user.getById\n";
echo "\nPress Ctrl+C to stop\n\n";

$app->run();
