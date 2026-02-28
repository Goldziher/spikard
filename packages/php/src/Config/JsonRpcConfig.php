<?php

declare(strict_types=1);

namespace Spikard\Config;

/**
 * JSON-RPC endpoint configuration.
 */
final class JsonRpcConfig
{
    /**
     * @param bool $enabled Enable JSON-RPC endpoint registration
     * @param string $endpointPath HTTP path for JSON-RPC requests
     * @param bool $enableBatch Enable JSON-RPC batch requests
     * @param int $maxBatchSize Maximum number of requests in a single batch
     */
    public function __construct(
        public readonly bool $enabled = true,
        public readonly string $endpointPath = '/rpc',
        public readonly bool $enableBatch = true,
        public readonly int $maxBatchSize = 100,
    ) {
    }

    public static function builder(): JsonRpcConfigBuilder
    {
        return new JsonRpcConfigBuilder();
    }
}

/**
 * Builder for JsonRpcConfig.
 */
final class JsonRpcConfigBuilder
{
    private bool $enabled = true;
    private string $endpointPath = '/rpc';
    private bool $enableBatch = true;
    private int $maxBatchSize = 100;

    public function withEnabled(bool $enabled): self
    {
        $this->enabled = $enabled;
        return $this;
    }

    public function withEndpointPath(string $endpointPath): self
    {
        $this->endpointPath = $endpointPath;
        return $this;
    }

    public function withEnableBatch(bool $enableBatch): self
    {
        $this->enableBatch = $enableBatch;
        return $this;
    }

    public function withMaxBatchSize(int $maxBatchSize): self
    {
        $this->maxBatchSize = $maxBatchSize;
        return $this;
    }

    public function build(): JsonRpcConfig
    {
        return new JsonRpcConfig(
            enabled: $this->enabled,
            endpointPath: $this->endpointPath,
            enableBatch: $this->enableBatch,
            maxBatchSize: $this->maxBatchSize,
        );
    }
}
