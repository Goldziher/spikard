<?php

declare(strict_types=1);

namespace Spikard\Testing;

/**
 * SSE event for PHP testing.
 *
 * Represents a single Server-Sent Event with data and metadata.
 */
class SseEvent
{
    /**
     * Get the data field of the event.
     */
    public function getData(): string
    {
    }

    /**
     * Parse the event data as JSON.
     *
     * @return array<string, mixed>
     */
    public function asJson(): array
    {
    }

    /**
     * Get the event type if specified.
     */
    public function getEventType(): ?string
    {
    }

    /**
     * Get the event ID if specified.
     */
    public function getId(): ?string
    {
    }
}
