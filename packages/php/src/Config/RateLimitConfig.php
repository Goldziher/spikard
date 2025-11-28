<?php

declare(strict_types=1);

namespace Spikard\Config;

final class RateLimitConfig
{
    public function __construct(
        public readonly int $perSecond,
        public readonly int $burst,
        public readonly ?bool $ipBased = null,
    ) {
    }
}
