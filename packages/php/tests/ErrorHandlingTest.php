<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Generator;
use InvalidArgumentException;
use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\Background\BackgroundTask;
use Spikard\Config\CompressionConfig;
use Spikard\Config\CorsConfig;
use Spikard\Config\RateLimitConfig;
use Spikard\Config\ServerConfig;
use Spikard\Config\StaticFilesConfig;
use Spikard\DI\DependencyContainer;
use Spikard\DI\DependencyContainerBuilder;
use Spikard\DI\Provide;
use Spikard\Http\Params\Body;
use Spikard\Http\Params\Cookie;
use Spikard\Http\Params\Header;
use Spikard\Http\Params\Path;
use Spikard\Http\Params\Query;
use Spikard\Http\StreamingResponse;

final class ErrorHandlingTest extends TestCase
{
    // Parameter validation error tests
    public function testQueryParamCannotHaveBothDefaultAndFactory(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('Cannot specify both');

        new Query(
            default: 'value',
            defaultFactory: fn () => 'factory_value'
        );
    }

    public function testPathParamCannotHaveBothDefaultAndFactory(): void
    {
        $this->expectException(InvalidArgumentException::class);

        new Path(
            default: 'value',
            defaultFactory: fn () => 'factory_value'
        );
    }

    public function testHeaderParamCannotHaveBothDefaultAndFactory(): void
    {
        $this->expectException(InvalidArgumentException::class);

        new Header(
            default: 'value',
            defaultFactory: fn () => 'factory_value'
        );
    }

    public function testCookieParamCannotHaveBothDefaultAndFactory(): void
    {
        $this->expectException(InvalidArgumentException::class);

        new Cookie(
            default: 'value',
            defaultFactory: fn () => 'factory_value'
        );
    }

    public function testBodyParamCannotHaveBothDefaultAndFactory(): void
    {
        $this->expectException(InvalidArgumentException::class);

        new Body(
            default: ['value'],
            defaultFactory: fn () => ['factory_value']
        );
    }

    // StreamingResponse validation tests
    public function testStreamingResponseFileThrowsForNonexistentFile(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('File not found');

        StreamingResponse::file('/nonexistent/path/to/file.txt');
    }

    public function testStreamingResponseFileThrowsForZeroChunkSize(): void
    {
        // Create a temporary file
        $tmpFile = tempnam(sys_get_temp_dir(), 'test');
        $this->assertTrue(file_put_contents($tmpFile, 'test content') !== false);

        try {
            $this->expectException(InvalidArgumentException::class);
            $this->expectExceptionMessage('Chunk size must be at least 1 byte');

            StreamingResponse::file($tmpFile, chunkSize: 0);
        } finally {
            @unlink($tmpFile);
        }
    }

    public function testStreamingResponseFileThrowsForNegativeChunkSize(): void
    {
        // Create a temporary file
        $tmpFile = tempnam(sys_get_temp_dir(), 'test');
        $this->assertTrue(file_put_contents($tmpFile, 'test content') !== false);

        try {
            $this->expectException(InvalidArgumentException::class);
            $this->expectExceptionMessage('Chunk size must be at least 1 byte');

            StreamingResponse::file($tmpFile, chunkSize: -1);
        } finally {
            @unlink($tmpFile);
        }
    }

    // Dependency container error tests
    public function testContainerBuilderWithBothValueAndFactory(): void
    {
        // Should allow builder to add both values and factories independently
        $container = DependencyContainer::builder()
            ->provideValue('value_dep', 'value')
            ->provideFactory('factory_dep', Provide::factory(fn () => 'factory'))
            ->build();

        $deps = $container->getDependencies();

        $this->assertArrayHasKey('value_dep', $deps);
        $this->assertArrayHasKey('factory_dep', $deps);
    }

    // BackgroundTask with invalid callables
    public function testBackgroundTaskWithCallableThrowingException(): void
    {
        // The mock implementation executes immediately, so this will throw
        $this->expectException(\Exception::class);
        $this->expectExceptionMessage('Test error');

        BackgroundTask::run(function (): void {
            throw new \Exception('Test error');
        });
    }

    public function testBackgroundTaskWithCallableThrowingRuntimeException(): void
    {
        $this->expectException(RuntimeException::class);

        BackgroundTask::run(function (): void {
            throw new RuntimeException('Runtime error');
        });
    }

    // Server configuration edge cases that might cause issues
    public function testServerConfigBuilderReturnsReference(): void
    {
        // ServerConfig builder returns the same builder instance (not immutable), allowing chaining
        $builder = ServerConfig::builder();
        $result1 = $builder->withPort(3000);
        $result2 = $result1->withWorkers(4);

        // All return the same instance
        $this->assertSame($builder, $result1);
        $this->assertSame($builder, $result2);

        $config = $builder->build();
        $this->assertSame(3000, $config->port);
        $this->assertSame(4, $config->workers);
    }

    public function testCompressionConfigWithGzipAndBrotli(): void
    {
        $config = new CompressionConfig(gzip: true, brotli: true);

        $this->assertTrue($config->gzip);
        $this->assertTrue($config->brotli);
    }

    public function testCorsConfigCanBeDisabled(): void
    {
        $config = new CorsConfig(enabled: false);

        // Verify it's in the disabled state
        $this->assertFalse($config->enabled);
    }

    public function testRateLimitConfigWithZeroPerSecond(): void
    {
        $config = new RateLimitConfig(perSecond: 0, burst: 0);

        $this->assertSame(0, $config->perSecond);
    }

    public function testStaticFilesConfigWithIndexFile(): void
    {
        $config = new StaticFilesConfig(
            enabled: true,
            root: '/public',
            indexFile: 'index.html'
        );

        $this->assertSame('index.html', $config->indexFile);
    }

    // Provide factory error tests
    public function testProvidedFactoryWithInvalidDependency(): void
    {
        // Creating with non-existent dependencies should not throw during Provide creation
        $factory = Provide::factory(
            fn (string $service) => 'result',
            dependsOn: ['non_existent_service']
        );

        // But the factory itself is created successfully
        $this->assertInstanceOf(Provide::class, $factory);
    }

    // Type safety tests - ensure type preservation in error scenarios
    public function testQueryParamWithComplexSchemaAndZeroDefault(): void
    {
        $schema = [
            'type' => 'integer',
            'minimum' => 0,
            'maximum' => 100,
        ];
        $query = new Query(default: 0, schema: $schema);

        $this->assertSame(0, $query->getDefault());
        $this->assertSame($schema, $query->getSchema());
    }

    public function testPathParamWithSchemaAndEmptyStringDefault(): void
    {
        $schema = ['type' => 'string', 'minLength' => 1];
        // Empty string should still be allowed as default, even if schema says minLength: 1
        $path = new Path(default: '', schema: $schema);

        $this->assertSame('', $path->getDefault());
    }

    // Multiple operations in sequence to catch state issues
    public function testMultipleContainerCreationsWithSameDependencies(): void
    {
        $deps = ['service' => 'instance'];

        $container1 = new DependencyContainer($deps);
        $container2 = new DependencyContainer($deps);

        // Both should have the same dependencies
        $this->assertSame($deps, $container1->getDependencies());
        $this->assertSame($deps, $container2->getDependencies());

        // But they are separate instances
        $this->assertNotSame($container1, $container2);
    }

    public function testMultipleBackgroundTasksInSequence(): void
    {
        $results = [];

        BackgroundTask::run(function () use (&$results): void {
            $results[] = 1;
        });

        BackgroundTask::run(function () use (&$results): void {
            $results[] = 2;
        });

        BackgroundTask::run(function () use (&$results): void {
            $results[] = 3;
        });

        // In mock implementation (synchronous), all should execute
        $this->assertSame([1, 2, 3], $results);
    }

    // Edge case: Ensure params can be reused
    public function testParamObjectsCanBeReusedMultipleTimes(): void
    {
        $query = new Query(default: 'value');

        $val1 = $query->getDefault();
        $val2 = $query->getDefault();
        $val3 = $query();

        $this->assertSame('value', $val1);
        $this->assertSame('value', $val2);
        $this->assertSame('value', $val3);
    }

    // Edge case: Factory functions with side effects
    public function testParameterFactoryIsCalledEachTime(): void
    {
        $callCount = 0;
        $query = new Query(defaultFactory: function () use (&$callCount): array {
            $callCount++;
            return ['count' => $callCount];
        });

        $val1 = $query->getDefault();
        $val2 = $query->getDefault();
        $val3 = $query->getDefault();

        // Factory should be called 3 times
        $this->assertSame(3, $callCount);
        $this->assertSame(['count' => 1], $val1);
        $this->assertSame(['count' => 2], $val2);
        $this->assertSame(['count' => 3], $val3);
    }

    // Streaming response error handling
    public function testStreamingResponseFileWithReadableFile(): void
    {
        $tmpFile = tempnam(sys_get_temp_dir(), 'stream_test');
        $testContent = 'Test streaming content with special chars: Ñ, 中文, Emoji 🚀';
        $this->assertTrue(file_put_contents($tmpFile, $testContent) !== false);

        try {
            $response = StreamingResponse::file($tmpFile, chunkSize: 8192, contentType: 'text/plain');
            $this->assertSame(200, $response->statusCode);
            // File size header should be set
            $this->assertArrayHasKey('Content-Length', $response->headers);
        } finally {
            @unlink($tmpFile);
        }
    }

    public function testStreamingResponseJsonLinesWithGeneratorYieldingVariousTypes(): void
    {
        $gen = (function (): Generator {
            yield ['string' => 'text', 'number' => 42, 'bool' => true, 'null' => null];
            yield [];
            yield ['nested' => ['deep' => ['value']]];
        })();

        $response = StreamingResponse::jsonLines($gen);
        $this->assertSame('application/x-ndjson', $response->headers['Content-Type']);
    }
}
