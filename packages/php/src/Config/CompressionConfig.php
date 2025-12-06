<?php

declare(strict_types=1);

namespace Spikard\Config;

/**
 * Compression middleware configuration.
 *
 * Configures gzip and brotli compression for HTTP responses.
 */
final class CompressionConfig
{
    /**
     * @param bool|null $gzip Enable gzip compression
     * @param bool|null $brotli Enable brotli compression
     * @param int|null $minSize Minimum response size to compress (bytes)
     * @param int|null $quality Compression quality level (0-11, higher = slower)
     */
    public function __construct(
        public readonly ?bool $gzip = null,
        public readonly ?bool $brotli = null,
        public readonly ?int $minSize = null,
        public readonly ?int $quality = null,
    ) {
    }

    public static function builder(): CompressionConfigBuilder
    {
        return new CompressionConfigBuilder();
    }
}

/**
 * Builder for CompressionConfig.
 *
 * Provides a fluent interface for constructing CompressionConfig instances.
 */
final class CompressionConfigBuilder
{
    private ?bool $gzip = null;
    private ?bool $brotli = null;
    private ?int $minSize = null;
    private ?int $quality = null;

    public function withGzip(bool $enabled): self
    {
        $this->gzip = $enabled;
        return $this;
    }

    public function withBrotli(bool $enabled): self
    {
        $this->brotli = $enabled;
        return $this;
    }

    public function withMinSize(int $size): self
    {
        $this->minSize = $size;
        return $this;
    }

    public function withQuality(int $quality): self
    {
        $this->quality = $quality;
        return $this;
    }

    public function build(): CompressionConfig
    {
        return new CompressionConfig(
            gzip: $this->gzip,
            brotli: $this->brotli,
            minSize: $this->minSize,
            quality: $this->quality,
        );
    }
}
