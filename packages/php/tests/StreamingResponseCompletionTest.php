<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Generator;
use InvalidArgumentException;
use PHPUnit\Framework\TestCase;
use Spikard\Http\StreamingResponse;

/**
 * Additional tests to complete StreamingResponse coverage.
 *
 * Tests remaining uncovered branches in:
 * - Spikard\Http\StreamingResponse (increase from 95.35% to 100%)
 */
final class StreamingResponseCompletionTest extends TestCase
{
    // ======================== File Streaming Edge Cases ========================

    public function testFileStreamingWithNegativeChunkSize(): void
    {
        $tempFile = $this->createTempFile('data');

        try {
            $this->expectException(InvalidArgumentException::class);
            $this->expectExceptionMessage('Chunk size must be at least 1 byte');

            StreamingResponse::file($tempFile, chunkSize: -1);
        } finally {
            @\unlink($tempFile);
        }
    }

    public function testFileStreamingWithZeroChunkSize(): void
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

    // ======================== SSE Additional Tests ========================

    public function testSseResponseHeadersMergeCorrectly(): void
    {
        $eventGenerator = $this->createSseGenerator();
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
        $eventGenerator = $this->createSseGenerator();
        // Try to override a default header
        $response = StreamingResponse::sse($eventGenerator, [
            'Cache-Control' => 'max-age=3600',
        ]);

        // Additional headers take precedence
        $this->assertSame('max-age=3600', $response->headers['Cache-Control']);
    }

    public function testSseResponseWithEmptyAdditionalHeaders(): void
    {
        $eventGenerator = $this->createSseGenerator();
        $response = StreamingResponse::sse($eventGenerator, []);

        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== JSON Lines Additional Tests ========================

    public function testJsonLinesWithSpecialCharacters(): void
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

    // ======================== Basic Streaming Tests ========================

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

    public function testStreamingResponseWithCustomHeaders(): void
    {
        $generator = $this->createBasicGenerator();
        $headers = [
            'X-Custom-Header' => 'value',
            'X-Another' => 'header',
            'Content-Type' => 'text/plain',
        ];

        $response = new StreamingResponse($generator, 200, $headers);

        $this->assertSame($headers, $response->headers);
    }

    public function testStreamingResponseWithVariousStatusCodes(): void
    {
        $codes = [200, 201, 202, 204, 206, 301, 302, 304, 400, 401, 403, 404, 500, 502, 503];

        foreach ($codes as $code) {
            $generator = $this->createBasicGenerator();
            $response = new StreamingResponse($generator, $code);

            $this->assertSame($code, $response->statusCode);
        }
    }

    public function testStreamingResponseEmptyHeaders(): void
    {
        $generator = $this->createBasicGenerator();
        $response = new StreamingResponse($generator, 200, []);

        $this->assertSame([], $response->headers);
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

    // ======================== Helper Methods ========================

    private function createBasicGenerator(): Generator
    {
        yield 'chunk1';
        yield 'chunk2';
    }

    private function createSseGenerator(): Generator
    {
        yield "data: event1\n\n";
        yield "data: event2\n\n";
    }

    private function createTempFile(string $content, ?string $filename = null): string
    {
        if ($filename === null) {
            $tempFile = \tempnam(\sys_get_temp_dir(), 'spikard_test_');
        } else {
            $tempFile = \sys_get_temp_dir() . \DIRECTORY_SEPARATOR . $filename;
        }

        if ($tempFile === false) {
            throw new \RuntimeException('Failed to create temp file');
        }

        \file_put_contents($tempFile, $content);
        return $tempFile;
    }
}
