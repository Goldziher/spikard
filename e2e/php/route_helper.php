<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\JsonRpcMethodInfo;

/**
 * Register a basic route for E2E fixtures using App internals.
 */
function register_route(App $app, string $method, string $path, HandlerInterface $handler): App
{
    return register_route_internal($app, $method, $path, $handler, null, null, null, null);
}

/**
 * Register a schema-backed route for E2E fixtures using App internals.
 *
 * @param array<string, mixed>|null $requestSchema
 * @param array<string, mixed>|null $responseSchema
 * @param array<string, mixed>|null $parameterSchema
 */
function register_route_with_schemas(
    App $app,
    string $method,
    string $path,
    HandlerInterface $handler,
    ?array $requestSchema,
    ?array $responseSchema,
    ?array $parameterSchema,
): App {
    return register_route_internal($app, $method, $path, $handler, $requestSchema, $responseSchema, $parameterSchema, null);
}

/**
 * @param array<string, mixed>|null $requestSchema
 * @param array<string, mixed>|null $responseSchema
 * @param array<string, mixed>|null $parameterSchema
 */
function register_route_internal(
    App $app,
    string $method,
    string $path,
    HandlerInterface $handler,
    ?array $requestSchema,
    ?array $responseSchema,
    ?array $parameterSchema,
    ?JsonRpcMethodInfo $jsonRpcMethod,
): App {
    $refMethod = new \ReflectionMethod($app, 'registerRoute');
    $refMethod->setAccessible(true);
    /** @var App $result */
    $result = $refMethod->invoke(
        $app,
        $method,
        $path,
        $handler,
        $requestSchema,
        $responseSchema,
        $parameterSchema,
        $jsonRpcMethod,
    );
    return $result;
}
