<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class NativeRoutesMetadataTest extends TestCase
{
    public function test_native_routes_include_schemas_and_handler_name(): void
    {
        $app = (new App())
            ->addRouteWithSchemas(
                'GET',
                '/hello',
                new class implements HandlerInterface {
                    public function matches(Request $request): bool
                    {
                        return true;
                    }

                    public function handle(Request $request): Response
                    {
                        return Response::json(['ok' => true]);
                    }
                },
                ['type' => 'object'],
                ['type' => 'object'],
                ['type' => 'object'],
            );

        $routes = $app->nativeRoutes();
        $this->assertCount(1, $routes);

        $route = $routes[0];
        $this->assertSame('GET', $route['method']);
        $this->assertSame('/hello', $route['path']);
        $this->assertArrayHasKey('handler_name', $route);
        $this->assertSame(['type' => 'object'], $route['request_schema']);
        $this->assertSame(['type' => 'object'], $route['response_schema']);
        $this->assertSame(['type' => 'object'], $route['parameter_schema']);
    }
}
