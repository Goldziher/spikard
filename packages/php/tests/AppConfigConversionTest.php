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
use Spikard\Config\JwtConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\OpenApiConfig;
use Spikard\Config\RateLimitConfig;
use Spikard\Config\ServerConfig;
use Spikard\Config\StaticFilesConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Behavioral tests for App::configToNative() and App::hooksToNative().
 *
 * Tests the conversion of PHP configuration and lifecycle hooks to native (Rust-compatible)
 * array format. This covers a major gap in App.php coverage (lines 368-500, ~130 lines).
 *
 * @internal
 */
final class AppConfigConversionTest extends TestCase
{
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
        $onRequest = static function (Request $request): \Spikard\Config\HookResult { return \Spikard\Config\HookResult::continue(); };
        $preValidation = static function (Request $request): \Spikard\Config\HookResult { return \Spikard\Config\HookResult::continue(); };
        $preHandler = static function (Request $request): \Spikard\Config\HookResult { return \Spikard\Config\HookResult::continue(); };
        $onResponse = static function (Request $request, \Spikard\Config\HookResult $hookResult): \Spikard\Config\HookResult { return $hookResult; };
        $onError = static function (Request $request, \Throwable $error): \Spikard\Config\HookResult { return \Spikard\Config\HookResult::continue(); };

        $hooks = LifecycleHooks::builder()
            ->onRequest($onRequest)
            ->preValidation($preValidation)
            ->preHandler($preHandler)
            ->onResponse($onResponse)
            ->onError($onError)
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
        $onRequest = static function (Request $request): \Spikard\Config\HookResult { return \Spikard\Config\HookResult::continue(); };
        $onResponse = static function (Request $request, \Spikard\Config\HookResult $hookResult): \Spikard\Config\HookResult { return $hookResult; };

        $hooks = LifecycleHooks::builder()
            ->onRequest($onRequest)
            ->onResponse($onResponse)
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

        $this->assertIsArray($native);
        $this->assertCount(0, $native);
    }

    public function testHooksToNativeSingleHook(): void
    {
        $onError = static function (Request $request, \Throwable $error): \Spikard\Config\HookResult { return \Spikard\Config\HookResult::continue(); };

        $hooks = LifecycleHooks::builder()
            ->onError($onError)
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

    /**
     * Invoke the private configToNative method via reflection.
     *
     * @return array<string, mixed>
     */
    private function invokeConfigToNative(App $app, ServerConfig $config): array
    {
        $method = new ReflectionMethod(App::class, 'configToNative');
        $method->setAccessible(true);
        /** @var array<string, mixed> */
        $result = $method->invoke($app, $config);
        return $result;
    }

    /**
     * Invoke the private hooksToNative method via reflection.
     *
     * @return array<string, mixed>
     */
    private function invokeHooksToNative(App $app, LifecycleHooks $hooks): array
    {
        $method = new ReflectionMethod(App::class, 'hooksToNative');
        $method->setAccessible(true);
        /** @var array<string, mixed> */
        $result = $method->invoke($app, $hooks);
        return $result;
    }
}
