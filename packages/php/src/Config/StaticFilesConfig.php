<?php

declare(strict_types=1);

namespace Spikard\Config;

final class StaticFilesConfig
{
    public function __construct(
        public readonly bool $enabled = false,
        public readonly ?string $root = null,
        public readonly ?string $indexFile = null,
        public readonly bool $cache = true,
    ) {
    }
}
