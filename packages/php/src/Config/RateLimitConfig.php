<?php

declare(strict_types=1);

namespace Spikard\Config;

/**
 * Rate limiting middleware configuration.
 *
 * Configures rate limiting for the HTTP server.
 */
final class RateLimitConfig
{
    /**
     * @param int $perSecond Requests allowed per second
     * @param int $burst Burst capacity
     * @param bool|null $ipBased Enable per-IP rate limiting
     */
    public function __construct(
        public readonly int $perSecond,
        public readonly int $burst,
        public readonly ?bool $ipBased = null,
    ) {
    }

    public static function builder(): RateLimitConfigBuilder
    {
        return new RateLimitConfigBuilder();
    }
}

/**
 * Builder for RateLimitConfig.
 *
 * Provides a fluent interface for constructing RateLimitConfig instances.
 */
final class RateLimitConfigBuilder
{
    private int $perSecond = 100;
    private int $burst = 200;
    private ?bool $ipBased = null;

    public function withPerSecond(int $perSecond): self
    {
        $this->perSecond = $perSecond;
        return $this;
    }

    public function withBurst(int $burst): self
    {
        $this->burst = $burst;
        return $this;
    }

    public function withIpBased(bool $enabled): self
    {
        $this->ipBased = $enabled;
        return $this;
    }

    public function build(): RateLimitConfig
    {
        return new RateLimitConfig(
            perSecond: $this->perSecond,
            burst: $this->burst,
            ipBased: $this->ipBased,
        );
    }
}
