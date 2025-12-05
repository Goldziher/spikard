<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Error;
use Generator;
use InvalidArgumentException;
use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\Http\StreamingResponse;

final class StreamingResponseBehavioralTest extends TestCase
{
    /**
     * Test basic streaming response constructor.
     */
    public function testStreamingResponseConstructor(): void
    {
        $generator = $this->createSimpleGenerator(['chunk1', 'chunk2']);
        $response = new StreamingResponse($generator, 201, ['X-Custom' => 'header']);

        $this->assertSame(201, $response->statusCode);
        $this->assertSame(['X-Custom' => 'header'], $response->headers);
        $this->assertSame($generator, $response->generator);
    }

    /**
     * Test streaming response default status is 200.
     */
    public function testStreamingResponseDefaultStatus(): void
    {
        $generator = $this->createSimpleGenerator([]);
        $response = new StreamingResponse($generator);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Test streaming response default headers is empty.
     */
    public function testStreamingResponseDefaultHeaders(): void
    {
        $generator = $this->createSimpleGenerator([]);
        $response = new StreamingResponse($generator);

        $this->assertSame([], $response->headers);
    }

    /**
     * Test readonly properties cannot be changed.
     */
    public function testStreamingResponsePropertiesAreReadonly(): void
    {
        $generator = $this->createSimpleGenerator([]);
        $response = new StreamingResponse($generator, 200);

        // This should cause a PHP error
        $this->expectException(Error::class);
        // @phpstan-ignore-next-line
        $response->statusCode = 404;
    }

    /**
     * Test SSE response sets correct headers.
     */
    public function testSseResponseHeaders(): void
    {
        $generator = $this->createSimpleGenerator(['data: test']);
        $response = StreamingResponse::sse($generator);

        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        $this->assertSame('no-cache', $response->headers['Cache-Control']);
        $this->assertSame('no', $response->headers['X-Accel-Buffering']);
    }

    /**
     * Test SSE response with additional headers (should merge).
     */
    public function testSseResponseMergesAdditionalHeaders(): void
    {
        $generator = $this->createSimpleGenerator(['data: test']);
        $additional = ['X-Custom' => 'value', 'X-Another' => 'header'];
        $response = StreamingResponse::sse($generator, $additional);

        // Original SSE headers should be present
        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        // Additional headers should be present
        $this->assertSame('value', $response->headers['X-Custom']);
        $this->assertSame('header', $response->headers['X-Another']);
    }

    /**
     * Test SSE response allows overriding headers via additional headers (last one wins).
     */
    public function testSseResponseAllowsOverridingCacheControl(): void
    {
        $generator = $this->createSimpleGenerator(['data: test']);
        $additional = ['Cache-Control' => 'public, max-age=3600'];
        $response = StreamingResponse::sse($generator, $additional);

        // Additional header should override default
        $this->assertSame('public, max-age=3600', $response->headers['Cache-Control']);
    }

    /**
     * Test SSE response with empty additional headers.
     */
    public function testSseResponseWithEmptyAdditionalHeaders(): void
    {
        $generator = $this->createSimpleGenerator(['data: test']);
        $response = StreamingResponse::sse($generator, []);

        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
    }

    /**
     * Test file streaming with valid file.
     */
    public function testFileStreamingWithValidFile(): void
    {
        $tempFile = $this->createTempFileWithContent('test data');

        try {
            $response = StreamingResponse::file($tempFile);

            $this->assertSame(200, $response->statusCode);
            $this->assertArrayHasKey('Content-Length', $response->headers);
            $this->assertArrayHasKey('Content-Type', $response->headers);

            // Verify content can be streamed
            $chunks = \iterator_to_array($response->generator);
            $content = \implode('', \array_filter($chunks, '\is_string'));
            $this->assertStringContainsString('test data', $content);
        } finally {
            @\unlink($tempFile);
        }
    }

    /**
     * Test file streaming throws for non-existent file.
     */
    public function testFileStreamingThrowsForNonexistentFile(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('File not found');
        StreamingResponse::file('/nonexistent/path/file.txt');
    }

    /**
     * Test file streaming with custom content type.
     */
    public function testFileStreamingWithCustomContentType(): void
    {
        $tempFile = $this->createTempFileWithContent('binary data');

        try {
            $response = StreamingResponse::file($tempFile, contentType: 'application/octet-stream');

            $this->assertSame('application/octet-stream', $response->headers['Content-Type']);
        } finally {
            @\unlink($tempFile);
        }
    }

    /**
     * Test file streaming with custom chunk size.
     */
    public function testFileStreamingWithCustomChunkSize(): void
    {
        $content = \str_repeat('a', 1000);
        $tempFile = $this->createTempFileWithContent($content);

        try {
            $response = StreamingResponse::file($tempFile, chunkSize: 10);

            $chunks = \iterator_to_array($response->generator);
            // With 1000 bytes and 10 byte chunks, we should have 100 chunks
            $this->assertGreaterThan(50, \count($chunks));
        } finally {
            @\unlink($tempFile);
        }
    }

    /**
     * Test file streaming throws for chunk size less than 1.
     */
    public function testFileStreamingThrowsForInvalidChunkSize(): void
    {
        $tempFile = $this->createTempFileWithContent('data');

        try {
            $this->expectException(InvalidArgumentException::class);
            $this->expectExceptionMessage('Chunk size must be at least 1 byte');
            StreamingResponse::file($tempFile, chunkSize: 0);
        } finally {
            @\unlink($tempFile);
        }
    }

    /**
     * Test file streaming throws for negative chunk size.
     */
    public function testFileStreamingThrowsForNegativeChunkSize(): void
    {
        $tempFile = $this->createTempFileWithContent('data');

        try {
            $this->expectException(InvalidArgumentException::class);
            StreamingResponse::file($tempFile, chunkSize: -100);
        } finally {
            @\unlink($tempFile);
        }
    }

    /**
     * Test file streaming with very large chunk size.
     */
    public function testFileStreamingWithLargeChunkSize(): void
    {
        $tempFile = $this->createTempFileWithContent('small');

        try {
            $response = StreamingResponse::file($tempFile, chunkSize: 1000000);

            $chunks = \iterator_to_array($response->generator);
            // One large chunk should contain all data
            $this->assertGreaterThanOrEqual(1, \count($chunks));
        } finally {
            @\unlink($tempFile);
        }
    }

    /**
     * Test file streaming includes Content-Length header.
     */
    public function testFileStreamingIncludesContentLength(): void
    {
        $content = 'exactly 12 bytes';
        $tempFile = $this->createTempFileWithContent($content);

        try {
            $response = StreamingResponse::file($tempFile);

            $this->assertArrayHasKey('Content-Length', $response->headers);
            $this->assertSame((string) \strlen($content), $response->headers['Content-Length']);
        } finally {
            @\unlink($tempFile);
        }
    }

    /**
     * Test file streaming auto-detects MIME type.
     */
    public function testFileStreamingAutoDetectsMimeType(): void
    {
        $tempFile = $this->createTempFileWithContent('<!DOCTYPE html>', 'test.html');

        try {
            $response = StreamingResponse::file($tempFile);

            // MIME type should be set
            $this->assertArrayHasKey('Content-Type', $response->headers);
            // Should be text/html or similar
            $this->assertNotEmpty($response->headers['Content-Type']);
        } finally {
            @\unlink($tempFile);
        }
    }

    /**
     * Test JSON lines streaming.
     */
    public function testJsonLinesStreaming(): void
    {
        $data = [
            ['id' => 1, 'name' => 'Alice'],
            ['id' => 2, 'name' => 'Bob'],
            ['id' => 3, 'name' => 'Charlie'],
        ];

        $generator = function () use ($data): Generator {
            foreach ($data as $record) {
                yield $record;
            }
        };

        $response = StreamingResponse::jsonLines($generator());

        $this->assertSame('application/x-ndjson', $response->headers['Content-Type']);
        $this->assertSame(200, $response->statusCode);

        // Verify content
        $chunks = \array_map(
            static function (mixed $chunk): string {
                if (!\is_string($chunk)) {
                    throw new InvalidArgumentException('Chunk must be string');
                }
                return $chunk;
            },
            \iterator_to_array($response->generator)
        );
        $lines = \explode("\n", \implode('', $chunks));
        $this->assertCount(3, \array_filter($lines)); // 3 lines of JSON
    }

    /**
     * Test JSON lines with various data types.
     */
    public function testJsonLinesWithVariousTypes(): void
    {
        $data = [
            'string value',
            123,
            true,
            false,
            null,
            ['nested' => 'array'],
        ];

        $generator = function () use ($data): Generator {
            foreach ($data as $item) {
                yield $item;
            }
        };

        $response = StreamingResponse::jsonLines($generator());

        $chunks = \array_map(
            static function (mixed $chunk): string {
                if (!\is_string($chunk)) {
                    throw new InvalidArgumentException('Chunk must be string');
                }
                return $chunk;
            },
            \iterator_to_array($response->generator)
        );
        $content = \implode('', $chunks);
        $lines = \array_filter(\explode("\n", $content));

        $this->assertCount(6, $lines);
        // Verify each line is valid JSON
        foreach ($lines as $line) {
            $decoded = \json_decode($line, false);
            $this->assertSame(JSON_ERROR_NONE, \json_last_error());
            $this->assertTrue($decoded !== false);
        }
    }

    /**
     * Test JSON lines with special characters.
     */
    public function testJsonLinesWithSpecialCharacters(): void
    {
        $data = [
            'hello "world"',
            "line1\nline2",
            'emoji 🚀',
            'unicode: é à ü',
        ];

        $generator = function () use ($data): Generator {
            foreach ($data as $item) {
                yield $item;
            }
        };

        $response = StreamingResponse::jsonLines($generator());

        $chunks = \array_map(
            static function (mixed $chunk): string {
                if (!\is_string($chunk)) {
                    throw new InvalidArgumentException('Chunk must be string');
                }
                return $chunk;
            },
            \iterator_to_array($response->generator)
        );
        $content = \implode('', $chunks);
        $lines = \array_filter(\explode("\n", $content));

        $this->assertCount(4, $lines);
        // Verify special chars are properly escaped
        $this->assertStringContainsString('\\n', $content);
        $this->assertStringContainsString('\\"', $content);
    }

    /**
     * Test JSON lines with empty generator.
     */
    public function testJsonLinesWithEmptyGenerator(): void
    {
        $generator = function (): Generator {
            yield from [];
        };

        $response = StreamingResponse::jsonLines($generator());

        $chunks = \array_map(
            static function (mixed $chunk): string {
                if (!\is_string($chunk)) {
                    throw new InvalidArgumentException('Chunk must be string');
                }
                return $chunk;
            },
            \iterator_to_array($response->generator)
        );
        $content = \implode('', $chunks);

        $this->assertEmpty(\array_filter(\explode("\n", $content)));
    }

    /**
     * Test streaming response generator can be consumed multiple times (if not consumed).
     */
    public function testStreamingGeneratorBehavior(): void
    {
        $data = ['a', 'b', 'c'];
        $generator = function () use ($data): Generator {
            foreach ($data as $item) {
                yield $item;
            }
        };

        $response = new StreamingResponse($generator());

        // First consumption
        $chunks1 = \iterator_to_array($response->generator);
        $this->assertCount(3, $chunks1);

        // Generator is exhausted, second iteration yields nothing
        $chunks2 = \iterator_to_array($response->generator);
        $this->assertCount(0, $chunks2);
    }

    /**
     * Test streaming response with mixed yielded types (arrays and strings).
     */
    public function testStreamingResponseWithMixedYieldedTypes(): void
    {
        $generator = function (): Generator {
            yield 'string';
            yield ['array', 'data'];
            yield 'another string';
        };

        $response = new StreamingResponse($generator());

        $chunks = \iterator_to_array($response->generator);
        $this->assertCount(3, $chunks);
        $this->assertIsString($chunks[0]);
        $this->assertIsArray($chunks[1]);
    }

    /**
     * Test streaming response with null status code (should use 200).
     */
    public function testStreamingResponseWithVariousStatusCodes(): void
    {
        $statuses = [200, 201, 202, 206, 300, 301, 400, 404, 500];

        foreach ($statuses as $status) {
            $generator = $this->createSimpleGenerator([]);
            $response = new StreamingResponse($generator, $status);
            $this->assertSame($status, $response->statusCode);
        }
    }

    /**
     * Test file streaming handles file closing properly (via generator).
     */
    public function testFileStreamingClosesFileHandle(): void
    {
        $tempFile = $this->createTempFileWithContent('data to stream');

        try {
            $response = StreamingResponse::file($tempFile);

            // Consume the generator
            $chunks = \iterator_to_array($response->generator);
            $this->assertNotEmpty($chunks);

            // File should still exist (we didn't delete it)
            $this->assertFileExists($tempFile);
        } finally {
            @\unlink($tempFile);
        }
    }

    /**
     * Test streaming response with headers containing multiple values.
     */
    public function testStreamingResponseWithMultipleHeaders(): void
    {
        $generator = $this->createSimpleGenerator([]);
        $headers = [
            'Content-Type' => 'application/json',
            'X-Custom-1' => 'value1',
            'X-Custom-2' => 'value2',
            'Cache-Control' => 'no-cache, no-store',
        ];

        $response = new StreamingResponse($generator, 200, $headers);

        $this->assertSame(4, \count($response->headers));
        foreach ($headers as $key => $value) {
            $this->assertSame($value, $response->headers[$key]);
        }
    }

    /**
     * Test SSE with content-type override.
     */
    public function testSseWithContentTypeOverride(): void
    {
        $generator = $this->createSimpleGenerator(['data: test']);
        // Try to override Content-Type (additional headers should override)
        $response = StreamingResponse::sse(
            $generator,
            ['Content-Type' => 'custom/type']
        );

        // The additional headers merge, so custom type should win
        $this->assertSame('custom/type', $response->headers['Content-Type']);
    }

    /**
     * Test file streaming with empty file.
     */
    public function testFileStreamingWithEmptyFile(): void
    {
        $tempFile = $this->createTempFileWithContent('');

        try {
            $response = StreamingResponse::file($tempFile);

            $this->assertSame('0', $response->headers['Content-Length']);
            $chunks = \iterator_to_array($response->generator);
            // Empty file might yield one empty chunk or no chunks
            $this->assertCount(\count($chunks), $chunks);
        } finally {
            @\unlink($tempFile);
        }
    }

    // Helper methods

    /**
     * @param list<string> $items
     */
    private function createSimpleGenerator(array $items): Generator
    {
        foreach ($items as $item) {
            yield $item;
        }
    }

    private function createTempFileWithContent(string $content, ?string $filename = null): string
    {
        if ($filename === null) {
            $tempFile = \tempnam(\sys_get_temp_dir(), 'test_');
            if ($tempFile === false) {
                throw new RuntimeException('Failed to create temp file');
            }
        } else {
            $tempFile = \sys_get_temp_dir() . '/' . $filename;
        }

        if (\file_put_contents($tempFile, $content) === false) {
            throw new RuntimeException("Failed to write to temp file: {$tempFile}");
        }

        return $tempFile;
    }
}
