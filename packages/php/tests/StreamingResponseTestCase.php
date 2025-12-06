<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Generator;
use PHPUnit\Framework\TestCase;
use RuntimeException;

/**
 * Abstract base class for StreamingResponse tests.
 *
 * Provides common helper methods and fixtures for all StreamingResponse test cases.
 */
abstract class StreamingResponseTestCase extends TestCase
{
    /**
     * Create a temporary file with given content.
     *
     * @throws RuntimeException If temp file creation fails
     */
    protected function createTempFile(string $content, ?string $filename = null): string
    {
        if ($filename === null) {
            $tempFile = \tempnam(\sys_get_temp_dir(), 'spikard_test_');
            if ($tempFile === false) {
                throw new RuntimeException('Failed to create temp file');
            }
        } else {
            $tempFile = \sys_get_temp_dir() . \DIRECTORY_SEPARATOR . $filename;
        }

        if (\file_put_contents($tempFile, $content) === false) {
            throw new RuntimeException("Failed to write to temp file: {$tempFile}");
        }

        return $tempFile;
    }

    /**
     * Create a generator from an array of items.
     *
     * @param list<mixed> $items
     */
    protected function createGenerator(array $items): Generator
    {
        foreach ($items as $item) {
            yield $item;
        }
    }
}
