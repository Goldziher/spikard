<?php

declare(strict_types=1);

namespace Spikard\Tests;

use ReflectionMethod;
use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\JsonRpcMethodInfo;

final class RouteTestHelper
{
    /**
     * Register a basic route for tests without exposing the public API.
     */
    public static function withRoute(
        App $app,
        string $method,
        string $path,
        HandlerInterface $handler,
    ): App {
        return self::registerRoute($app, $method, $path, $handler, null, null, null, null);
    }

    /**
     * Register a schema-backed route for tests without exposing the public API.
     *
     * @param array<string, mixed>|null $requestSchema
     * @param array<string, mixed>|null $responseSchema
     * @param array<string, mixed>|null $parameterSchema
     */
    public static function withRouteWithSchemas(
        App $app,
        string $method,
        string $path,
        HandlerInterface $handler,
        ?array $requestSchema,
        ?array $responseSchema,
        ?array $parameterSchema,
    ): App {
        return self::registerRoute($app, $method, $path, $handler, $requestSchema, $responseSchema, $parameterSchema, null);
    }

    /**
     * Register a JSON-RPC route for tests without exposing the public API.
     *
     * @param array<string, mixed>|null $requestSchema
     * @param array<string, mixed>|null $responseSchema
     * @param array<string, mixed>|null $parameterSchema
     */
    public static function withJsonRpcRoute(
        App $app,
        string $method,
        string $path,
        HandlerInterface $handler,
        JsonRpcMethodInfo $jsonRpcMethod,
        ?array $requestSchema = null,
        ?array $responseSchema = null,
        ?array $parameterSchema = null,
    ): App {
        return self::registerRoute(
            $app,
            $method,
            $path,
            $handler,
            $requestSchema,
            $responseSchema,
            $parameterSchema,
            $jsonRpcMethod,
        );
    }

    /**
     * @param array<string, mixed>|null $requestSchema
     * @param array<string, mixed>|null $responseSchema
     * @param array<string, mixed>|null $parameterSchema
     */
    private static function registerRoute(
        App $app,
        string $method,
        string $path,
        HandlerInterface $handler,
        ?array $requestSchema,
        ?array $responseSchema,
        ?array $parameterSchema,
        ?JsonRpcMethodInfo $jsonRpcMethod,
    ): App {
        $refMethod = new ReflectionMethod($app, 'registerRoute');
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
}
