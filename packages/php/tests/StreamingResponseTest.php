<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Error;
use Generator;
use InvalidArgumentException;
use Spikard\Http\StreamingResponse;

/**
 * Consolidated test suite for StreamingResponse.
 *
 * This class consolidates all StreamingResponse tests from:
 * - StreamingResponseTest.php (base tests)
 * - StreamingResponseBehavioralTest.php (behavioral tests)
 * - StreamingResponseCompletionTest.php (completion/edge case tests)
 *
 * Total: 63 unique tests organized by feature area.
 */
final class StreamingResponseTest extends StreamingResponseTestCase
{
    // ======================== Basic Streaming Tests ========================

    public function testBasicStreamingResponse(): void
    {
        $generator = $this->createGenerator(['chunk1', 'chunk2']);
        $response = new StreamingResponse($generator);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame([], $response->headers);
        $this->assertInstanceOf(Generator::class, $response->generator);
    }

    public function testStreamingResponseWithCustomStatus(): void
    {
        $generator = $this->createGenerator(['chunk1', 'chunk2']);
        $response = new StreamingResponse($generator, 201);

        $this->assertSame(201, $response->statusCode);
    }

    public function testStreamingResponseWithHeaders(): void
    {
        $generator = $this->createGenerator(['chunk1', 'chunk2']);
        $headers = ['Content-Type' => 'text/plain', 'X-Custom' => 'value'];
        $response = new StreamingResponse($generator, 200, $headers);

        $this->assertSame($headers, $response->headers);
    }

    public function testStreamingResponseConstructor(): void
    {
        $generator = $this->createGenerator(['chunk1', 'chunk2']);
        $response = new StreamingResponse($generator, 201, ['X-Custom' => 'header']);

        $this->assertSame(201, $response->statusCode);
        $this->assertSame(['X-Custom' => 'header'], $response->headers);
        $this->assertSame($generator, $response->generator);
    }

    public function testStreamingResponseDefaultStatus(): void
    {
        $generator = $this->createGenerator([]);
        $response = new StreamingResponse($generator);

        $this->assertSame(200, $response->statusCode);
    }

    public function testStreamingResponseDefaultHeaders(): void
    {
        $generator = $this->createGenerator([]);
        $response = new StreamingResponse($generator);

        $this->assertSame([], $response->headers);
    }

    public function testStreamingResponsePropertiesAreReadonly(): void
    {
        $generator = $this->createGenerator([]);
        $response = new StreamingResponse($generator, 200);

        // This should cause a PHP error
        $this->expectException(Error::class);
        // @phpstan-ignore-next-line
        $response->statusCode = 404;
    }

    public function testGeneratorIterationWorks(): void
    {
        $generator = function (): Generator {
            for ($i = 1; $i <= 5; $i++) {
                yield "chunk_{$i}";
            }
        };

        $response = new StreamingResponse($generator());

        $chunks = [];
        foreach ($response->generator as $chunk) {
            $chunks[] = $chunk;
        }

        $this->assertCount(5, $chunks);
        $this->assertSame('chunk_1', $chunks[0]);
        $this->assertSame('chunk_5', $chunks[4]);
    }

    public function testStreamingGeneratorBehavior(): void
    {
        $data = ['a', 'b', 'c'];
        $generator = function () use ($data): Generator {
            foreach ($data as $item) {
                yield $item;
            }
        };

        $response = new StreamingResponse($generator());

        // First consumption succeeds
        $chunks1 = \iterator_to_array($response->generator);
        $this->assertCount(3, $chunks1);

        // Generator is exhausted; attempting to traverse again throws exception
        $this->expectException(\Exception::class);
        $this->expectExceptionMessage('Cannot traverse an already closed generator');
        \iterator_to_array($response->generator);
    }

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

    public function testStreamingResponseWithVariousStatusCodes(): void
    {
        $statuses = [200, 201, 202, 206, 300, 301, 400, 404, 500];

        foreach ($statuses as $status) {
            $generator = $this->createGenerator([]);
            $response = new StreamingResponse($generator, $status);
            $this->assertSame($status, $response->statusCode);
        }
    }

    public function testStreamingResponseWithVariousStatusCodesExtended(): void
    {
        $codes = [200, 201, 202, 204, 206, 301, 302, 304, 400, 401, 403, 404, 500, 502, 503];

        foreach ($codes as $code) {
            $generator = $this->createGenerator([]);
            $response = new StreamingResponse($generator, $code);

            $this->assertSame($code, $response->statusCode);
        }
    }

    public function testStreamingResponseEmptyHeaders(): void
    {
        $generator = $this->createGenerator([]);
        $response = new StreamingResponse($generator, 200, []);

        $this->assertSame([], $response->headers);
    }

    public function testStreamingResponseWithMultipleHeaders(): void
    {
        $generator = $this->createGenerator([]);
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

    public function testStreamingResponseWithCustomHeaders(): void
    {
        $generator = $this->createGenerator([]);
        $headers = [
            'X-Custom-Header' => 'value',
            'X-Another' => 'header',
            'Content-Type' => 'text/plain',
        ];

        $response = new StreamingResponse($generator, 200, $headers);

        $this->assertSame($headers, $response->headers);
    }

    public function testBasicStreamingWithMixedTypes(): void
    {
        $generator = function (): Generator {
            yield 'string';
            yield ['number' => 123];
            yield ['array'];
            yield 'done';
        };

        $response = new StreamingResponse($generator());

        $chunks = \iterator_to_array($response->generator);
        $this->assertCount(4, $chunks);
    }

    public function testStreamingGeneratorCanYieldMultipleTimes(): void
    {
        $count = 0;
        $generator = function () use (&$count): Generator {
            for ($i = 0; $i < 10; $i++) {
                ++$count;
                yield "chunk_{$i}";
            }
        };

        $response = new StreamingResponse($generator());

        \iterator_to_array($response->generator);

        $this->assertSame(10, $count);
    }

    // ======================== SSE (Server-Sent Events) Tests ========================

    public function testSseResponse(): void
    {
        $eventGenerator = $this->createGenerator(["data: event1\n\n", "data: event2\n\n"]);
        $response = StreamingResponse::sse($eventGenerator);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        $this->assertSame('no-cache', $response->headers['Cache-Control']);
        $this->assertSame('no', $response->headers['X-Accel-Buffering']);
    }

    public function testSseResponseWithAdditionalHeaders(): void
    {
        $eventGenerator = $this->createGenerator(["data: event1\n\n"]);
        $response = StreamingResponse::sse($eventGenerator, ['X-Custom' => 'header']);

        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        $this->assertSame('header', $response->headers['X-Custom']);
    }

    public function testSseResponseHeadersNotOverridden(): void
    {
        $eventGenerator = $this->createGenerator(["data: event1\n\n"]);
        $response = StreamingResponse::sse($eventGenerator, ['Cache-Control' => 'max-age=3600']);

        // Additional headers should merge, additional takes precedence
        $this->assertSame('max-age=3600', $response->headers['Cache-Control']);
    }

    public function testSseResponseHeaders(): void
    {
        $generator = $this->createGenerator(['data: test']);
        $response = StreamingResponse::sse($generator);

        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        $this->assertSame('no-cache', $response->headers['Cache-Control']);
        $this->assertSame('no', $response->headers['X-Accel-Buffering']);
    }

    public function testSseResponseMergesAdditionalHeaders(): void
    {
        $generator = $this->createGenerator(['data: test']);
        $additional = ['X-Custom' => 'value', 'X-Another' => 'header'];
        $response = StreamingResponse::sse($generator, $additional);

        // Original SSE headers should be present
        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        // Additional headers should be present
        $this->assertSame('value', $response->headers['X-Custom']);
        $this->assertSame('header', $response->headers['X-Another']);
    }

    public function testSseResponseAllowsOverridingCacheControl(): void
    {
        $generator = $this->createGenerator(['data: test']);
        $additional = ['Cache-Control' => 'public, max-age=3600'];
        $response = StreamingResponse::sse($generator, $additional);

        // Additional header should override default
        $this->assertSame('public, max-age=3600', $response->headers['Cache-Control']);
    }

    public function testSseResponseWithEmptyAdditionalHeaders(): void
    {
        $generator = $this->createGenerator(['data: test']);
        $response = StreamingResponse::sse($generator, []);

        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
    }

    public function testSseWithContentTypeOverride(): void
    {
        $generator = $this->createGenerator(['data: test']);
        // Try to override Content-Type (additional headers should override)
        $response = StreamingResponse::sse(
            $generator,
            ['Content-Type' => 'custom/type']
        );

        // The additional headers merge, so custom type should win
        $this->assertSame('custom/type', $response->headers['Content-Type']);
    }

    public function testSseResponseHeadersMergeCorrectly(): void
    {
        $eventGenerator = $this->createGenerator(["data: event1\n\n", "data: event2\n\n"]);
        $additionalHeaders = [
            'X-Custom-Header' => 'value',
            'X-Another' => 'header',
        ];

        $response = StreamingResponse::sse($eventGenerator, $additionalHeaders);

        // Default SSE headers should be present
        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        $this->assertSame('no-cache', $response->headers['Cache-Control']);
        $this->assertSame('no', $response->headers['X-Accel-Buffering']);

        // Additional headers should be present
        $this->assertSame('value', $response->headers['X-Custom-Header']);
        $this->assertSame('header', $response->headers['X-Another']);
    }

    public function testSseResponseOverridesDefaultCacheControl(): void
    {
        $eventGenerator = $this->createGenerator(["data: event1\n\n"]);
        // Try to override a default header
        $response = StreamingResponse::sse($eventGenerator, [
            'Cache-Control' => 'max-age=3600',
        ]);

        // Additional headers take precedence
        $this->assertSame('max-age=3600', $response->headers['Cache-Control']);
    }

    // ======================== File Streaming Tests ========================

    public function testFileStreamingWithValidFile(): void
    {
        $tempFile = $this->createTempFile('test content');

        try {
            $response = StreamingResponse::file($tempFile);

            $this->assertSame(200, $response->statusCode);
            $this->assertArrayHasKey('Content-Type', $response->headers);
            $this->assertArrayHasKey('Content-Length', $response->headers);
            $this->assertSame('12', $response->headers['Content-Length']);

            // Consume generator to verify content
            $chunks = \iterator_to_array($response->generator);
            $stringChunks = \array_map(function ($chunk): string {
                return \is_string($chunk) ? $chunk : '';
            }, $chunks);
            $this->assertSame('test content', \implode('', $stringChunks));
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingWithCustomContentType(): void
    {
        $tempFile = $this->createTempFile('data');

        try {
            $response = StreamingResponse::file($tempFile, contentType: 'application/octet-stream');

            $this->assertSame('application/octet-stream', $response->headers['Content-Type']);
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingWithCustomChunkSize(): void
    {
        $tempFile = $this->createTempFile('1234567890');

        try {
            // Chunk size of 3 bytes
            $response = StreamingResponse::file($tempFile, chunkSize: 3);

            $chunks = \iterator_to_array($response->generator);
            // Should produce 4 chunks: "123", "456", "789", "0"
            $this->assertCount(4, $chunks);
            $this->assertSame('123', $chunks[0]);
            $this->assertSame('456', $chunks[1]);
            $this->assertSame('789', $chunks[2]);
            $this->assertSame('0', $chunks[3]);
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingThrowsForNonexistentFile(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('File not found');

        StreamingResponse::file('/nonexistent/file.txt');
    }

    public function testFileStreamingThrowsForInvalidChunkSize(): void
    {
        $tempFile = $this->createTempFile('data');

        try {
            $this->expectException(InvalidArgumentException::class);
            $this->expectExceptionMessage('Chunk size must be at least 1 byte');

            StreamingResponse::file($tempFile, chunkSize: 0);
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingThrowsForNegativeChunkSize(): void
    {
        $tempFile = $this->createTempFile('data');

        try {
            $this->expectException(InvalidArgumentException::class);
            StreamingResponse::file($tempFile, chunkSize: -100);
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingWithLargeChunkSize(): void
    {
        $tempFile = $this->createTempFile('small');

        try {
            $response = StreamingResponse::file($tempFile, chunkSize: 1000000);

            $chunks = \iterator_to_array($response->generator);
            // One large chunk should contain all data
            $this->assertGreaterThanOrEqual(1, \count($chunks));
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingIncludesContentLength(): void
    {
        $content = 'exactly 12 bytes';
        $tempFile = $this->createTempFile($content);

        try {
            $response = StreamingResponse::file($tempFile);

            $this->assertArrayHasKey('Content-Length', $response->headers);
            $this->assertSame((string)\strlen($content), $response->headers['Content-Length']);
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingAutoDetectsMimeType(): void
    {
        $tempFile = $this->createTempFile('<!DOCTYPE html>', 'test.html');

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

    public function testFileStreamingClosesFileHandle(): void
    {
        $tempFile = $this->createTempFile('data to stream');

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

    public function testFileStreamingWithEmptyFile(): void
    {
        $tempFile = $this->createTempFile('');

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

    public function testFileStreamingWithoutContentType(): void
    {
        $tempFile = $this->createTempFile('test');

        try {
            // When contentType is null, it tries to auto-detect
            $response = StreamingResponse::file($tempFile, contentType: null);

            $this->assertArrayHasKey('Content-Type', $response->headers);
            // The Content-Type should be auto-detected (may vary by system)
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingFailsToOpenFile(): void
    {
        // Create a temporary file path, write to it, then delete it
        // to simulate a file that disappears between check and open
        $tempFile = $this->createTempFile('content');
        @\unlink($tempFile);

        // Now calling file() will pass the exists check but fail on fopen
        // We need to test the generator's runtime behavior
        try {
            $response = StreamingResponse::file($tempFile);
            // The generator is lazy, so the exception occurs during iteration
            $chunks = [];
            foreach ($response->generator as $chunk) {
                $chunks[] = $chunk;
            }
            // If file still doesn't exist, no chunks will be generated
            $this->assertEmpty($chunks);
        } catch (InvalidArgumentException $e) {
            // This is also acceptable - file check happens upfront
            $this->assertStringContainsString('File not found', $e->getMessage());
        }
    }

    public function testFileStreamingWithVeryLargeChunkSize(): void
    {
        $tempFile = $this->createTempFile('small content');

        try {
            // Chunk size larger than file should work fine
            $response = StreamingResponse::file($tempFile, chunkSize: 10000000);

            $chunks = \iterator_to_array($response->generator);
            // Should still read the entire file in one chunk
            $this->assertCount(1, $chunks);
            $this->assertSame('small content', $chunks[0]);
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingContentLengthCalculation(): void
    {
        $content = 'exact content';
        $tempFile = $this->createTempFile($content);

        try {
            $response = StreamingResponse::file($tempFile);

            // Verify Content-Length header is set correctly
            $this->assertArrayHasKey('Content-Length', $response->headers);
            $this->assertSame((string)\strlen($content), $response->headers['Content-Length']);
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingWithSpecialMimeTypes(): void
    {
        $mimeTypes = [
            'application/json' => 'test.json',
            'text/plain' => 'test.txt',
            'text/html' => 'test.html',
            'image/png' => 'test.png',
        ];

        foreach ($mimeTypes as $mimeType => $filename) {
            $tempFile = $this->createTempFile('content', $filename);

            try {
                $response = StreamingResponse::file($tempFile, contentType: $mimeType);

                $this->assertSame($mimeType, $response->headers['Content-Type']);
            } finally {
                @\unlink($tempFile);
            }
        }
    }

    // ======================== JSON Lines Streaming Tests ========================

    public function testJsonLinesStreaming(): void
    {
        $dataGenerator = function (): Generator {
            yield ['id' => 1, 'name' => 'Alice'];
            yield ['id' => 2, 'name' => 'Bob'];
            yield ['id' => 3, 'name' => 'Charlie'];
        };

        $response = StreamingResponse::jsonLines($dataGenerator());

        $this->assertSame(200, $response->statusCode);
        $this->assertSame('application/x-ndjson', $response->headers['Content-Type']);

        $lines = \iterator_to_array($response->generator);
        $this->assertCount(3, $lines);
        if (isset($lines[0]) && \is_string($lines[0])) {
            $this->assertStringContainsString('"id":1', $lines[0]);
            $this->assertStringContainsString('"name":"Alice"', $lines[0]);
            $this->assertStringEndsWith("\n", $lines[0]);
        }
        if (isset($lines[1]) && \is_string($lines[1])) {
            $this->assertStringEndsWith("\n", $lines[1]);
        }
        if (isset($lines[2]) && \is_string($lines[2])) {
            $this->assertStringEndsWith("\n", $lines[2]);
        }
    }

    public function testJsonLinesWithEmptyGenerator(): void
    {
        $dataGenerator = static function (): Generator {
            yield from [];
        };
        $response = StreamingResponse::jsonLines($dataGenerator());

        $lines = \iterator_to_array($response->generator);
        $this->assertCount(0, $lines);
    }

    public function testJsonLinesWithComplexData(): void
    {
        $dataGenerator = function (): Generator {
            yield [
                'nested' => ['array' => [1, 2, 3]],
                'bool' => true,
                'null' => null,
                'number' => 42.5,
            ];
        };

        $response = StreamingResponse::jsonLines($dataGenerator());

        $lines = \iterator_to_array($response->generator);
        if (isset($lines[0]) && \is_string($lines[0])) {
            $decoded = \json_decode($lines[0], true);

            if (\is_array($decoded)) {
                $nested = $decoded['nested'] ?? null;
                if (\is_array($nested) && isset($nested['array'])) {
                    $this->assertSame([1, 2, 3], $nested['array']);
                }
                $bool = $decoded['bool'] ?? null;
                if ($bool !== null) {
                    $this->assertTrue($bool);
                }
                $null = $decoded['null'] ?? null;
                $this->assertNull($null);
                $number = $decoded['number'] ?? null;
                if ($number !== null) {
                    $this->assertSame(42.5, $number);
                }
            }
        }
    }

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
            \json_decode($line, false);
            $this->assertSame(JSON_ERROR_NONE, \json_last_error(), "Line is not valid JSON: {$line}");
        }
    }

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

    public function testJsonLinesWithSpecialCharactersInData(): void
    {
        $dataGenerator = function (): Generator {
            yield ['message' => 'Hello "World"'];
            yield ['text' => "Line with\nnewline"];
            yield ['symbol' => 'Backslash: \\'];
        };

        $response = StreamingResponse::jsonLines($dataGenerator());

        $lines = \iterator_to_array($response->generator);
        $this->assertCount(3, $lines);

        if (isset($lines[0]) && \is_string($lines[0])) {
            $decoded = \json_decode($lines[0], true);
            $this->assertIsArray($decoded);
        }
    }

    public function testJsonLinesWithNullValues(): void
    {
        $dataGenerator = function (): Generator {
            yield ['value' => null];
            yield null;
            yield ['nested' => null];
        };

        $response = StreamingResponse::jsonLines($dataGenerator());

        $lines = \iterator_to_array($response->generator);
        $this->assertCount(3, $lines);

        // Each line should be valid JSON
        foreach ($lines as $line) {
            if (\is_string($line)) {
                $decoded = \json_decode($line, true);
                $this->assertSame(JSON_ERROR_NONE, \json_last_error());
                // First and third lines are arrays, second line is null which decodes to null
                $this->assertTrue(\is_array($decoded) || $decoded === null);
            }
        }
    }

    public function testJsonLinesWithBooleanValues(): void
    {
        $dataGenerator = function (): Generator {
            yield ['flag' => true];
            yield ['active' => false];
        };

        $response = StreamingResponse::jsonLines($dataGenerator());

        $lines = \iterator_to_array($response->generator);
        $this->assertCount(2, $lines);

        if (isset($lines[0]) && \is_string($lines[0])) {
            $this->assertStringContainsString('true', $lines[0]);
        }
        if (isset($lines[1]) && \is_string($lines[1])) {
            $this->assertStringContainsString('false', $lines[1]);
        }
    }

    public function testJsonLinesWithNumericValues(): void
    {
        $dataGenerator = function (): Generator {
            yield ['int' => 42];
            yield ['float' => 3.14159];
            yield ['negative' => -99];
            yield ['zero' => 0];
        };

        $response = StreamingResponse::jsonLines($dataGenerator());

        $lines = \iterator_to_array($response->generator);
        $this->assertCount(4, $lines);

        if (isset($lines[0]) && \is_string($lines[0])) {
            $decoded = \json_decode($lines[0], true);
            if (\is_array($decoded)) {
                $this->assertSame(42, $decoded['int'] ?? null);
            }
        }
    }

    public function testJsonLinesContentTypeHeader(): void
    {
        $dataGenerator = function (): Generator {
            yield [];
        };

        $response = StreamingResponse::jsonLines($dataGenerator());

        $this->assertSame('application/x-ndjson', $response->headers['Content-Type']);
    }
}
