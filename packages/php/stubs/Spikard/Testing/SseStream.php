<?php

declare(strict_types=1);

namespace Spikard\Testing;

/**
 * SSE stream for PHP testing.
 *
 * Provides methods to read Server-Sent Events in tests.
 */
class SseStream
{
    /**
     * Get all events from the stream as an array.
     *
     * @return array<int, SseEvent>
     */
    public function events(): array
    {
    }

    /**
     * Get all events as JSON values.
     *
     * @return array<int, array<string, mixed>>
     */
    public function eventsAsJson(): array
    {
    }

    /**
     * Get the raw body of the SSE response.
     */
    public function body(): string
    {
    }

    /**
     * Get the number of events in the stream.
     */
    public function count(): int
    {
    }
}
