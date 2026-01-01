<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use ReflectionMethod;
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
use Spikard\Native\TestClient as NativeTestClient;

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
                /** @return array<string, mixed> */
                #[Post('/schema')]
                #[SchemaRef(request: 'req', response: 'resp', parameters: 'params')]
                public function create(): array
                {
                    return ['ok' => true];
                }
            });

        $routes = $app->routes();
        self::assertCount(1, $routes);
        /** @var array{request_schema: array<string, mixed>, response_schema: array<string, mixed>, parameter_schema: array<string, mixed>} $route */
        $route = $routes[0];

        self::assertSame($requestSchema, $route['request_schema']);
        self::assertSame($responseSchema, $route['response_schema']);
        self::assertSame($parameterSchema, $route['parameter_schema']);
    }

    public function testRegisterControllerInfersHeaderSchema(): void
    {
        $app = (new App())->registerController(new class () {
            /** @return array<string, mixed> */
            #[Get('/headers/{id}')]
            public function show(string $id, mixed $x_custom = new Header(alias: 'X-Custom-Header')): array
            {
                return ['id' => $id, 'header' => $x_custom];
            }
        });

        $routes = $app->routes();
        self::assertCount(1, $routes);
        /** @var array<string, mixed> $route */
        $route = $routes[0];
        /** @var mixed $schemaRaw */
        $schemaRaw = $route['parameter_schema'];
        if (!\is_array($schemaRaw)) {
            $schemaRaw = [];
        }
        /** @var array<string, mixed> $schema */
        $schema = $schemaRaw;
        self::assertSame('object', $schema['type']);

        /** @var mixed $propertiesRaw */
        $propertiesRaw = $schema['properties'];
        if (!\is_array($propertiesRaw)) {
            $propertiesRaw = [];
        }
        /** @var array<string, mixed> $properties */
        $properties = $propertiesRaw;
        self::assertArrayHasKey('x-custom-header', $properties);
        /** @var mixed $customHeaderRaw */
        $customHeaderRaw = $properties['x-custom-header'];
        if (\is_array($customHeaderRaw)) {
            self::assertSame('header', $customHeaderRaw['source']);
        }
        self::assertArrayHasKey('id', $properties);
        /** @var mixed $idRaw */
        $idRaw = $properties['id'];
        if (\is_array($idRaw)) {
            self::assertSame('path', $idRaw['source']);
            self::assertArrayHasKey('type', $idRaw);
            self::assertSame('string', $idRaw['type']);
        }
    }

    public function testNativeConfigReturnsDefaults(): void
    {
        $payload = (new App())->nativeConfig();

        self::assertArrayHasKey('host', $payload);
        self::assertArrayHasKey('port', $payload);
        self::assertArrayHasKey('static_files', $payload);
        self::assertIsArray($payload['static_files']);
    }

    public function testResolveSchemaRefThrowsWhenRegistryMissing(): void
    {
        $method = new ReflectionMethod(App::class, 'resolveSchemaRef');
        $method->setAccessible(true);

        $this->expectException(RuntimeException::class);
        $method->invoke(new App(), 'missing', null, 'request');
    }

    public function testResolveSchemaRefThrowsWhenKeyMissing(): void
    {
        $method = new ReflectionMethod(App::class, 'resolveSchemaRef');
        $method->setAccessible(true);

        $this->expectException(RuntimeException::class);
        $method->invoke(new App(), 'missing', ['present' => ['type' => 'object']], 'request');
    }

    public function testSchemaForTypeHandlesUnion(): void
    {
        $method = new ReflectionMethod(App::class, 'schemaForType');
        $method->setAccessible(true);

        $holder = new class () {
            public function demo(int|string|null $value): void
            {
            }
        };

        $reflection = new ReflectionMethod($holder, 'demo');
        $type = $reflection->getParameters()[0]->getType();
        $schema = $method->invoke(new App(), $type);

        self::assertIsArray($schema);
        self::assertArrayHasKey('anyOf', $schema);
        /** @var array<int, array<string, mixed>> $anyOf */
        $anyOf = $schema['anyOf'];
        $types = \array_map(static fn (array $entry) => $entry['type'] ?? null, $anyOf);
        \sort($types);
        self::assertSame(['integer', 'string'], $types);
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
        if (!\function_exists('spikard_version') || !\class_exists(NativeTestClient::class)) {
            $this->markTestSkipped('Native test client not available.');
        }

        $previous = \getenv('SPIKARD_TEST_CLIENT_FORCE_PHP');
        \putenv('SPIKARD_TEST_CLIENT_FORCE_PHP=0');

        try {
            $app = (new App())->registerController(new class () {
                /** @return array<string, mixed> */
                #[Get('/ping')]
                public function ping(): array
                {
                    return ['ok' => true];
                }
            });

            $client = TestClient::create($app);
            $nativeMethod = new ReflectionMethod(TestClient::class, 'nativeClient');
            $nativeMethod->setAccessible(true);
            $nativeClient = $nativeMethod->invoke($client);

            $nativeProperty = new ReflectionProperty(TestClient::class, 'native');
            $nativeProperty->setAccessible(true);
            $storedNative = $nativeProperty->getValue($client);

            self::assertInstanceOf(NativeTestClient::class, $nativeClient);
            self::assertSame($nativeClient, $storedNative);

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
