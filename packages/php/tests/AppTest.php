<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use ReflectionMethod;
use RuntimeException;
use Spikard\App;
use Spikard\Config\ApiKeyConfig;
use Spikard\Config\CompressionConfig;
use Spikard\Config\CorsConfig;
use Spikard\Config\HookResult;
use Spikard\Config\JwtConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\OpenApiConfig;
use Spikard\Config\RateLimitConfig;
use Spikard\Config\ServerConfig;
use Spikard\Config\StaticFilesConfig;
use Spikard\DI\DependencyContainer;
use Spikard\Handlers\HandlerInterface;
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Throwable;

final class AppTest extends TestCase
{
    /**
     * Helper method to invoke configToNative using reflection.
     *
     * @return array<string, mixed> The native array representation
     */
    private function invokeConfigToNative(App $app, ServerConfig $config): array
    {
        $method = new ReflectionMethod($app, 'configToNative');
        $method->setAccessible(true);
        /** @var array<string, mixed> $result */
        $result = $method->invoke($app, $config);
        return $result;
    }

    /**
     * Helper method to invoke hooksToNative using reflection.
     *
     * @return array<string, callable> The native array representation
     */
    private function invokeHooksToNative(App $app, LifecycleHooks $hooks): array
    {
        $method = new ReflectionMethod($app, 'hooksToNative');
        $method->setAccessible(true);
        /** @var array<string, callable> $result */
        $result = $method->invoke($app, $hooks);
        return $result;
    }

    // ======================== Basic App Tests ========================

    public function testAppCreation(): void
    {
        $app = new App();

        $this->assertInstanceOf(App::class, $app);
        $this->assertNull($app->config());
        $this->assertNull($app->lifecycleHooks());
        $this->assertNull($app->dependencies());
        $this->assertSame([], $app->routes());
    }

    public function testAppWithConfig(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);

        $this->assertSame($config, $app->config());
    }

    public function testAppWithConfigMethod(): void
    {
        $config = ServerConfig::builder()->build();
        $app = (new App())->withConfig($config);

        $this->assertSame($config, $app->config());
    }

    public function testAppWithConfigIsImmutable(): void
    {
        $config = ServerConfig::builder()->build();
        $original = new App();
        $modified = $original->withConfig($config);

        $this->assertNotSame($original, $modified);
        $this->assertNull($original->config());
        $this->assertSame($config, $modified->config());
    }

    public function testAppWithLifecycleHooks(): void
    {
        $hooks = LifecycleHooks::builder()->build();
        $app = (new App())->withLifecycleHooks($hooks);

        $this->assertSame($hooks, $app->lifecycleHooks());
    }

    public function testAppWithLifecycleHooksIsImmutable(): void
    {
        $hooks = LifecycleHooks::builder()->build();
        $original = new App();
        $modified = $original->withLifecycleHooks($hooks);

        $this->assertNotSame($original, $modified);
        $this->assertNull($original->lifecycleHooks());
        $this->assertSame($hooks, $modified->lifecycleHooks());
    }

    public function testAppWithDependencies(): void
    {
        $deps = DependencyContainer::builder()->build();
        $app = (new App())->withDependencies($deps);

        $this->assertSame($deps, $app->dependencies());
    }

    public function testAppWithDependenciesIsImmutable(): void
    {
        $deps = DependencyContainer::builder()->build();
        $original = new App();
        $modified = $original->withDependencies($deps);

        $this->assertNotSame($original, $modified);
        $this->assertNull($original->dependencies());
        $this->assertSame($deps, $modified->dependencies());
    }

    public function testAppAddRoute(): void
    {
        $handler = new AppTestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/test', $routes[0]['path']);
        $this->assertSame($handler, $routes[0]['handler']);
    }

    public function testAppAddRouteIsImmutable(): void
    {
        $handler = new AppTestHandler();
        $original = new App();
        $modified = $original->addRoute('GET', '/test', $handler);

        $this->assertNotSame($original, $modified);
        $this->assertSame([], $original->routes());
        $this->assertCount(1, $modified->routes());
    }

    public function testAppAddMultipleRoutes(): void
    {
        $handler1 = new AppTestHandler();
        $handler2 = new AppTestHandler();

        $app = (new App())
            ->addRoute('GET', '/users', $handler1)
            ->addRoute('POST', '/users', $handler2);

        $routes = $app->routes();
        $this->assertCount(2, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('POST', $routes[1]['method']);
    }

    public function testAppAddRouteWithSchemas(): void
    {
        $handler = new AppTestHandler();
        $requestSchema = ['type' => 'object'];
        $responseSchema = ['type' => 'object'];
        $paramSchema = ['type' => 'object'];

        $app = (new App())->addRouteWithSchemas(
            'POST',
            '/items',
            $handler,
            $requestSchema,
            $responseSchema,
            $paramSchema
        );

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $route = $routes[0];
        $this->assertTrue(isset($route['request_schema']));
        $this->assertTrue(isset($route['response_schema']));
        $this->assertTrue(isset($route['parameter_schema']));
        $this->assertSame($requestSchema, $route['request_schema']);
    }

    public function testAppAddWebSocket(): void
    {
        $wsHandler = new AppTestWebSocketHandler();
        $app = (new App())->addWebSocket('/ws', $wsHandler);

        $handlers = $app->websocketHandlers();
        $this->assertCount(1, $handlers);
        $this->assertArrayHasKey('/ws', $handlers);
        $this->assertSame($wsHandler, $handlers['/ws']);
    }

    public function testAppAddSse(): void
    {
        $sseProducer = new AppTestSseProducer();
        $app = (new App())->addSse('/events', $sseProducer);

        $producers = $app->sseProducers();
        $this->assertCount(1, $producers);
        $this->assertArrayHasKey('/events', $producers);
        $this->assertSame($sseProducer, $producers['/events']);
    }

    public function testAppFindHandlerMatching(): void
    {
        $handler = new AppTestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $request = new Request('GET', '/test', null);
        $found = $app->findHandler($request);

        $this->assertSame($handler, $found);
    }

    public function testAppFindHandlerNotFound(): void
    {
        $handler = new AppTestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $request = new Request('GET', '/other', null);
        $found = $app->findHandler($request);

        $this->assertNull($found);
    }

    public function testAppFindHandlerDifferentMethod(): void
    {
        $handler = new AppTestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $request = new Request('POST', '/test', null);
        $found = $app->findHandler($request);

        $this->assertNull($found);
    }

    public function testAppFindHandlerStripQueryString(): void
    {
        $handler = new AppTestHandler();
        $app = (new App())->addRoute('GET', '/test?page=1', $handler);

        // Should match without query string
        $request = new Request('GET', '/test', null);
        $found = $app->findHandler($request);

        $this->assertSame($handler, $found);
    }

    public function testAppNativeRoutes(): void
    {
        $handler = new AppTestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(1, $nativeRoutes);
        $this->assertSame('GET', $nativeRoutes[0]['method']);
        $this->assertSame('/test', $nativeRoutes[0]['path']);
        if (isset($nativeRoutes[0]['handler'])) {
            $this->assertSame($handler, $nativeRoutes[0]['handler']);
        }
    }

    public function testAppNativeRoutesIncludesWebSocket(): void
    {
        $wsHandler = new AppTestWebSocketHandler();
        $app = (new App())->addWebSocket('/ws', $wsHandler);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(1, $nativeRoutes);
        $this->assertSame('GET', $nativeRoutes[0]['method']);
        $this->assertSame('/ws', $nativeRoutes[0]['path']);
        if (isset($nativeRoutes[0]['websocket'])) {
            $this->assertTrue($nativeRoutes[0]['websocket']);
        }
    }

    public function testAppNativeRoutesIncludesSse(): void
    {
        $sseProducer = new AppTestSseProducer();
        $app = (new App())->addSse('/events', $sseProducer);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(1, $nativeRoutes);
        $this->assertSame('GET', $nativeRoutes[0]['method']);
        $this->assertSame('/events', $nativeRoutes[0]['path']);
        if (isset($nativeRoutes[0]['sse'])) {
            $this->assertTrue($nativeRoutes[0]['sse']);
        }
    }

    public function testAppSingleRoute(): void
    {
        $handler = new AppTestHandler();
        $app = App::singleRoute('GET', '/hello', $handler);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/hello', $routes[0]['path']);
    }

    public function testAppMethodsCaseInsensitive(): void
    {
        $handler = new AppTestHandler();
        $app = (new App())
            ->addRoute('get', '/test1', $handler)
            ->addRoute('POST', '/test2', $handler);

        $request1 = new Request('GET', '/test1', null);
        $request2 = new Request('post', '/test2', null);

        $this->assertSame($handler, $app->findHandler($request1));
        $this->assertSame($handler, $app->findHandler($request2));
    }

    public function testAppChaining(): void
    {
        $config = ServerConfig::builder()->build();
        $hooks = LifecycleHooks::builder()->build();
        $deps = DependencyContainer::builder()->build();
        $handler = new AppTestHandler();

        $app = (new App())
            ->withConfig($config)
            ->withLifecycleHooks($hooks)
            ->withDependencies($deps)
            ->addRoute('GET', '/test', $handler);

        $this->assertSame($config, $app->config());
        $this->assertSame($hooks, $app->lifecycleHooks());
        $this->assertSame($deps, $app->dependencies());
        $this->assertCount(1, $app->routes());
    }

    public function testAppImmutabilityThroughChain(): void
    {
        $original = new App();
        $step1 = $original->withConfig(ServerConfig::builder()->build());
        $step2 = $step1->addRoute('GET', '/test', new AppTestHandler());

        $this->assertNotSame($original, $step1);
        $this->assertNotSame($step1, $step2);
        $this->assertNull($original->config());
        $this->assertSame([], $original->routes());
        $this->assertCount(1, $step2->routes());
    }

    // ======================== Configuration Conversion Tests ========================

    public function testConfigToNativeBasicSettings(): void
    {
        $config = ServerConfig::builder()
            ->withHost('0.0.0.0')
            ->withPort(9000)
            ->withWorkers(4)
            ->withRequestId(false)
            ->withMaxBodySize(5242880) // 5 MB
            ->withRequestTimeout(60)
            ->withGracefulShutdown(false)
            ->withShutdownTimeout(15)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertSame('0.0.0.0', $native['host']);
        $this->assertSame(9000, $native['port']);
        $this->assertSame(4, $native['workers']);
        $this->assertSame(false, $native['enable_request_id']);
        $this->assertSame(5242880, $native['max_body_size']);
        $this->assertSame(60, $native['request_timeout']);
        $this->assertSame(false, $native['graceful_shutdown']);
        $this->assertSame(15, $native['shutdown_timeout']);
    }

    public function testConfigToNativeCompressionEnabled(): void
    {
        $compression = CompressionConfig::builder()
            ->withGzip(true)
            ->withBrotli(true)
            ->withMinSize(2048)
            ->withQuality(8)
            ->build();

        $config = ServerConfig::builder()
            ->withCompression($compression)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('compression', $native);
        /** @var array<string, mixed> */
        $compressionConfig = $native['compression'];
        $this->assertSame(true, $compressionConfig['gzip']);
        $this->assertSame(true, $compressionConfig['brotli']);
        $this->assertSame(2048, $compressionConfig['min_size']);
        $this->assertSame(8, $compressionConfig['quality']);
    }

    public function testConfigToNativeCompressionDisabled(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('compression', $native);
    }

    public function testConfigToNativeRateLimitEnabled(): void
    {
        $rateLimit = RateLimitConfig::builder()
            ->withPerSecond(100)
            ->withBurst(200)
            ->withIpBased(true)
            ->build();

        $config = ServerConfig::builder()
            ->withRateLimit($rateLimit)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('rate_limit', $native);
        /** @var array<string, mixed> */
        $rateLimitConfig = $native['rate_limit'];
        $this->assertSame(100, $rateLimitConfig['per_second']);
        $this->assertSame(200, $rateLimitConfig['burst']);
        $this->assertSame(true, $rateLimitConfig['ip_based']);
    }

    public function testConfigToNativeRateLimitDisabled(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('rate_limit', $native);
    }

    public function testConfigToNativeJwtAuthEnabled(): void
    {
        $jwtAuth = JwtConfig::builder()
            ->withSecret('my-secret-key')
            ->withAlgorithm('HS256')
            ->withAudience(['myapp'])
            ->withIssuer('issuer.example.com')
            ->withLeeway(10)
            ->build();

        $config = ServerConfig::builder()
            ->withJwtAuth($jwtAuth)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('jwt_auth', $native);
        /** @var array<string, mixed> */
        $jwtAuthConfig = $native['jwt_auth'];
        $this->assertSame('my-secret-key', $jwtAuthConfig['secret']);
        $this->assertSame('HS256', $jwtAuthConfig['algorithm']);
        $this->assertSame(['myapp'], $jwtAuthConfig['audience']);
        $this->assertSame('issuer.example.com', $jwtAuthConfig['issuer']);
        $this->assertSame(10, $jwtAuthConfig['leeway']);
    }

    public function testConfigToNativeJwtAuthDisabled(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('jwt_auth', $native);
    }

    public function testConfigToNativeApiKeyAuthEnabled(): void
    {
        $apiKeyAuth = ApiKeyConfig::builder()
            ->withKeys(['key1', 'key2', 'key3'])
            ->withHeaderName('X-API-Key')
            ->build();

        $config = ServerConfig::builder()
            ->withApiKeyAuth($apiKeyAuth)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('api_key_auth', $native);
        /** @var array<string, mixed> */
        $apiKeyAuthConfig = $native['api_key_auth'];
        $this->assertSame(['key1', 'key2', 'key3'], $apiKeyAuthConfig['keys']);
        $this->assertSame('X-API-Key', $apiKeyAuthConfig['header_name']);
    }

    public function testConfigToNativeApiKeyAuthDisabled(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('api_key_auth', $native);
    }

    public function testConfigToNativeCorsEnabled(): void
    {
        $cors = CorsConfig::builder()
            ->withEnabled(true)
            ->withAllowedOrigins(['https://example.com', 'https://api.example.com'])
            ->withAllowedMethods(['GET', 'POST', 'PUT', 'DELETE'])
            ->withAllowedHeaders(['Content-Type', 'Authorization'])
            ->withExposedHeaders(['X-Total-Count', 'X-Page'])
            ->withMaxAgeSeconds(3600)
            ->withAllowCredentials(true)
            ->build();

        $config = ServerConfig::builder()
            ->withCors($cors)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('cors', $native);
        /** @var array<string, mixed> */
        $corsConfig = $native['cors'];
        $this->assertSame(['https://example.com', 'https://api.example.com'], $corsConfig['allowed_origins']);
        $this->assertSame(['GET', 'POST', 'PUT', 'DELETE'], $corsConfig['allowed_methods']);
        $this->assertSame(['Content-Type', 'Authorization'], $corsConfig['allowed_headers']);
        $this->assertSame(['X-Total-Count', 'X-Page'], $corsConfig['expose_headers']);
        $this->assertSame(3600, $corsConfig['max_age']);
        $this->assertSame(true, $corsConfig['allow_credentials']);
    }

    public function testConfigToNativeCorsDisabled(): void
    {
        $cors = CorsConfig::builder()
            ->withEnabled(false)
            ->build();

        $config = ServerConfig::builder()
            ->withCors($cors)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('cors', $native);
    }

    public function testConfigToNativeCorsNotSet(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('cors', $native);
    }

    public function testConfigToNativeStaticFilesEnabled(): void
    {
        $staticFiles = StaticFilesConfig::builder()
            ->withEnabled(true)
            ->withRoot('/var/www/public')
            ->withIndexFile('index.html')
            ->withCache(true)
            ->build();

        $config = ServerConfig::builder()
            ->withStaticFiles($staticFiles)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('static_files', $native);
        $this->assertIsArray($native['static_files']);
        $this->assertCount(1, $native['static_files']);
        /** @var array<string, mixed> */
        $staticFilesEntry = $native['static_files'][0];
        $this->assertSame('/var/www/public', $staticFilesEntry['directory']);
        $this->assertSame('/', $staticFilesEntry['route_prefix']);
        $this->assertTrue($staticFilesEntry['index_file']);
        $this->assertNotNull($staticFilesEntry['cache_control']);
    }

    public function testConfigToNativeStaticFilesDisabled(): void
    {
        $staticFiles = StaticFilesConfig::builder()
            ->withEnabled(false)
            ->build();

        $config = ServerConfig::builder()
            ->withStaticFiles($staticFiles)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('static_files', $native);
        $this->assertSame([], $native['static_files']);
    }

    public function testConfigToNativeStaticFilesNotSet(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('static_files', $native);
        $this->assertSame([], $native['static_files']);
    }

    public function testConfigToNativeOpenApiEnabled(): void
    {
        $openapi = OpenApiConfig::builder()
            ->withEnabled(true)
            ->withTitle('My API')
            ->withVersion('1.0.0')
            ->withDescription('API documentation')
            ->withSwaggerUiPath('/swagger')
            ->withRedocPath('/redoc')
            ->withOpenApiJsonPath('/openapi.json')
            ->build();

        $config = ServerConfig::builder()
            ->withOpenapi($openapi)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('openapi', $native);
        /** @var array<string, mixed> */
        $openapiConfig = $native['openapi'];
        $this->assertSame(true, $openapiConfig['enabled']);
        $this->assertSame('My API', $openapiConfig['title']);
        $this->assertSame('1.0.0', $openapiConfig['version']);
        $this->assertSame('API documentation', $openapiConfig['description']);
        $this->assertSame('/swagger', $openapiConfig['swagger_ui_path']);
        $this->assertSame('/redoc', $openapiConfig['redoc_path']);
        $this->assertSame('/openapi.json', $openapiConfig['openapi_json_path']);
    }

    public function testConfigToNativeOpenApiDisabled(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('openapi', $native);
    }

    public function testConfigToNativeAllConfigurationsEnabled(): void
    {
        $compression = CompressionConfig::builder()->build();
        $rateLimit = RateLimitConfig::builder()->build();
        $jwtAuth = JwtConfig::builder()->withSecret('secret')->build();
        $apiKeyAuth = ApiKeyConfig::builder()->withKeys(['key1'])->build();
        $cors = CorsConfig::builder()->withEnabled(true)->build();
        $staticFiles = StaticFilesConfig::builder()->withRoot('/public')->build();
        $openapi = OpenApiConfig::builder()->withEnabled(true)->build();

        $config = ServerConfig::builder()
            ->withCompression($compression)
            ->withRateLimit($rateLimit)
            ->withJwtAuth($jwtAuth)
            ->withApiKeyAuth($apiKeyAuth)
            ->withCors($cors)
            ->withStaticFiles($staticFiles)
            ->withOpenapi($openapi)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('compression', $native);
        $this->assertArrayHasKey('rate_limit', $native);
        $this->assertArrayHasKey('jwt_auth', $native);
        $this->assertArrayHasKey('api_key_auth', $native);
        $this->assertArrayHasKey('cors', $native);
        $this->assertArrayHasKey('static_files', $native);
        $this->assertArrayHasKey('openapi', $native);
    }

    public function testHooksToNativeAllHooks(): void
    {
        $onRequest = static function (Request $request): HookResult { return HookResult::continue(); };
        $preValidation = static function (Request $request): HookResult { return HookResult::continue(); };
        $preHandler = static function (Request $request): HookResult { return HookResult::continue(); };
        $onResponse = static function (Request $request, HookResult $hookResult): HookResult { return $hookResult; };
        $onError = static function (Request $request, Throwable $error): HookResult { return HookResult::continue(); };

        $hooks = LifecycleHooks::builder()
            ->withOnRequest($onRequest)
            ->withPreValidation($preValidation)
            ->withPreHandler($preHandler)
            ->withOnResponse($onResponse)
            ->withOnError($onError)
            ->build();

        $app = (new App())->withLifecycleHooks($hooks);
        $native = $this->invokeHooksToNative($app, $hooks);

        $this->assertArrayHasKey('onRequest', $native);
        $this->assertArrayHasKey('preValidation', $native);
        $this->assertArrayHasKey('preHandler', $native);
        $this->assertArrayHasKey('onResponse', $native);
        $this->assertArrayHasKey('onError', $native);

        $this->assertSame($onRequest, $native['onRequest']);
        $this->assertSame($preValidation, $native['preValidation']);
        $this->assertSame($preHandler, $native['preHandler']);
        $this->assertSame($onResponse, $native['onResponse']);
        $this->assertSame($onError, $native['onError']);
    }

    public function testHooksToNativePartialHooks(): void
    {
        $onRequest = static function (Request $request): HookResult { return HookResult::continue(); };
        $onResponse = static function (Request $request, HookResult $hookResult): HookResult { return $hookResult; };

        $hooks = LifecycleHooks::builder()
            ->withOnRequest($onRequest)
            ->withOnResponse($onResponse)
            ->build();

        $app = (new App())->withLifecycleHooks($hooks);
        $native = $this->invokeHooksToNative($app, $hooks);

        $this->assertArrayHasKey('onRequest', $native);
        $this->assertArrayHasKey('onResponse', $native);
        $this->assertArrayNotHasKey('preValidation', $native);
        $this->assertArrayNotHasKey('preHandler', $native);
        $this->assertArrayNotHasKey('onError', $native);

        $this->assertCount(2, $native);
    }

    public function testHooksToNativeEmptyHooks(): void
    {
        $hooks = LifecycleHooks::builder()->build();
        $app = (new App())->withLifecycleHooks($hooks);
        $native = $this->invokeHooksToNative($app, $hooks);

        $this->assertCount(0, $native);
    }

    public function testHooksToNativeSingleHook(): void
    {
        $onError = static function (Request $request, Throwable $error): HookResult { return HookResult::continue(); };

        $hooks = LifecycleHooks::builder()
            ->withOnError($onError)
            ->build();

        $app = (new App())->withLifecycleHooks($hooks);
        $native = $this->invokeHooksToNative($app, $hooks);

        $this->assertArrayHasKey('onError', $native);
        $this->assertArrayNotHasKey('onRequest', $native);
        $this->assertCount(1, $native);
    }

    public function testConfigToNativeSnakeCaseFieldNames(): void
    {
        $config = ServerConfig::builder()
            ->withRequestId(true)
            ->withMaxBodySize(1024)
            ->withRequestTimeout(30)
            ->withGracefulShutdown(true)
            ->withShutdownTimeout(60)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        // Verify snake_case field names are used (not camelCase)
        $this->assertArrayHasKey('enable_request_id', $native);
        $this->assertArrayHasKey('max_body_size', $native);
        $this->assertArrayHasKey('request_timeout', $native);
        $this->assertArrayHasKey('graceful_shutdown', $native);
        $this->assertArrayHasKey('shutdown_timeout', $native);

        // These should NOT exist (camelCase versions)
        $this->assertArrayNotHasKey('enableRequestId', $native);
        $this->assertArrayNotHasKey('maxBodySize', $native);
        $this->assertArrayNotHasKey('requestTimeout', $native);
        $this->assertArrayNotHasKey('gracefulShutdown', $native);
        $this->assertArrayNotHasKey('shutdownTimeout', $native);
    }

    public function testConfigToNativeCompressionDefaults(): void
    {
        $compression = CompressionConfig::builder()->build();
        $config = ServerConfig::builder()
            ->withCompression($compression)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        // Compression should have defaults applied via null coalescing
        /** @var array<string, mixed> */
        $compressionConfig = $native['compression'];
        $this->assertSame(true, $compressionConfig['gzip']);
        $this->assertSame(true, $compressionConfig['brotli']);
        $this->assertSame(1024, $compressionConfig['min_size']);
        $this->assertSame(6, $compressionConfig['quality']);
    }

    public function testConfigToNativeRateLimitDefaults(): void
    {
        $rateLimit = RateLimitConfig::builder()
            ->withPerSecond(100)
            ->withBurst(50)
            ->build();

        $config = ServerConfig::builder()
            ->withRateLimit($rateLimit)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        /** @var array<string, mixed> */
        $rateLimitConfig = $native['rate_limit'];
        $this->assertSame(true, $rateLimitConfig['ip_based']); // Default from null coalescing
    }

    public function testConfigToNativeStaticFilesWithoutCache(): void
    {
        $staticFiles = StaticFilesConfig::builder()
            ->withRoot('/public')
            ->withCache(false)
            ->build();

        $config = ServerConfig::builder()
            ->withStaticFiles($staticFiles)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        /** @var list<array<string, mixed>> */
        $staticFilesArray = $native['static_files'];
        /** @var array<string, mixed> */
        $staticFilesEntry = $staticFilesArray[0];
        $this->assertNull($staticFilesEntry['cache_control']);
    }

    // ======================== Additional Serialization Tests ========================

    public function testConfigToNativeWithCorsConfigEnabledExtended(): void
    {
        $corsConfig = CorsConfig::builder()
            ->withEnabled(true)
            ->withAllowedOrigins(['http://localhost:3000', 'https://example.com'])
            ->withAllowedMethods(['GET', 'POST', 'OPTIONS'])
            ->withAllowedHeaders(['Content-Type', 'Authorization'])
            ->withExposedHeaders(['X-Total-Count'])
            ->withMaxAgeSeconds(3600)
            ->withAllowCredentials(true)
            ->build();

        $config = ServerConfig::builder()
            ->withCors($corsConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('cors', $native);
        /** @var array<string, mixed> */
        $cors = $native['cors'];
        $this->assertSame(
            ['http://localhost:3000', 'https://example.com'],
            $cors['allowed_origins']
        );
        $this->assertSame(['GET', 'POST', 'OPTIONS'], $cors['allowed_methods']);
        $this->assertSame(['Content-Type', 'Authorization'], $cors['allowed_headers']);
        $this->assertSame(['X-Total-Count'], $cors['expose_headers']);
        $this->assertSame(3600, $cors['max_age']);
        $this->assertTrue($cors['allow_credentials']);
    }

    public function testConfigToNativeWithStaticFilesConfigEnabledExtended(): void
    {
        $staticConfig = StaticFilesConfig::builder()
            ->withEnabled(true)
            ->withRoot('/var/www/public')
            ->withIndexFile('index.html')
            ->withCache(true)
            ->build();

        $config = ServerConfig::builder()
            ->withStaticFiles($staticConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('static_files', $native);
        /** @var list<array<string, mixed>> */
        $staticFiles = $native['static_files'];
        $this->assertCount(1, $staticFiles);
        /** @var array<string, mixed> */
        $firstFile = $staticFiles[0];
        $this->assertSame('/var/www/public', $firstFile['directory']);
        $this->assertSame('/', $firstFile['route_prefix']);
        $this->assertTrue($firstFile['index_file']);
        $this->assertSame('public, max-age=3600', $firstFile['cache_control']);
    }

    public function testConfigToNativeWithAllConfigurationsExtended(): void
    {
        $compressionConfig = CompressionConfig::builder()
            ->withGzip(true)
            ->withBrotli(true)
            ->withMinSize(1024)
            ->withQuality(6)
            ->build();

        $rateLimitConfig = RateLimitConfig::builder()
            ->withPerSecond(100)
            ->withBurst(50)
            ->withIpBased(true)
            ->build();

        $jwtConfig = JwtConfig::builder()
            ->withSecret('secret')
            ->withAlgorithm('HS256')
            ->withAudience(['api'])
            ->withIssuer('issuer')
            ->withLeeway(5)
            ->build();

        $apiKeyConfig = ApiKeyConfig::builder()
            ->withKeys(['key1'])
            ->withHeaderName('X-API-Key')
            ->build();

        $corsConfig = CorsConfig::builder()
            ->withEnabled(true)
            ->withAllowedOrigins(['*'])
            ->withAllowedMethods(['GET', 'POST'])
            ->withAllowedHeaders(['Content-Type'])
            ->withExposedHeaders(['X-Total'])
            ->withMaxAgeSeconds(86400)
            ->withAllowCredentials(true)
            ->build();

        $staticConfig = StaticFilesConfig::builder()
            ->withEnabled(true)
            ->withRoot('/public')
            ->withIndexFile('index.html')
            ->withCache(true)
            ->build();

        $openApiConfig = OpenApiConfig::builder()
            ->withEnabled(true)
            ->withTitle('API')
            ->withVersion('1.0')
            ->withDescription('desc')
            ->withSwaggerUiPath('/docs')
            ->withRedocPath('/redoc')
            ->withOpenapiJsonPath('/openapi.json')
            ->build();

        $config = ServerConfig::builder()
            ->withHost('127.0.0.1')
            ->withPort(8080)
            ->withWorkers(4)
            ->withRequestId(true)
            ->withMaxBodySize(1048576)
            ->withRequestTimeout(30000)
            ->withGracefulShutdown(true)
            ->withShutdownTimeout(5000)
            ->withCompression($compressionConfig)
            ->withRateLimit($rateLimitConfig)
            ->withJwtAuth($jwtConfig)
            ->withApiKeyAuth($apiKeyConfig)
            ->withCors($corsConfig)
            ->withStaticFiles($staticConfig)
            ->withOpenApi($openApiConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        // Verify all major sections are present
        $this->assertSame('127.0.0.1', $native['host']);
        $this->assertArrayHasKey('compression', $native);
        $this->assertArrayHasKey('rate_limit', $native);
        $this->assertArrayHasKey('jwt_auth', $native);
        $this->assertArrayHasKey('api_key_auth', $native);
        $this->assertArrayHasKey('cors', $native);
        $this->assertArrayHasKey('static_files', $native);
        $this->assertArrayHasKey('openapi', $native);
    }

    public function testConfigToNativeSnakeCaseConversion(): void
    {
        $config = ServerConfig::builder()
            ->withRequestId(false)
            ->withMaxBodySize(5242880)
            ->withRequestTimeout(60000)
            ->withGracefulShutdown(false)
            ->withShutdownTimeout(30000)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        // Verify snake_case conversion
        $this->assertArrayHasKey('enable_request_id', $native);
        $this->assertArrayHasKey('max_body_size', $native);
        $this->assertArrayHasKey('request_timeout', $native);
        $this->assertArrayHasKey('graceful_shutdown', $native);
        $this->assertArrayHasKey('shutdown_timeout', $native);

        // Verify values
        $this->assertFalse($native['enable_request_id']);
        $this->assertSame(5242880, $native['max_body_size']);
        $this->assertSame(60000, $native['request_timeout']);
        $this->assertFalse($native['graceful_shutdown']);
        $this->assertSame(30000, $native['shutdown_timeout']);
    }

    public function testHooksToNativeCallablePreservation(): void
    {
        $callables = [
            'onRequest' => static function (Request $request): HookResult {
                return HookResult::continue();
            },
            'preValidation' => static function (Request $request): HookResult {
                return HookResult::continue();
            },
            'preHandler' => static function (Request $request): HookResult {
                return HookResult::continue();
            },
            'onResponse' => static function (Request $request, HookResult $result): HookResult {
                return $result;
            },
            'onError' => static function (Request $request, Throwable $error): HookResult {
                return HookResult::continue();
            },
        ];

        $hooks = LifecycleHooks::builder()
            ->withOnRequest($callables['onRequest'])
            ->withPreValidation($callables['preValidation'])
            ->withPreHandler($callables['preHandler'])
            ->withOnResponse($callables['onResponse'])
            ->withOnError($callables['onError'])
            ->build();

        $app = new App();
        $native = $this->invokeHooksToNative($app, $hooks);

        // Callables should be preserved exactly
        foreach (['onRequest', 'preValidation', 'preHandler', 'onResponse', 'onError'] as $hookName) {
            /** @var callable $nativeCallable */
            $nativeCallable = $native[$hookName];
            $this->assertSame($callables[$hookName], $nativeCallable);
        }
    }
}

// Test helpers
final class AppTestHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['ok' => true], 200);
    }
}

final class AppTestWebSocketHandler implements WebSocketHandlerInterface
{
    public function onConnect(): void
    {
    }

    public function onMessage(string $message): void
    {
    }

    public function onClose(int $code, ?string $reason = null): void
    {
    }
}

final class AppTestSseProducer implements SseEventProducerInterface
{
    public function __invoke(): \Generator
    {
        yield "data: test\n\n";
    }
}
