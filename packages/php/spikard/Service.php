<?php

declare(strict_types=1);

// Spikard application builder.
final class App
{
    private array $registrations = [];

    // Create a new application with the default server configuration.
    public function __construct()
    {
    }

    // Set the server configuration.
    public function config(ServerConfig $config): self
    {
        $this->_config = $config;
        return $this;
    }

    /**
     * Register a route using the provided builder and handler function.
     *
     * # Errors
     *
     * Returns an error if route construction fails or if the handler registration fails.
     */
    public function route(RouteBuilder $builder, ?callable $handler = null): mixed
    {
        if ($handler !== null) {
            $this->registrations[] = ['route', [$builder], $handler];
            return $this;
        }

        return function (callable $handler) {
            $this->registrations[] = ['route', [$builder], $handler];
            return $handler;
        };
    }

    // Register a GET route at the given path.
    public function get(string $path, callable $handler): self
    {
        $builder = RouteBuilder::new(Method::Get, $path);
        return $this->route($builder, $handler);
    }

    // Register a GET route at the given path.
    public function getDecorator(string $path): Closure
    {
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Get, $path);
            return $this->route($builder, $handler);
        };
    }

    // Register a POST route at the given path.
    public function post(string $path, callable $handler): self
    {
        $builder = RouteBuilder::new(Method::Post, $path);
        return $this->route($builder, $handler);
    }

    // Register a POST route at the given path.
    public function postDecorator(string $path): Closure
    {
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Post, $path);
            return $this->route($builder, $handler);
        };
    }

    // Register a PUT route at the given path.
    public function put(string $path, callable $handler): self
    {
        $builder = RouteBuilder::new(Method::Put, $path);
        return $this->route($builder, $handler);
    }

    // Register a PUT route at the given path.
    public function putDecorator(string $path): Closure
    {
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Put, $path);
            return $this->route($builder, $handler);
        };
    }

    // Register a PATCH route at the given path.
    public function patch(string $path, callable $handler): self
    {
        $builder = RouteBuilder::new(Method::Patch, $path);
        return $this->route($builder, $handler);
    }

    // Register a PATCH route at the given path.
    public function patchDecorator(string $path): Closure
    {
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Patch, $path);
            return $this->route($builder, $handler);
        };
    }

    // Register a DELETE route at the given path.
    public function delete(string $path, callable $handler): self
    {
        $builder = RouteBuilder::new(Method::Delete, $path);
        return $this->route($builder, $handler);
    }

    // Register a DELETE route at the given path.
    public function deleteDecorator(string $path): Closure
    {
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Delete, $path);
            return $this->route($builder, $handler);
        };
    }

    // Register a HEAD route at the given path.
    public function head(string $path, callable $handler): self
    {
        $builder = RouteBuilder::new(Method::Head, $path);
        return $this->route($builder, $handler);
    }

    // Register a HEAD route at the given path.
    public function headDecorator(string $path): Closure
    {
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Head, $path);
            return $this->route($builder, $handler);
        };
    }

    // Register an OPTIONS route at the given path.
    public function options(string $path, callable $handler): self
    {
        $builder = RouteBuilder::new(Method::Options, $path);
        return $this->route($builder, $handler);
    }

    // Register an OPTIONS route at the given path.
    public function optionsDecorator(string $path): Closure
    {
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Options, $path);
            return $this->route($builder, $handler);
        };
    }

    // Register a CONNECT route at the given path.
    public function connect(string $path, callable $handler): self
    {
        $builder = RouteBuilder::new(Method::Connect, $path);
        return $this->route($builder, $handler);
    }

    // Register a CONNECT route at the given path.
    public function connectDecorator(string $path): Closure
    {
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Connect, $path);
            return $this->route($builder, $handler);
        };
    }

    // Register a TRACE route at the given path.
    public function trace(string $path, callable $handler): self
    {
        $builder = RouteBuilder::new(Method::Trace, $path);
        return $this->route($builder, $handler);
    }

    // Register a TRACE route at the given path.
    public function traceDecorator(string $path): Closure
    {
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Trace, $path);
            return $this->route($builder, $handler);
        };
    }

    /**
     * Run the HTTP server using the configured routes.
     *
     * # Errors
     *
     * Returns an error if server construction or execution fails.
     */
    public function run(): void
    {
        app_run($this->registrations);
    }

    /**
     * Build the underlying Axum router.
     *
     * # Errors
     *
     * Returns an error if server or router construction fails.
     */
    public function into_router(): string
    {
        return app_into_router($this->registrations);
    }

}
