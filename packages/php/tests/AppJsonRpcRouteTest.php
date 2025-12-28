<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use ReflectionClass;
use Spikard\App;
use Spikard\Attributes\JsonRpcMethod;
use Spikard\Attributes\Post;

final class AppJsonRpcRouteTest extends TestCase
{
    public function testRegisterControllerStoresJsonRpcMetadata(): void
    {
        $app = (new App())->registerController(new class () {
            /**
             * @return array<string, mixed>
             */
            #[Post('/rpc')]
            #[JsonRpcMethod(
                methodName: 'math.add',
                description: 'Add two numbers',
                paramsSchema: ['type' => 'object'],
                resultSchema: ['type' => 'number'],
                deprecated: false,
                tags: ['math'],
            )]
            public function add(): array
            {
                return ['ok' => true];
            }
        });

        $routesProperty = (new ReflectionClass($app))->getProperty('routes');
        $routesProperty->setAccessible(true);
        /**  */
        $routes = $routesProperty->getValue($app);
        self::assertIsArray($routes);

        self::assertCount(1, $routes);
        /** @var array<string, mixed> $route */
        $route = $routes[0];
        self::assertIsArray($route);
        self::assertSame('POST', $route['method']);
        self::assertSame('/rpc', $route['path']);
        self::assertArrayHasKey('handler', $route);
        self::assertArrayHasKey('jsonrpc_method', $route);
        self::assertIsObject($route['jsonrpc_method']);
        /** @var object{methodName: string} $jsonRpcMethod */
        $jsonRpcMethod = $route['jsonrpc_method'];
        self::assertSame('math.add', $jsonRpcMethod->methodName);
    }
}
