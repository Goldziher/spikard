<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use ReflectionProperty;
use RuntimeException;
use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Attributes\SchemaRef;
use Spikard\DI\ResolvedDependencies;
use Spikard\Handlers\ClosureHandler;
use Spikard\Http\JsonRpcMethodInfo;
use Spikard\Http\Params\Header;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

final class CoverageBoostTest extends TestCase
{
    public function testSchemaRefUsesRegistry(): void
    {
        $requestSchema = ['type' => 'object', 'title' => 'request'];
        $responseSchema = ['type' => 'object', 'title' => 'response'];
        $parameterSchema = ['type' => 'object', 'title' => 'params'];

        $app = (new App())
            ->withSchemas(
                ['req' => $requestSchema],
                ['resp' => $responseSchema],
                ['params' => $parameterSchema],
            )
            ->registerController(new class () {
                #[Post('/schema')]
                #[SchemaRef(request: 'req', response: 'resp', parameters: 'params')]
                public function create(): array
                {
                    return ['ok' => true];
                }
            });

        $routes = $app->routes();
        self::assertCount(1, $routes);
        $route = $routes[0];

        self::assertSame($requestSchema, $route['request_schema']);
        self::assertSame($responseSchema, $route['response_schema']);
        self::assertSame($parameterSchema, $route['parameter_schema']);
    }

    public function testRegisterControllerInfersHeaderSchema(): void
    {
        $app = (new App())->registerController(new class () {
            #[Get('/headers/{id}')]
            public function show(string $id, string $x_custom = new Header(alias: 'X-Custom-Header')): array
            {
                return ['id' => $id, 'header' => $x_custom];
            }
        });

        $routes = $app->routes();
        self::assertCount(1, $routes);
        $schema = $routes[0]['parameter_schema'];
        self::assertIsArray($schema);
        self::assertSame('object', $schema['type']);

        $properties = $schema['properties'];
        self::assertArrayHasKey('x-custom-header', $properties);
        self::assertSame('header', $properties['x-custom-header']['source']);
        self::assertSame('string', $properties['x-custom-header']['type']);
        self::assertArrayHasKey('id', $properties);
        self::assertSame('path', $properties['id']['source']);
    }

    public function testNativeConfigReturnsDefaults(): void
    {
        $payload = (new App())->nativeConfig();

        self::assertArrayHasKey('host', $payload);
        self::assertArrayHasKey('port', $payload);
        self::assertArrayHasKey('static_files', $payload);
        self::assertIsArray($payload['static_files']);
    }

    public function testResolvedDependenciesAccessors(): void
    {
        $deps = new ResolvedDependencies(['db' => 'sqlite']);

        self::assertSame('sqlite', $deps->get('db'));
        self::assertSame(['db' => 'sqlite'], $deps->all());
    }

    public function testResolvedDependenciesThrowsForMissingKey(): void
    {
        $deps = new ResolvedDependencies([]);

        $this->expectException(RuntimeException::class);
        $deps->get('missing');
    }

    public function testClosureHandlerWrapsPlainResponse(): void
    {
        $handler = new ClosureHandler(static fn (Request $request) => ['ok' => true]);
        $request = make_request(method: 'GET', path: '/closure', body: null);
        $response = $handler->handle($request);

        self::assertSame(['ok' => true], $response->body);
        self::assertSame('application/json', $response->headers['Content-Type']);
        self::assertTrue($handler->matches($request));
    }

    public function testClosureHandlerReturnsResponseDirectly(): void
    {
        $expected = Response::json(['status' => 'ok']);
        $handler = new ClosureHandler(static fn (Request $request) => $expected);
        $request = make_request(method: 'GET', path: '/closure', body: null);

        self::assertSame($expected, $handler->handle($request));
        self::assertSame($expected, $handler($request));
    }

    public function testJsonRpcMethodInfoFromArray(): void
    {
        $info = JsonRpcMethodInfo::fromArray([
            'method_name' => 'math.add',
            'description' => 'Add numbers',
            'params_schema' => ['type' => 'object'],
            'result_schema' => ['type' => 'number'],
            'deprecated' => true,
            'tags' => ['math'],
        ]);

        self::assertSame('math.add', $info->methodName);
        self::assertSame(['type' => 'object'], $info->paramsSchema);
        self::assertSame(['type' => 'number'], $info->resultSchema);
        self::assertTrue($info->deprecated);
        self::assertSame(['math'], $info->tags);

        $serialized = $info->toArray();
        self::assertSame('math.add', $serialized['method_name']);
        self::assertSame(true, $serialized['deprecated']);
    }

    public function testTestClientNativeOnlyMethodsThrowWhenForcedPhp(): void
    {
        $client = TestClient::create(new App());

        $this->expectException(RuntimeException::class);
        $client->connectWebSocket('/ws');
    }

    public function testTestClientSseThrowsWhenForcedPhp(): void
    {
        $client = TestClient::create(new App());

        $this->expectException(RuntimeException::class);
        $client->connectSse('/events');
    }

    public function testTestClientUsesNativePathWhenAvailable(): void
    {
        if (!\function_exists('spikard_version') || !\class_exists('\\Spikard\\Native\\TestClient')) {
            $this->markTestSkipped('Native test client not available.');
        }

        $previous = \getenv('SPIKARD_TEST_CLIENT_FORCE_PHP');
        \putenv('SPIKARD_TEST_CLIENT_FORCE_PHP=0');

        try {
            $app = (new App())->registerController(new class () {
                #[Get('/ping')]
                public function ping(): array
                {
                    return ['ok' => true];
                }
            });

            $client = TestClient::create($app);
            $response = $client->get('/ping');

            self::assertInstanceOf(Response::class, $response);

            $nativeProperty = new ReflectionProperty(TestClient::class, 'native');
            $nativeProperty->setAccessible(true);
            $nativeClient = $nativeProperty->getValue($client);
            self::assertNotNull($nativeClient);

            $client->close();
        } finally {
            if ($previous === false) {
                \putenv('SPIKARD_TEST_CLIENT_FORCE_PHP');
            } else {
                \putenv('SPIKARD_TEST_CLIENT_FORCE_PHP=' . $previous);
            }
        }
    }
}
