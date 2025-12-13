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
use Spikard\Http\JsonRpcMethodInfo;
use Spikard\Handlers\ClosureHandler;

// Create the application
$app = new App();

// Define JSON-RPC method metadata for math.add
$addInfo = new JsonRpcMethodInfo(
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
);

// Register JSON-RPC route for addition
$app = $app->addJsonRpcRoute(
    'POST',
    '/rpc',
    new ClosureHandler(function ($a, $b) {
        return $a + $b;
    }),
    $addInfo,
);

// Define JSON-RPC method metadata for math.subtract
$subtractInfo = new JsonRpcMethodInfo(
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
);

// Register JSON-RPC route for subtraction
$app = $app->addJsonRpcRoute(
    'POST',
    '/rpc',
    new ClosureHandler(function ($a, $b) {
        return $a - $b;
    }),
    $subtractInfo,
);

// Define JSON-RPC method metadata for math.multiply
$multiplyInfo = new JsonRpcMethodInfo(
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
);

// Register JSON-RPC route for multiplication
$app = $app->addJsonRpcRoute(
    'POST',
    '/rpc',
    new ClosureHandler(function ($a, $b) {
        return $a * $b;
    }),
    $multiplyInfo,
);

// Define JSON-RPC method metadata for user.create
$createUserInfo = new JsonRpcMethodInfo(
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
);

// Register JSON-RPC route for user creation
$app = $app->addJsonRpcRoute(
    'POST',
    '/rpc',
    new ClosureHandler(function ($email, $name) {
        return [
            'id' => random_int(1000, 9999),
            'email' => $email,
            'name' => $name,
            'created_at' => date('c'),
        ];
    }),
    $createUserInfo,
);

// Define JSON-RPC method metadata for user.getById
$getUserInfo = new JsonRpcMethodInfo(
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
);

// Register JSON-RPC route for getting user by ID
$app = $app->addJsonRpcRoute(
    'POST',
    '/rpc',
    new ClosureHandler(function ($id) {
        return [
            'id' => $id,
            'email' => "user{$id}@example.com",
            'name' => "User {$id}",
        ];
    }),
    $getUserInfo,
);

// Health check endpoint (non-JSON-RPC)
$app = $app->addRoute(
    'GET',
    '/health',
    new ClosureHandler(function () {
        return ['status' => 'healthy'];
    }),
);

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
