<?php

declare(strict_types=1);

namespace Spikard\Config;

use Spikard\Config\LifecycleHooks;

final class ServerConfig
{
    public function __construct(
        public readonly ?CompressionConfig $compression = null,
        public readonly ?RateLimitConfig $rateLimit = null,
        public readonly ?CorsConfig $cors = null,
        public readonly ?StaticFilesConfig $staticFiles = null,
        public readonly ?LifecycleHooks $hooks = null,
    ) {
    }

    public static function builder(): ServerConfigBuilder
    {
        return new ServerConfigBuilder();
    }
}

final class ServerConfigBuilder
{
    private ?CompressionConfig $compression = null;
    private ?RateLimitConfig $rateLimit = null;
    private ?CorsConfig $cors = null;
    private ?StaticFilesConfig $staticFiles = null;
    private ?LifecycleHooks $hooks = null;

    public function withCompression(CompressionConfig $config): self
    {
        $this->compression = $config;
        return $this;
    }

    public function withRateLimit(RateLimitConfig $config): self
    {
        $this->rateLimit = $config;
        return $this;
    }

    public function withCors(CorsConfig $config): self
    {
        $this->cors = $config;
        return $this;
    }

    public function withStaticFiles(StaticFilesConfig $config): self
    {
        $this->staticFiles = $config;
        return $this;
    }

    public function withLifecycleHooks(LifecycleHooks $hooks): self
    {
        $this->hooks = $hooks;
        return $this;
    }

    public function build(): ServerConfig
    {
        return new ServerConfig(
            compression: $this->compression,
            rateLimit: $this->rateLimit,
            cors: $this->cors,
            staticFiles: $this->staticFiles,
            hooks: $this->hooks,
        );
    }
}
