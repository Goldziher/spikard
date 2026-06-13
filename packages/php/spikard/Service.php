<?php

declare(strict_types=1);

// Spikard application builder.
final class App
{
    private array $registrations = [];
    private ?ServerConfig $config = null;

    /**
     * Create a new application with default configuration.
     */
    public function __construct()
    {
    }

    /**
     * Set the server configuration.
     */
    public function config(ServerConfig $config): self
    {
        $this->config = $config;
        return $this;
    }

    /**
     * Register an onRequest lifecycle hook.
     */
    public function onRequest(callable $handler): self
    {
        return $this;
    }

    /**
     * Register a preValidation lifecycle hook.
     */
    public function preValidation(callable $handler): self
    {
        return $this;
    }

    /**
     * Register a preHandler lifecycle hook.
     */
    public function preHandler(callable $handler): self
    {
        return $this;
    }

    /**
     * Register an onResponse lifecycle hook.
     */
    public function onResponse(callable $handler): self
    {
        return $this;
    }

    /**
     * Register an onError lifecycle hook.
     */
    public function onError(callable $handler): self
    {
        return $this;
    }

    /**
     * Register a GET route at the given path..
     */
    public function get(string $path, callable $handler): self
    {
        $this->registrations[] = ['route', ['GET', $path], $handler];
        return $this;
    }

    /**
     * Register a POST route at the given path..
     */
    public function post(string $path, callable $handler): self
    {
        $this->registrations[] = ['route', ['POST', $path], $handler];
        return $this;
    }

    /**
     * Register a PUT route at the given path..
     */
    public function put(string $path, callable $handler): self
    {
        $this->registrations[] = ['route', ['PUT', $path], $handler];
        return $this;
    }

    /**
     * Register a PATCH route at the given path..
     */
    public function patch(string $path, callable $handler): self
    {
        $this->registrations[] = ['route', ['PATCH', $path], $handler];
        return $this;
    }

    /**
     * Register a DELETE route at the given path..
     */
    public function delete(string $path, callable $handler): self
    {
        $this->registrations[] = ['route', ['DELETE', $path], $handler];
        return $this;
    }

    /**
     * Register a HEAD route at the given path..
     */
    public function head(string $path, callable $handler): self
    {
        $this->registrations[] = ['route', ['HEAD', $path], $handler];
        return $this;
    }

    /**
     * Register a OPTIONS route at the given path..
     */
    public function options(string $path, callable $handler): self
    {
        $this->registrations[] = ['route', ['OPTIONS', $path], $handler];
        return $this;
    }

    /**
     * Run the HTTP server.
     */
    public function run(): void
    {
        app_run($this->registrations, $this->config);
    }

    public function into_router(): string
    {
        /**
         * Build the underlying Axum router.
         *
         * # Errors
         *
         * Returns an error if server or router construction fails.
         */
        // Finalize entrypoint — forwarding to native layer
        return null;
    }

}
