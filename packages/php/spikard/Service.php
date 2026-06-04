<?php

declare(strict_types=1);

// Spikard application builder.
class App
{
    private array $registrations = [];

    public function __construct()
    {
        // Create a new application with the default server configuration.
    }

    public function route(RouteBuilder $builder): callable
    {
        /**
         * Register a route using the provided builder and handler function.
         *
         * # Errors
         *
         * Returns an error if route construction fails or if the handler registration fails.
         */
        return function (callable $handler) {
            $this->registrations[] = ['route', [$builder], $handler];
            return $handler;
        };
    }

    public function register_route(RouteBuilder $builder, callable $handler): self
    {
        $this->registrations[] = ['route', [$builder], $handler];
        return $this;
    }

    public function get(string $path, callable $handler): self
    {
        // Register a GET route at the given path.
        $builder = RouteBuilder::new(Method::Get, $path);
        return $this->route($builder, $handler);
    }

    public function getDecorator(string $path): Closure
    {
        // Register a GET route at the given path.
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Get, $path);
            return $this->route($builder, $handler);
        };
    }

    public function post(string $path, callable $handler): self
    {
        // Register a POST route at the given path.
        $builder = RouteBuilder::new(Method::Post, $path);
        return $this->route($builder, $handler);
    }

    public function postDecorator(string $path): Closure
    {
        // Register a POST route at the given path.
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Post, $path);
            return $this->route($builder, $handler);
        };
    }

    public function put(string $path, callable $handler): self
    {
        // Register a PUT route at the given path.
        $builder = RouteBuilder::new(Method::Put, $path);
        return $this->route($builder, $handler);
    }

    public function putDecorator(string $path): Closure
    {
        // Register a PUT route at the given path.
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Put, $path);
            return $this->route($builder, $handler);
        };
    }

    public function patch(string $path, callable $handler): self
    {
        // Register a PATCH route at the given path.
        $builder = RouteBuilder::new(Method::Patch, $path);
        return $this->route($builder, $handler);
    }

    public function patchDecorator(string $path): Closure
    {
        // Register a PATCH route at the given path.
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Patch, $path);
            return $this->route($builder, $handler);
        };
    }

    public function delete(string $path, callable $handler): self
    {
        // Register a DELETE route at the given path.
        $builder = RouteBuilder::new(Method::Delete, $path);
        return $this->route($builder, $handler);
    }

    public function deleteDecorator(string $path): Closure
    {
        // Register a DELETE route at the given path.
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Delete, $path);
            return $this->route($builder, $handler);
        };
    }

    public function head(string $path, callable $handler): self
    {
        // Register a HEAD route at the given path.
        $builder = RouteBuilder::new(Method::Head, $path);
        return $this->route($builder, $handler);
    }

    public function headDecorator(string $path): Closure
    {
        // Register a HEAD route at the given path.
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Head, $path);
            return $this->route($builder, $handler);
        };
    }

    public function options(string $path, callable $handler): self
    {
        // Register an OPTIONS route at the given path.
        $builder = RouteBuilder::new(Method::Options, $path);
        return $this->route($builder, $handler);
    }

    public function optionsDecorator(string $path): Closure
    {
        // Register an OPTIONS route at the given path.
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Options, $path);
            return $this->route($builder, $handler);
        };
    }

    public function connect(string $path, callable $handler): self
    {
        // Register a CONNECT route at the given path.
        $builder = RouteBuilder::new(Method::Connect, $path);
        return $this->route($builder, $handler);
    }

    public function connectDecorator(string $path): Closure
    {
        // Register a CONNECT route at the given path.
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Connect, $path);
            return $this->route($builder, $handler);
        };
    }

    public function trace(string $path, callable $handler): self
    {
        // Register a TRACE route at the given path.
        $builder = RouteBuilder::new(Method::Trace, $path);
        return $this->route($builder, $handler);
    }

    public function traceDecorator(string $path): Closure
    {
        // Register a TRACE route at the given path.
        return function (callable $handler): self {
            $builder = RouteBuilder::new(Method::Trace, $path);
            return $this->route($builder, $handler);
        };
    }

    public function run(): void
    {
        /**
         * Run the HTTP server using the configured routes.
         *
         * # Errors
         *
         * Returns an error if server construction or execution fails.
         */
        app_run($this->registrations);
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
        return app_into_router($this->registrations);
    }

}
