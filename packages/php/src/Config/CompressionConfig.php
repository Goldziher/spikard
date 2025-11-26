<?php

declare(strict_types=1);

namespace Spikard\Config;

final class CompressionConfig
{
    public function __construct(
        public readonly bool $enabled = true,
        public readonly int $quality = 6,
    ) {
    }
}
