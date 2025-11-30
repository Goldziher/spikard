<?php

declare(strict_types=1);

namespace Spikard\Http;

use Generator;
use InvalidArgumentException;
use RuntimeException;

/**
 * Streaming HTTP response backed by a PHP Generator.
 *
 * Use this for large files, real-time data feeds, or any response where
 * data is generated incrementally. Automatically sets Transfer-Encoding: chunked.
 *
 * @example Basic streaming
 * ```php
 * function streamNumbers(): Generator {
 *     for ($i = 1; $i <= 5; $i++) {
 *         yield "Chunk {$i}\n";
 *         usleep(100000); // 100ms delay between chunks
 *     }
 * }
 *
 * return new StreamingResponse(streamNumbers());
 * ```
 *
 * @example Streaming with custom status and headers
 * ```php
 * $generator = function(): Generator {
 *     yield "data: Hello\n\n";
 *     yield "data: World\n\n";
 * };
 *
 * return new StreamingResponse(
 *     $generator(),
 *     statusCode: 200,
 *     headers: ['Content-Type' => 'text/event-stream']
 * );
 * ```
 */
final class StreamingResponse
{
    /**
     * @param Generator<int, string|array<mixed>, mixed, void> $generator Generator that yields string chunks
     * @param int $statusCode HTTP status code (default: 200)
     * @param array<string, string> $headers Response headers
     */
    public function __construct(
        public readonly Generator $generator,
        public readonly int $statusCode = 200,
        public readonly array $headers = [],
    ) {
    }

    /**
     * Create a Server-Sent Events (SSE) streaming response.
     *
     * Automatically sets Content-Type: text/event-stream and disables buffering.
     *
     * @param Generator<int, string, mixed, void> $eventGenerator Generator yielding SSE-formatted strings
     * @param array<string, string> $additionalHeaders Additional headers
     *
     * @example
     * ```php
     * $events = function(): Generator {
     *     for ($i = 0; $i < 5; $i++) {
     *         yield "data: " . json_encode(['time' => time(), 'count' => $i]) . "\n\n";
     *         sleep(1);
     *     }
     * };
     *
     * return StreamingResponse::sse($events());
     * ```
     */
    public static function sse(
        Generator $eventGenerator,
        array $additionalHeaders = []
    ): self {
        $headers = \array_merge(
            [
                'Content-Type' => 'text/event-stream',
                'Cache-Control' => 'no-cache',
                'X-Accel-Buffering' => 'no', // Disable nginx buffering
            ],
            $additionalHeaders
        );

        return new self($eventGenerator, 200, $headers);
    }

    /**
     * Stream a file in chunks without loading it entirely into memory.
     *
     * @param string $filePath Path to file to stream
     * @param int $chunkSize Size of each chunk in bytes (default: 8KB)
     * @param string|null $contentType Content-Type header (auto-detected if null)
     *
     * @example
     * ```php
     * // Stream a large video file
     * return StreamingResponse::file('/path/to/video.mp4', chunkSize: 65536);
     * ```
     */
    public static function file(
        string $filePath,
        int $chunkSize = 8192,
        ?string $contentType = null
    ): self {
        if (!\file_exists($filePath)) {
            throw new InvalidArgumentException("File not found: {$filePath}");
        }

        if ($chunkSize < 1) {
            throw new InvalidArgumentException('Chunk size must be at least 1 byte');
        }

        $generator = function () use ($filePath, $chunkSize): Generator {
            $handle = \fopen($filePath, 'rb');
            if ($handle === false) {
                throw new RuntimeException("Failed to open file: {$filePath}");
            }

            try {
                while (!\feof($handle)) {
                    $chunk = \fread($handle, $chunkSize);
                    if ($chunk === false) {
                        break;
                    }
                    yield $chunk;
                }
            } finally {
                \fclose($handle);
            }
        };

        $headers = [];
        if ($contentType !== null) {
            $headers['Content-Type'] = $contentType;
        } else {
            // Auto-detect MIME type
            $mimeType = \mime_content_type($filePath);
            if ($mimeType !== false) {
                $headers['Content-Type'] = $mimeType;
            }
        }

        // Add Content-Length if possible
        $fileSize = \filesize($filePath);
        if ($fileSize !== false) {
            $headers['Content-Length'] = (string) $fileSize;
        }

        return new self($generator(), headers: $headers);
    }

    /**
     * Stream JSON lines (JSONL/ndjson format).
     *
     * Each yielded value is JSON-encoded and appended with a newline.
     *
     * @param Generator<int, mixed, mixed, void> $dataGenerator Generator yielding data to JSON-encode
     *
     * @example
     * ```php
     * $records = function(): Generator {
     *     foreach ($users as $user) {
     *         yield ['id' => $user->id, 'name' => $user->name];
     *     }
     * };
     *
     * return StreamingResponse::jsonLines($records());
     * ```
     */
    public static function jsonLines(Generator $dataGenerator): self
    {
        $generator = function () use ($dataGenerator): Generator {
            foreach ($dataGenerator as $data) {
                yield \json_encode($data, JSON_THROW_ON_ERROR) . "\n";
            }
        };

        return new self(
            $generator(),
            headers: ['Content-Type' => 'application/x-ndjson']
        );
    }
}
