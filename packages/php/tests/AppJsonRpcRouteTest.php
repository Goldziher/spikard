<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use ReflectionClass;
use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\JsonRpcMethodInfo;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class AppJsonRpcRouteTest extends TestCase
{
    public function testAddJsonRpcRouteStoresMetadata(): void
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['ok' => true], 200, []);
            }

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };

        $methodInfo = new JsonRpcMethodInfo(
            methodName: 'math.add',
            description: 'Add two numbers',
            paramsSchema: ['type' => 'object'],
            resultSchema: ['type' => 'number'],
            deprecated: false,
            tags: ['math'],
        );

        $withRoute = $app->addJsonRpcRoute('POST', '/rpc', $handler, $methodInfo);

        $routesProperty = (new ReflectionClass($withRoute))->getProperty('routes');
        $routesProperty->setAccessible(true);
        /** @var mixed $routes */
        $routes = $routesProperty->getValue($withRoute);
        self::assertIsArray($routes);

        self::assertCount(1, $routes);
        /** @var mixed $route */
        $route = $routes[0];
        self::assertIsArray($route);
        self::assertSame('POST', $route['method']);
        self::assertSame('/rpc', $route['path']);
        self::assertSame($handler, $route['handler']);
        self::assertSame($methodInfo, $route['jsonrpc_method']);
        self::assertArrayHasKey('request_schema', $route);
        self::assertArrayHasKey('response_schema', $route);
        self::assertArrayHasKey('parameter_schema', $route);
    }
}
