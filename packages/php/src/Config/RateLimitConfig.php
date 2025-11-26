<?php

declare(strict_types=1);

namespace Spikard\Config;

final class RateLimitConfig
{
    public function __construct(
        public readonly int $burst = 20,
        public readonly int $refill = 5,
    ) {
    }
}
