<?php

declare(strict_types=1);

namespace Spikard\Config;

final class CompressionConfig
{
    public function __construct(
        public readonly ?bool $gzip = null,
        public readonly ?bool $brotli = null,
        public readonly ?int $minSize = null,
        public readonly ?int $quality = null,
    ) {
    }
}
