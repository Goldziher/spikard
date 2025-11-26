<?php

declare(strict_types=1);

namespace Spikard\Handlers;

use Generator;

interface SseEventProducerInterface
{
    /** @return Generator<int, string, mixed, void> */
    public function __invoke(): Generator;
}
