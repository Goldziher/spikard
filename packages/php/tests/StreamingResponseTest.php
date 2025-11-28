<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Generator;
use InvalidArgumentException;
use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\Http\StreamingResponse;

final class StreamingResponseTest extends TestCase
{
    public function testBasicStreamingResponse(): void
    {
        $generator = $this->createBasicGenerator();
        $response = new StreamingResponse($generator);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame([], $response->headers);
        $this->assertInstanceOf(Generator::class, $response->generator);
    }

    public function testStreamingResponseWithCustomStatus(): void
    {
        $generator = $this->createBasicGenerator();
        $response = new StreamingResponse($generator, 201);

        $this->assertSame(201, $response->statusCode);
    }

    public function testStreamingResponseWithHeaders(): void
    {
        $generator = $this->createBasicGenerator();
        $headers = ['Content-Type' => 'text/plain', 'X-Custom' => 'value'];
        $response = new StreamingResponse($generator, 200, $headers);

        $this->assertSame($headers, $response->headers);
    }

    public function testSseResponse(): void
    {
        $eventGenerator = $this->createSseGenerator();
        $response = StreamingResponse::sse($eventGenerator);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        $this->assertSame('no-cache', $response->headers['Cache-Control']);
        $this->assertSame('no', $response->headers['X-Accel-Buffering']);
    }

    public function testSseResponseWithAdditionalHeaders(): void
    {
        $eventGenerator = $this->createSseGenerator();
        $response = StreamingResponse::sse($eventGenerator, ['X-Custom' => 'header']);

        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
        $this->assertSame('header', $response->headers['X-Custom']);
    }

    public function testSseResponseHeadersNotOverridden(): void
    {
        $eventGenerator = $this->createSseGenerator();
        $response = StreamingResponse::sse($eventGenerator, ['Cache-Control' => 'max-age=3600']);

        // Additional headers should merge, original takes precedence
        $this->assertSame('max-age=3600', $response->headers['Cache-Control']);
    }

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
            $chunks = iterator_to_array($response->generator);
            $stringChunks = array_map(function ($chunk): string {
                return is_string($chunk) ? $chunk : '';
            }, $chunks);
            $this->assertSame('test content', implode('', $stringChunks));
        } finally {
            @unlink($tempFile);
        }
    }

    public function testFileStreamingWithCustomContentType(): void
    {
        $tempFile = $this->createTempFile('data');

        try {
            $response = StreamingResponse::file($tempFile, contentType: 'application/octet-stream');

            $this->assertSame('application/octet-stream', $response->headers['Content-Type']);
        } finally {
            @unlink($tempFile);
        }
    }

    public function testFileStreamingWithCustomChunkSize(): void
    {
        $tempFile = $this->createTempFile('1234567890');

        try {
            // Chunk size of 3 bytes
            $response = StreamingResponse::file($tempFile, chunkSize: 3);

            $chunks = iterator_to_array($response->generator);
            // Should produce 4 chunks: "123", "456", "789", "0"
            $this->assertCount(4, $chunks);
            $this->assertSame('123', $chunks[0]);
            $this->assertSame('456', $chunks[1]);
            $this->assertSame('789', $chunks[2]);
            $this->assertSame('0', $chunks[3]);
        } finally {
            @unlink($tempFile);
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
            @unlink($tempFile);
        }
    }

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

        $lines = iterator_to_array($response->generator);
        $this->assertCount(3, $lines);
        if (isset($lines[0]) && is_string($lines[0])) {
            $this->assertStringContainsString('"id":1', $lines[0]);
            $this->assertStringContainsString('"name":"Alice"', $lines[0]);
            $this->assertStringEndsWith("\n", $lines[0]);
        }
        if (isset($lines[1]) && is_string($lines[1])) {
            $this->assertStringEndsWith("\n", $lines[1]);
        }
        if (isset($lines[2]) && is_string($lines[2])) {
            $this->assertStringEndsWith("\n", $lines[2]);
        }
    }

    public function testJsonLinesWithEmptyGenerator(): void
    {
        $dataGenerator = $this->createEmptyGenerator();
        $response = StreamingResponse::jsonLines($dataGenerator);

        $lines = iterator_to_array($response->generator);
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

        $lines = iterator_to_array($response->generator);
        if (isset($lines[0]) && is_string($lines[0])) {
            $decoded = json_decode($lines[0], true);

            $this->assertIsArray($decoded);
            if (is_array($decoded) && isset($decoded['nested']['array'])) {
                $this->assertSame([1, 2, 3], $decoded['nested']['array']);
            }
            if (is_array($decoded) && isset($decoded['bool'])) {
                $this->assertTrue($decoded['bool']);
            }
            if (is_array($decoded) && isset($decoded['null'])) {
                $this->assertNull($decoded['null']);
            }
            if (is_array($decoded) && isset($decoded['number'])) {
                $this->assertSame(42.5, $decoded['number']);
            }
        }
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

    private function createEmptyGenerator(): Generator
    {
        return;
        yield; // @phpstan-ignore-line
    }

    private function createTempFile(string $content): string
    {
        $tempFile = tempnam(sys_get_temp_dir(), 'spikard_test_');
        if ($tempFile === false) {
            throw new RuntimeException('Failed to create temp file');
        }

        file_put_contents($tempFile, $content);
        return $tempFile;
    }
}
