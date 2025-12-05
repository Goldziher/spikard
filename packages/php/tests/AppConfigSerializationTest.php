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
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Throwable;

/**
 * Tests for App.php configuration serialization (configToNative, hooksToNative).
 *
 * Pushes App.php from 50.26% coverage to 80%+ by testing:
 * 1. configToNative() serialization of all ServerConfig fields
 * 2. hooksToNative() serialization of all LifecycleHooks
 * 3. Edge cases and null handling
 * 4. Snake_case field name transformation
 */
final class AppConfigSerializationTest extends TestCase
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

    // ======================== configToNative - Basic Fields ========================

    public function testConfigToNativeBasicServerSettings(): void
    {
        $config = ServerConfig::builder()
            ->withHost('0.0.0.0')
            ->withPort(9000)
            ->withWorkers(8)
            ->withRequestId(true)
            ->withMaxBodySize(10485760)
            ->withRequestTimeout(30000)
            ->withGracefulShutdown(true)
            ->withShutdownTimeout(10000)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertSame('0.0.0.0', $native['host']);
        $this->assertSame(9000, $native['port']);
        $this->assertSame(8, $native['workers']);
        $this->assertTrue($native['enable_request_id']);
        $this->assertSame(10485760, $native['max_body_size']);
        $this->assertSame(30000, $native['request_timeout']);
        $this->assertTrue($native['graceful_shutdown']);
        $this->assertSame(10000, $native['shutdown_timeout']);
    }

    // ======================== configToNative - Compression ========================

    public function testConfigToNativeWithCompressionConfig(): void
    {
        $compressionConfig = CompressionConfig::builder()
            ->withGzip(true)
            ->withBrotli(true)
            ->withMinSize(2048)
            ->withQuality(7)
            ->build();

        $config = ServerConfig::builder()
            ->withCompression($compressionConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('compression', $native);
        /** @var array<string, mixed> */
        $compression = $native['compression'];
        $this->assertTrue($compression['gzip']);
        $this->assertTrue($compression['brotli']);
        $this->assertSame(2048, $compression['min_size']);
        $this->assertSame(7, $compression['quality']);
    }

    public function testConfigToNativeWithoutCompressionConfig(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('compression', $native);
    }

    // ======================== configToNative - Rate Limiting ========================

    public function testConfigToNativeWithRateLimitConfig(): void
    {
        $rateLimitConfig = RateLimitConfig::builder()
            ->withPerSecond(100)
            ->withBurst(50)
            ->withIpBased(true)
            ->build();

        $config = ServerConfig::builder()
            ->withRateLimit($rateLimitConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('rate_limit', $native);
        /** @var array<string, mixed> */
        $rateLimit = $native['rate_limit'];
        $this->assertSame(100, $rateLimit['per_second']);
        $this->assertSame(50, $rateLimit['burst']);
        $this->assertTrue($rateLimit['ip_based']);
    }

    public function testConfigToNativeWithoutRateLimitConfig(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('rate_limit', $native);
    }

    // ======================== configToNative - JWT Authentication ========================

    public function testConfigToNativeWithJwtAuthConfig(): void
    {
        $jwtConfig = JwtConfig::builder()
            ->withSecret('my-secret-key')
            ->withAlgorithm('HS256')
            ->withAudience(['api.example.com'])
            ->withIssuer('example.com')
            ->withLeeway(10)
            ->build();

        $config = ServerConfig::builder()
            ->withJwtAuth($jwtConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('jwt_auth', $native);
        /** @var array<string, mixed> */
        $jwtAuth = $native['jwt_auth'];
        $this->assertSame('my-secret-key', $jwtAuth['secret']);
        $this->assertSame('HS256', $jwtAuth['algorithm']);
        /** @var list<string> */
        $audience = $jwtAuth['audience'];
        $this->assertSame('api.example.com', $audience[0]);
        $this->assertSame('example.com', $jwtAuth['issuer']);
        $this->assertSame(10, $jwtAuth['leeway']);
    }

    public function testConfigToNativeWithoutJwtAuthConfig(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('jwt_auth', $native);
    }

    // ======================== configToNative - API Key Authentication ========================

    public function testConfigToNativeWithApiKeyAuthConfig(): void
    {
        $apiKeyConfig = ApiKeyConfig::builder()
            ->withKeys(['key1', 'key2', 'key3'])
            ->withHeaderName('X-API-Key')
            ->build();

        $config = ServerConfig::builder()
            ->withApiKeyAuth($apiKeyConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('api_key_auth', $native);
        /** @var array<string, mixed> */
        $apiKeyAuth = $native['api_key_auth'];
        $this->assertSame(['key1', 'key2', 'key3'], $apiKeyAuth['keys']);
        $this->assertSame('X-API-Key', $apiKeyAuth['header_name']);
    }

    public function testConfigToNativeWithoutApiKeyAuthConfig(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('api_key_auth', $native);
    }

    // ======================== configToNative - CORS ========================

    public function testConfigToNativeWithCorsConfigEnabled(): void
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

    public function testConfigToNativeWithCorsConfigDisabled(): void
    {
        $corsConfig = CorsConfig::builder()
            ->withEnabled(false)
            ->build();

        $config = ServerConfig::builder()
            ->withCors($corsConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('cors', $native);
    }

    public function testConfigToNativeWithoutCorsConfig(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('cors', $native);
    }

    // ======================== configToNative - Static Files ========================

    public function testConfigToNativeWithStaticFilesConfigEnabled(): void
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

    public function testConfigToNativeWithStaticFilesConfigDisabled(): void
    {
        $staticConfig = StaticFilesConfig::builder()
            ->withEnabled(false)
            ->build();

        $config = ServerConfig::builder()
            ->withStaticFiles($staticConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('static_files', $native);
        /** @var array<string, mixed> */
        $staticFiles = $native['static_files'];
        $this->assertCount(0, $staticFiles);
    }

    public function testConfigToNativeWithoutStaticFilesConfig(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('static_files', $native);
        /** @var array<string, mixed> */
        $staticFiles = $native['static_files'];
        $this->assertCount(0, $staticFiles);
    }

    // ======================== configToNative - OpenAPI ========================

    public function testConfigToNativeWithOpenApiConfig(): void
    {
        $openApiConfig = OpenApiConfig::builder()
            ->withEnabled(true)
            ->withTitle('My API')
            ->withVersion('1.0.0')
            ->withDescription('API description')
            ->withSwaggerUiPath('/docs')
            ->withRedocPath('/redoc')
            ->withOpenapiJsonPath('/openapi.json')
            ->build();

        $config = ServerConfig::builder()
            ->withOpenApi($openApiConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayHasKey('openapi', $native);
        /** @var array<string, mixed> */
        $openapi = $native['openapi'];
        $this->assertTrue($openapi['enabled']);
        $this->assertSame('My API', $openapi['title']);
        $this->assertSame('1.0.0', $openapi['version']);
        $this->assertSame('API description', $openapi['description']);
        $this->assertSame('/docs', $openapi['swagger_ui_path']);
        $this->assertSame('/redoc', $openapi['redoc_path']);
        $this->assertSame('/openapi.json', $openapi['openapi_json_path']);
    }

    public function testConfigToNativeWithoutOpenApiConfig(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        $this->assertArrayNotHasKey('openapi', $native);
    }

    // ======================== hooksToNative - All Hook Types ========================

    public function testHooksToNativeWithAllHooks(): void
    {
        $onRequestHook = static function (Request $request): HookResult {
            return HookResult::continue();
        };
        $preValidationHook = static function (Request $request): HookResult {
            return HookResult::continue();
        };
        $preHandlerHook = static function (Request $request): HookResult {
            return HookResult::continue();
        };
        $onResponseHook = static function (Request $request, HookResult $result): HookResult {
            return $result;
        };
        $onErrorHook = static function (Request $request, Throwable $error): HookResult {
            return HookResult::continue();
        };

        $hooks = LifecycleHooks::builder()
            ->withOnRequest($onRequestHook)
            ->withPreValidation($preValidationHook)
            ->withPreHandler($preHandlerHook)
            ->withOnResponse($onResponseHook)
            ->withOnError($onErrorHook)
            ->build();

        $app = new App();
        $native = $this->invokeHooksToNative($app, $hooks);

        $this->assertArrayHasKey('onRequest', $native);
        $this->assertArrayHasKey('preValidation', $native);
        $this->assertArrayHasKey('preHandler', $native);
        $this->assertArrayHasKey('onResponse', $native);
        $this->assertArrayHasKey('onError', $native);
        $this->assertSame($onRequestHook, $native['onRequest']);
        $this->assertSame($preValidationHook, $native['preValidation']);
        $this->assertSame($preHandlerHook, $native['preHandler']);
        $this->assertSame($onResponseHook, $native['onResponse']);
        $this->assertSame($onErrorHook, $native['onError']);
    }

    public function testHooksToNativeWithSingleHook(): void
    {
        $onRequestHook = static function (Request $request): HookResult {
            return HookResult::continue();
        };

        $hooks = LifecycleHooks::builder()
            ->withOnRequest($onRequestHook)
            ->build();

        $app = new App();
        $native = $this->invokeHooksToNative($app, $hooks);

        $this->assertArrayHasKey('onRequest', $native);
        $this->assertArrayNotHasKey('preValidation', $native);
        $this->assertArrayNotHasKey('preHandler', $native);
        $this->assertArrayNotHasKey('onResponse', $native);
        $this->assertArrayNotHasKey('onError', $native);
    }

    public function testHooksToNativeWithoutHooks(): void
    {
        $hooks = LifecycleHooks::builder()->build();

        $app = new App();
        $native = $this->invokeHooksToNative($app, $hooks);

        $this->assertCount(0, $native);
    }

    // ======================== Integration - configToNative with All Configs ========================

    public function testConfigToNativeWithAllConfigurations(): void
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

    // ======================== Edge Cases ========================

    public function testConfigToNativeWithCompressionDefaults(): void
    {
        $compressionConfig = CompressionConfig::builder()->build();

        $config = ServerConfig::builder()
            ->withCompression($compressionConfig)
            ->build();

        $app = new App($config);
        $native = $this->invokeConfigToNative($app, $config);

        // Defaults should use null coalescing operator
        /** @var array<string, mixed> */
        $compression = $native['compression'];
        $this->assertTrue($compression['gzip'] ?? false);
        $this->assertTrue($compression['brotli'] ?? false);
        $this->assertSame(1024, $compression['min_size'] ?? 0);
        $this->assertSame(6, $compression['quality'] ?? 0);
    }

    public function testConfigToNativeSnakeCaseFieldNames(): void
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
