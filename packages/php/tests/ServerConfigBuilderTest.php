<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Config\ApiKeyConfig;
use Spikard\Config\CompressionConfig;
use Spikard\Config\CorsConfig;
use Spikard\Config\JwtConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\OpenApiConfig;
use Spikard\Config\RateLimitConfig;
use Spikard\Config\ServerConfig;
use Spikard\Config\ServerConfigBuilder;
use Spikard\Config\StaticFilesConfig;

/**
 * Comprehensive tests for ServerConfigBuilder to increase coverage.
 *
 * Tests all builder methods and build process for:
 * - Spikard\Config\ServerConfigBuilder (increase from 72% to 80%+)
 */
final class ServerConfigBuilderTest extends TestCase
{
    // ======================== Default Configuration Tests ========================

    public function testBuilderCreatesDefaultConfig(): void
    {
        $config = ServerConfig::builder()->build();

        $this->assertInstanceOf(ServerConfig::class, $config);
        $this->assertSame('127.0.0.1', $config->host);
        $this->assertSame(8000, $config->port);
        $this->assertSame(1, $config->workers);
        $this->assertTrue($config->enableRequestId);
        $this->assertSame(10485760, $config->maxBodySize);
        $this->assertSame(30, $config->requestTimeout);
        $this->assertTrue($config->gracefulShutdown);
        $this->assertSame(30, $config->shutdownTimeout);
        $this->assertNull($config->compression);
        $this->assertNull($config->rateLimit);
        $this->assertNull($config->cors);
        $this->assertNull($config->staticFiles);
        $this->assertNull($config->jwtAuth);
        $this->assertNull($config->apiKeyAuth);
        $this->assertNull($config->openapi);
        $this->assertNull($config->hooks);
    }

    // ======================== Host Configuration Tests ========================

    public function testWithHostSetsHost(): void
    {
        $config = ServerConfig::builder()
            ->withHost('0.0.0.0')
            ->build();

        $this->assertSame('0.0.0.0', $config->host);
    }

    public function testWithHostOverridesDefault(): void
    {
        $config = ServerConfig::builder()
            ->withHost('localhost')
            ->build();

        $this->assertSame('localhost', $config->host);
    }

    public function testWithHostMultipleCalls(): void
    {
        $config = ServerConfig::builder()
            ->withHost('192.168.1.1')
            ->withHost('10.0.0.1')
            ->build();

        // Last value should win
        $this->assertSame('10.0.0.1', $config->host);
    }

    public function testWithHostVariousValues(): void
    {
        $hosts = ['localhost', '127.0.0.1', '0.0.0.0', '192.168.1.1', 'example.com'];

        foreach ($hosts as $host) {
            $config = ServerConfig::builder()->withHost($host)->build();
            $this->assertSame($host, $config->host);
        }
    }

    // ======================== Port Configuration Tests ========================

    public function testWithPortSetsPort(): void
    {
        $config = ServerConfig::builder()
            ->withPort(9000)
            ->build();

        $this->assertSame(9000, $config->port);
    }

    public function testWithPortOverridesDefault(): void
    {
        $config = ServerConfig::builder()
            ->withPort(3000)
            ->build();

        $this->assertSame(3000, $config->port);
    }

    public function testWithPortVariousValues(): void
    {
        $ports = [80, 443, 3000, 5000, 8080, 9000, 65535];

        foreach ($ports as $port) {
            $config = ServerConfig::builder()->withPort($port)->build();
            $this->assertSame($port, $config->port);
        }
    }

    public function testWithPortMultipleCalls(): void
    {
        $config = ServerConfig::builder()
            ->withPort(3000)
            ->withPort(4000)
            ->build();

        // Last value should win
        $this->assertSame(4000, $config->port);
    }

    // ======================== Workers Configuration Tests ========================

    public function testWithWorkersSetsWorkers(): void
    {
        $config = ServerConfig::builder()
            ->withWorkers(4)
            ->build();

        $this->assertSame(4, $config->workers);
    }

    public function testWithWorkersVariousValues(): void
    {
        $workerCounts = [1, 2, 4, 8, 16, 32];

        foreach ($workerCounts as $count) {
            $config = ServerConfig::builder()->withWorkers($count)->build();
            $this->assertSame($count, $config->workers);
        }
    }

    public function testWithWorkersMultipleCalls(): void
    {
        $config = ServerConfig::builder()
            ->withWorkers(2)
            ->withWorkers(8)
            ->build();

        $this->assertSame(8, $config->workers);
    }

    // ======================== Request ID Configuration Tests ========================

    public function testWithRequestIdEnables(): void
    {
        $config = ServerConfig::builder()
            ->withRequestId(true)
            ->build();

        $this->assertTrue($config->enableRequestId);
    }

    public function testWithRequestIdDisables(): void
    {
        $config = ServerConfig::builder()
            ->withRequestId(false)
            ->build();

        $this->assertFalse($config->enableRequestId);
    }

    public function testWithRequestIdMultipleCalls(): void
    {
        $config = ServerConfig::builder()
            ->withRequestId(true)
            ->withRequestId(false)
            ->build();

        $this->assertFalse($config->enableRequestId);
    }

    // ======================== Max Body Size Configuration Tests ========================

    public function testWithMaxBodySizeSetsSize(): void
    {
        $config = ServerConfig::builder()
            ->withMaxBodySize(52428800) // 50 MB
            ->build();

        $this->assertSame(52428800, $config->maxBodySize);
    }

    public function testWithMaxBodySizeNull(): void
    {
        $config = ServerConfig::builder()
            ->withMaxBodySize(null)
            ->build();

        $this->assertNull($config->maxBodySize);
    }

    public function testWithMaxBodySizeVariousValues(): void
    {
        $sizes = [
            1024,           // 1 KB
            1048576,        // 1 MB
            5242880,        // 5 MB
            10485760,       // 10 MB
            52428800,       // 50 MB
            104857600,      // 100 MB
        ];

        foreach ($sizes as $size) {
            $config = ServerConfig::builder()->withMaxBodySize($size)->build();
            $this->assertSame($size, $config->maxBodySize);
        }
    }

    public function testWithMaxBodySizeMultipleCalls(): void
    {
        $config = ServerConfig::builder()
            ->withMaxBodySize(10485760)
            ->withMaxBodySize(52428800)
            ->build();

        $this->assertSame(52428800, $config->maxBodySize);
    }

    // ======================== Request Timeout Configuration Tests ========================

    public function testWithRequestTimeoutSetsTimeout(): void
    {
        $config = ServerConfig::builder()
            ->withRequestTimeout(60)
            ->build();

        $this->assertSame(60, $config->requestTimeout);
    }

    public function testWithRequestTimeoutNull(): void
    {
        $config = ServerConfig::builder()
            ->withRequestTimeout(null)
            ->build();

        $this->assertNull($config->requestTimeout);
    }

    public function testWithRequestTimeoutVariousValues(): void
    {
        $timeouts = [5, 10, 30, 60, 120, 300];

        foreach ($timeouts as $timeout) {
            $config = ServerConfig::builder()->withRequestTimeout($timeout)->build();
            $this->assertSame($timeout, $config->requestTimeout);
        }
    }

    public function testWithRequestTimeoutMultipleCalls(): void
    {
        $config = ServerConfig::builder()
            ->withRequestTimeout(30)
            ->withRequestTimeout(120)
            ->build();

        $this->assertSame(120, $config->requestTimeout);
    }

    // ======================== Graceful Shutdown Configuration Tests ========================

    public function testWithGracefulShutdownEnables(): void
    {
        $config = ServerConfig::builder()
            ->withGracefulShutdown(true)
            ->build();

        $this->assertTrue($config->gracefulShutdown);
    }

    public function testWithGracefulShutdownDisables(): void
    {
        $config = ServerConfig::builder()
            ->withGracefulShutdown(false)
            ->build();

        $this->assertFalse($config->gracefulShutdown);
    }

    public function testWithGracefulShutdownMultipleCalls(): void
    {
        $config = ServerConfig::builder()
            ->withGracefulShutdown(false)
            ->withGracefulShutdown(true)
            ->build();

        $this->assertTrue($config->gracefulShutdown);
    }

    // ======================== Shutdown Timeout Configuration Tests ========================

    public function testWithShutdownTimeoutSetsTimeout(): void
    {
        $config = ServerConfig::builder()
            ->withShutdownTimeout(60)
            ->build();

        $this->assertSame(60, $config->shutdownTimeout);
    }

    public function testWithShutdownTimeoutVariousValues(): void
    {
        $timeouts = [5, 10, 30, 60, 120, 300];

        foreach ($timeouts as $timeout) {
            $config = ServerConfig::builder()->withShutdownTimeout($timeout)->build();
            $this->assertSame($timeout, $config->shutdownTimeout);
        }
    }

    public function testWithShutdownTimeoutMultipleCalls(): void
    {
        $config = ServerConfig::builder()
            ->withShutdownTimeout(30)
            ->withShutdownTimeout(120)
            ->build();

        $this->assertSame(120, $config->shutdownTimeout);
    }

    // ======================== Middleware Configuration Tests ========================

    public function testWithCompressionSetsConfig(): void
    {
        $compression = new CompressionConfig(gzip: true, brotli: false);
        $config = ServerConfig::builder()
            ->withCompression($compression)
            ->build();

        $this->assertSame($compression, $config->compression);
    }

    public function testWithCompressionMultipleCalls(): void
    {
        $compression1 = new CompressionConfig(gzip: true);
        $compression2 = new CompressionConfig(gzip: false, brotli: true);

        $config = ServerConfig::builder()
            ->withCompression($compression1)
            ->withCompression($compression2)
            ->build();

        $this->assertSame($compression2, $config->compression);
    }

    public function testWithRateLimitSetsConfig(): void
    {
        $rateLimit = new RateLimitConfig(
            perSecond: 100,
            burst: 10
        );
        $config = ServerConfig::builder()
            ->withRateLimit($rateLimit)
            ->build();

        $this->assertSame($rateLimit, $config->rateLimit);
    }

    public function testWithRateLimitMultipleCalls(): void
    {
        $rateLimit1 = new RateLimitConfig(
            perSecond: 100,
            burst: 10
        );
        $rateLimit2 = new RateLimitConfig(
            perSecond: 50,
            burst: 5
        );

        $config = ServerConfig::builder()
            ->withRateLimit($rateLimit1)
            ->withRateLimit($rateLimit2)
            ->build();

        $this->assertSame($rateLimit2, $config->rateLimit);
    }

    public function testWithCorsSetsConfig(): void
    {
        $cors = new CorsConfig(
            allowedOrigins: ['https://example.com'],
            allowedMethods: ['GET', 'POST']
        );
        $config = ServerConfig::builder()
            ->withCors($cors)
            ->build();

        $this->assertSame($cors, $config->cors);
    }

    public function testWithStaticFilesSetsConfig(): void
    {
        $staticFiles = new StaticFilesConfig(
            enabled: true,
            root: './public',
            indexFile: 'index.html',
            cache: false
        );
        $config = ServerConfig::builder()
            ->withStaticFiles($staticFiles)
            ->build();

        $this->assertSame($staticFiles, $config->staticFiles);
    }

    public function testWithJwtAuthSetsConfig(): void
    {
        $jwtAuth = new JwtConfig(
            secret: 'secret_key',
            algorithm: 'HS256'
        );
        $config = ServerConfig::builder()
            ->withJwtAuth($jwtAuth)
            ->build();

        $this->assertSame($jwtAuth, $config->jwtAuth);
    }

    public function testWithApiKeyAuthSetsConfig(): void
    {
        $apiKeyAuth = new ApiKeyConfig(
            headerName: 'X-API-Key',
            keys: ['key1', 'key2']
        );
        $config = ServerConfig::builder()
            ->withApiKeyAuth($apiKeyAuth)
            ->build();

        $this->assertSame($apiKeyAuth, $config->apiKeyAuth);
    }

    public function testWithOpenApiSetsConfig(): void
    {
        $openapi = new OpenApiConfig(
            title: 'My API',
            version: '1.0.0',
            openapiJsonPath: '/openapi.json'
        );
        $config = ServerConfig::builder()
            ->withOpenApi($openapi)
            ->build();

        $this->assertSame($openapi, $config->openapi);
    }

    public function testWithLifecycleHooksSetsHooks(): void
    {
        $hooks = LifecycleHooks::builder()->build();
        $config = ServerConfig::builder()
            ->withLifecycleHooks($hooks)
            ->build();

        $this->assertSame($hooks, $config->hooks);
    }

    // ======================== Fluent Interface Tests ========================

    public function testBuilderReturnsSelfForChaining(): void
    {
        $builder = ServerConfig::builder();

        $result = $builder->withHost('0.0.0.0');
        $this->assertSame($builder, $result);

        $result = $builder->withPort(3000);
        $this->assertSame($builder, $result);

        $result = $builder->withWorkers(4);
        $this->assertSame($builder, $result);
    }

    public function testBuilderChaining(): void
    {
        $config = ServerConfig::builder()
            ->withHost('0.0.0.0')
            ->withPort(3000)
            ->withWorkers(4)
            ->withRequestId(false)
            ->withMaxBodySize(52428800)
            ->withRequestTimeout(60)
            ->withGracefulShutdown(false)
            ->withShutdownTimeout(10)
            ->build();

        $this->assertSame('0.0.0.0', $config->host);
        $this->assertSame(3000, $config->port);
        $this->assertSame(4, $config->workers);
        $this->assertFalse($config->enableRequestId);
        $this->assertSame(52428800, $config->maxBodySize);
        $this->assertSame(60, $config->requestTimeout);
        $this->assertFalse($config->gracefulShutdown);
        $this->assertSame(10, $config->shutdownTimeout);
    }

    public function testComplexBuilderChaining(): void
    {
        $compression = new CompressionConfig(gzip: true, quality: 5);
        $rateLimit = new RateLimitConfig(perSecond: 100, burst: 10);
        $cors = new CorsConfig(allowedOrigins: ['*'], allowedMethods: ['*']);
        $hooks = LifecycleHooks::builder()->build();

        $config = ServerConfig::builder()
            ->withHost('0.0.0.0')
            ->withPort(3000)
            ->withWorkers(8)
            ->withCompression($compression)
            ->withRateLimit($rateLimit)
            ->withCors($cors)
            ->withLifecycleHooks($hooks)
            ->build();

        $this->assertSame('0.0.0.0', $config->host);
        $this->assertSame(3000, $config->port);
        $this->assertSame(8, $config->workers);
        $this->assertSame($compression, $config->compression);
        $this->assertSame($rateLimit, $config->rateLimit);
        $this->assertSame($cors, $config->cors);
        $this->assertSame($hooks, $config->hooks);
    }

    // ======================== Configuration Combination Tests ========================

    public function testAllConfigurationsCanBeSet(): void
    {
        $compression = new CompressionConfig(gzip: true, quality: 4);
        $rateLimit = new RateLimitConfig(perSecond: 100, burst: 10);
        $cors = new CorsConfig(allowedOrigins: ['*'], allowedMethods: ['*']);
        $staticFiles = new StaticFilesConfig(enabled: true, root: './public', indexFile: 'index.html', cache: false);
        $jwtAuth = new JwtConfig(secret: 'key', algorithm: 'HS256');
        $apiKeyAuth = new ApiKeyConfig(headerName: 'X-API-Key', keys: ['key1']);
        $openapi = new OpenApiConfig(title: 'API', version: '1.0.0', openapiJsonPath: '/docs');
        $hooks = LifecycleHooks::builder()->build();

        $config = ServerConfig::builder()
            ->withHost('localhost')
            ->withPort(8000)
            ->withWorkers(4)
            ->withRequestId(true)
            ->withMaxBodySize(10485760)
            ->withRequestTimeout(30)
            ->withGracefulShutdown(true)
            ->withShutdownTimeout(30)
            ->withCompression($compression)
            ->withRateLimit($rateLimit)
            ->withCors($cors)
            ->withStaticFiles($staticFiles)
            ->withJwtAuth($jwtAuth)
            ->withApiKeyAuth($apiKeyAuth)
            ->withOpenApi($openapi)
            ->withLifecycleHooks($hooks)
            ->build();

        $this->assertSame('localhost', $config->host);
        $this->assertSame(8000, $config->port);
        $this->assertSame(4, $config->workers);
        $this->assertTrue($config->enableRequestId);
        $this->assertSame(10485760, $config->maxBodySize);
        $this->assertSame(30, $config->requestTimeout);
        $this->assertTrue($config->gracefulShutdown);
        $this->assertSame(30, $config->shutdownTimeout);
        $this->assertSame($compression, $config->compression);
        $this->assertSame($rateLimit, $config->rateLimit);
        $this->assertSame($cors, $config->cors);
        $this->assertSame($staticFiles, $config->staticFiles);
        $this->assertSame($jwtAuth, $config->jwtAuth);
        $this->assertSame($apiKeyAuth, $config->apiKeyAuth);
        $this->assertSame($openapi, $config->openapi);
        $this->assertSame($hooks, $config->hooks);
    }

    // ======================== Independent Builder Instances ========================

    public function testEachBuilderIsIndependent(): void
    {
        $builder1 = ServerConfig::builder()->withHost('127.0.0.1');
        $builder2 = ServerConfig::builder()->withHost('0.0.0.0');

        $config1 = $builder1->build();
        $config2 = $builder2->build();

        $this->assertSame('127.0.0.1', $config1->host);
        $this->assertSame('0.0.0.0', $config2->host);
    }

    public function testBuilderCanBeReusedAfterBuild(): void
    {
        $builder = ServerConfig::builder()
            ->withHost('localhost')
            ->withPort(3000);

        $config1 = $builder->build();

        // Modify and build again
        $builder->withPort(4000);
        $config2 = $builder->build();

        $this->assertSame(3000, $config1->port);
        $this->assertSame(4000, $config2->port);
    }

    // ======================== Null Middleware Tests ========================

    public function testDefaultNullMiddleware(): void
    {
        $config = ServerConfig::builder()->build();

        $this->assertNull($config->compression);
        $this->assertNull($config->rateLimit);
        $this->assertNull($config->cors);
        $this->assertNull($config->staticFiles);
        $this->assertNull($config->jwtAuth);
        $this->assertNull($config->apiKeyAuth);
        $this->assertNull($config->openapi);
        $this->assertNull($config->hooks);
    }
}
