<?php

declare(strict_types=1);

namespace Spikard\Config;

/**
 * Static file serving configuration.
 *
 * Configures serving static files from the filesystem.
 */
final class StaticFilesConfig
{
    /**
     * @param bool $enabled Enable static file serving
     * @param string|null $root Root directory for static files
     * @param string|null $indexFile Index file name (default: index.html)
     * @param bool $cache Enable caching headers
     */
    public function __construct(
        public readonly bool $enabled = false,
        public readonly ?string $root = null,
        public readonly ?string $indexFile = null,
        public readonly bool $cache = true,
    ) {
    }

    public static function builder(): StaticFilesConfigBuilder
    {
        return new StaticFilesConfigBuilder();
    }
}

/**
 * Builder for StaticFilesConfig.
 *
 * Provides a fluent interface for constructing StaticFilesConfig instances.
 */
final class StaticFilesConfigBuilder
{
    private bool $enabled = false;
    private ?string $root = null;
    private ?string $indexFile = null;
    private bool $cache = true;

    public function withEnabled(bool $enabled): self
    {
        $this->enabled = $enabled;
        return $this;
    }

    public function withRoot(string $root): self
    {
        $this->root = $root;
        return $this;
    }

    public function withIndexFile(string $indexFile): self
    {
        $this->indexFile = $indexFile;
        return $this;
    }

    public function withCache(bool $cache): self
    {
        $this->cache = $cache;
        return $this;
    }

    public function build(): StaticFilesConfig
    {
        return new StaticFilesConfig(
            enabled: $this->enabled,
            root: $this->root,
            indexFile: $this->indexFile,
            cache: $this->cache,
        );
    }
}
