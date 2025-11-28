<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Config\CompressionConfig;
use Spikard\Config\CorsConfig;
use Spikard\Config\RateLimitConfig;
use Spikard\Config\ServerConfig;
use Spikard\Config\StaticFilesConfig;

final class ConfigTest extends TestCase
{
    // ServerConfig tests
    public function testServerConfigDefaults(): void
    {
        $config = new ServerConfig();

        $this->assertSame('127.0.0.1', $config->host);
        $this->assertSame(8000, $config->port);
        $this->assertSame(1, $config->workers);
        $this->assertTrue($config->enableRequestId);
        $this->assertSame(10485760, $config->maxBodySize); // 10 MB
        $this->assertSame(30, $config->requestTimeout);
        $this->assertTrue($config->gracefulShutdown);
        $this->assertSame(30, $config->shutdownTimeout);
        $this->assertNull($config->compression);
        $this->assertNull($config->rateLimit);
        $this->assertNull($config->cors);
        $this->assertNull($config->staticFiles);
    }

    public function testServerConfigBuilder(): void
    {
        $config = ServerConfig::builder()
            ->withHost('0.0.0.0')
            ->withPort(3000)
            ->withWorkers(4)
            ->build();

        $this->assertSame('0.0.0.0', $config->host);
        $this->assertSame(3000, $config->port);
        $this->assertSame(4, $config->workers);
    }

    public function testServerConfigBuilderChaining(): void
    {
        $builder = ServerConfig::builder();
        $result1 = $builder->withHost('localhost');
        $result2 = $result1->withPort(9000);

        $this->assertSame($builder, $result1);
        $this->assertSame($builder, $result2);
    }

    public function testServerConfigBuilderWithCompression(): void
    {
        $compression = new CompressionConfig(gzip: true, brotli: true);
        $config = ServerConfig::builder()
            ->withCompression($compression)
            ->build();

        $this->assertSame($compression, $config->compression);
    }

    public function testServerConfigBuilderWithRateLimit(): void
    {
        $rateLimit = new RateLimitConfig(perSecond: 100, burst: 10);
        $config = ServerConfig::builder()
            ->withRateLimit($rateLimit)
            ->build();

        $this->assertSame($rateLimit, $config->rateLimit);
    }

    public function testServerConfigBuilderWithCors(): void
    {
        $cors = new CorsConfig(enabled: true);
        $config = ServerConfig::builder()
            ->withCors($cors)
            ->build();

        $this->assertSame($cors, $config->cors);
    }

    public function testServerConfigBuilderWithStaticFiles(): void
    {
        $staticFiles = new StaticFilesConfig(enabled: true, root: '/public');
        $config = ServerConfig::builder()
            ->withStaticFiles($staticFiles)
            ->build();

        $this->assertSame($staticFiles, $config->staticFiles);
    }

    public function testServerConfigWithCustomValues(): void
    {
        $config = new ServerConfig(
            host: '192.168.1.1',
            port: 4000,
            workers: 8,
            enableRequestId: false,
            maxBodySize: 5242880, // 5 MB
            requestTimeout: 60,
            gracefulShutdown: false,
            shutdownTimeout: 10
        );

        $this->assertSame('192.168.1.1', $config->host);
        $this->assertSame(4000, $config->port);
        $this->assertSame(8, $config->workers);
        $this->assertFalse($config->enableRequestId);
        $this->assertSame(5242880, $config->maxBodySize);
        $this->assertSame(60, $config->requestTimeout);
        $this->assertFalse($config->gracefulShutdown);
        $this->assertSame(10, $config->shutdownTimeout);
    }

    // CorsConfig tests
    public function testCorsConfigDefaults(): void
    {
        $cors = new CorsConfig();

        $this->assertTrue($cors->enabled);
        $this->assertSame(['*'], $cors->allowedOrigins);
        $this->assertSame(['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'], $cors->allowedMethods);
        $this->assertSame(['*'], $cors->allowedHeaders);
        $this->assertSame([], $cors->exposedHeaders);
        $this->assertTrue($cors->allowCredentials);
        $this->assertSame(600, $cors->maxAgeSeconds);
    }

    public function testCorsConfigCustomValues(): void
    {
        $cors = new CorsConfig(
            enabled: true,
            allowedOrigins: ['https://example.com', 'https://app.example.com'],
            allowedMethods: ['GET', 'POST'],
            allowedHeaders: ['Content-Type', 'Authorization'],
            exposedHeaders: ['X-Total-Count'],
            allowCredentials: false,
            maxAgeSeconds: 3600
        );

        $this->assertTrue($cors->enabled);
        $this->assertSame(['https://example.com', 'https://app.example.com'], $cors->allowedOrigins);
        $this->assertSame(['GET', 'POST'], $cors->allowedMethods);
        $this->assertSame(['Content-Type', 'Authorization'], $cors->allowedHeaders);
        $this->assertSame(['X-Total-Count'], $cors->exposedHeaders);
        $this->assertFalse($cors->allowCredentials);
        $this->assertSame(3600, $cors->maxAgeSeconds);
    }

    public function testCorsConfigDisabled(): void
    {
        $cors = new CorsConfig(enabled: false);

        $this->assertFalse($cors->enabled);
    }

    // CompressionConfig tests
    public function testCompressionConfigDefaults(): void
    {
        $compression = new CompressionConfig();

        $this->assertNull($compression->gzip);
        $this->assertNull($compression->brotli);
        $this->assertNull($compression->minSize);
        $this->assertNull($compression->quality);
    }

    public function testCompressionConfigWithGzip(): void
    {
        $compression = new CompressionConfig(gzip: true);

        $this->assertTrue($compression->gzip);
        $this->assertNull($compression->brotli);
    }

    public function testCompressionConfigWithBrotli(): void
    {
        $compression = new CompressionConfig(brotli: true);

        $this->assertTrue($compression->brotli);
        $this->assertNull($compression->gzip);
    }

    public function testCompressionConfigWithAllOptions(): void
    {
        $compression = new CompressionConfig(
            gzip: true,
            brotli: true,
            minSize: 1024,
            quality: 6
        );

        $this->assertTrue($compression->gzip);
        $this->assertTrue($compression->brotli);
        $this->assertSame(1024, $compression->minSize);
        $this->assertSame(6, $compression->quality);
    }

    public function testCompressionConfigDisabled(): void
    {
        $compression = new CompressionConfig(gzip: false, brotli: false);

        $this->assertFalse($compression->gzip);
        $this->assertFalse($compression->brotli);
    }

    // RateLimitConfig tests
    public function testRateLimitConfig(): void
    {
        $rateLimit = new RateLimitConfig(perSecond: 100, burst: 10);

        $this->assertSame(100, $rateLimit->perSecond);
        $this->assertSame(10, $rateLimit->burst);
        $this->assertNull($rateLimit->ipBased);
    }

    public function testRateLimitConfigWithIpBased(): void
    {
        $rateLimit = new RateLimitConfig(perSecond: 50, burst: 5, ipBased: true);

        $this->assertSame(50, $rateLimit->perSecond);
        $this->assertSame(5, $rateLimit->burst);
        $this->assertTrue($rateLimit->ipBased);
    }

    public function testRateLimitConfigIpBasedFalse(): void
    {
        $rateLimit = new RateLimitConfig(perSecond: 1000, burst: 100, ipBased: false);

        $this->assertFalse($rateLimit->ipBased);
    }

    public function testRateLimitConfigHighLimits(): void
    {
        $rateLimit = new RateLimitConfig(perSecond: 10000, burst: 1000);

        $this->assertSame(10000, $rateLimit->perSecond);
        $this->assertSame(1000, $rateLimit->burst);
    }

    // StaticFilesConfig tests
    public function testStaticFilesConfigDefaults(): void
    {
        $staticFiles = new StaticFilesConfig();

        $this->assertFalse($staticFiles->enabled);
        $this->assertNull($staticFiles->root);
        $this->assertNull($staticFiles->indexFile);
        $this->assertTrue($staticFiles->cache); // Default is true
    }

    public function testStaticFilesConfigEnabled(): void
    {
        $staticFiles = new StaticFilesConfig(
            enabled: true,
            root: '/var/www/public'
        );

        $this->assertTrue($staticFiles->enabled);
        $this->assertSame('/var/www/public', $staticFiles->root);
    }

    public function testStaticFilesConfigWithIndex(): void
    {
        $staticFiles = new StaticFilesConfig(
            enabled: true,
            root: '/public',
            indexFile: 'index.html'
        );

        $this->assertSame('index.html', $staticFiles->indexFile);
    }

    public function testStaticFilesConfigWithCache(): void
    {
        $staticFiles = new StaticFilesConfig(
            enabled: true,
            root: '/static',
            cache: true
        );

        $this->assertTrue($staticFiles->cache);
    }

    public function testStaticFilesConfigAllOptions(): void
    {
        $staticFiles = new StaticFilesConfig(
            enabled: true,
            root: '/app/public',
            indexFile: 'home.html',
            cache: true
        );

        $this->assertTrue($staticFiles->enabled);
        $this->assertSame('/app/public', $staticFiles->root);
        $this->assertSame('home.html', $staticFiles->indexFile);
        $this->assertTrue($staticFiles->cache);
    }

    // Integration tests
    public function testServerConfigWithAllMiddleware(): void
    {
        $compression = new CompressionConfig(gzip: true, brotli: true);
        $rateLimit = new RateLimitConfig(perSecond: 100, burst: 10);
        $cors = new CorsConfig(enabled: true);
        $staticFiles = new StaticFilesConfig(enabled: true, root: '/public');

        $config = new ServerConfig(
            host: '0.0.0.0',
            port: 8080,
            workers: 4,
            compression: $compression,
            rateLimit: $rateLimit,
            cors: $cors,
            staticFiles: $staticFiles
        );

        $this->assertSame('0.0.0.0', $config->host);
        $this->assertSame(8080, $config->port);
        $this->assertSame($compression, $config->compression);
        $this->assertSame($rateLimit, $config->rateLimit);
        $this->assertSame($cors, $config->cors);
        $this->assertSame($staticFiles, $config->staticFiles);
    }

    public function testServerConfigBuilderComplex(): void
    {
        $config = ServerConfig::builder()
            ->withHost('0.0.0.0')
            ->withPort(9000)
            ->withWorkers(8)
            ->withCompression(new CompressionConfig(gzip: true))
            ->withRateLimit(new RateLimitConfig(perSecond: 1000, burst: 100))
            ->withCors(new CorsConfig(allowedOrigins: ['https://example.com']))
            ->build();

        $this->assertSame('0.0.0.0', $config->host);
        $this->assertSame(9000, $config->port);
        $this->assertSame(8, $config->workers);
        $this->assertNotNull($config->compression);
        $this->assertNotNull($config->rateLimit);
        $this->assertNotNull($config->cors);
    }
}
