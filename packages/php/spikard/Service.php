<?php

declare(strict_types=1);

// Spikard application builder.
class App {
    private array $registrations = [];

    public function __construct() {
        // Create a new application with the default server configuration.
    }

    public function config(ServerConfig $config): self {
        // Set the server configuration.
        $this->_config = $config;
        return $this;
    }

    public function route(RouteBuilder $builder): callable {
        /**
         * Register a route using the provided builder and handler function.
         *
         * # Errors
         *
         * Returns an error if route construction fails or if the handler registration fails.
         */
        return function (callable $handler) {
            $this->registrations[] = ["route", [$builder], $handler];
            return $handler;
        };
    }

    public function register_route(RouteBuilder $builder, callable $handler): self {
        $this->registrations[] = ["route", [$builder], $handler];
        return $this;
    }

    public function get(string $path, callable $handler): callable {
        // Register a GET route at the given path.
        return function (callable $handler) {
            return $this->route()($handler);
        };
    }

    public function post(string $path, callable $handler): callable {
        // Register a POST route at the given path.
        return function (callable $handler) {
            return $this->route()($handler);
        };
    }

    public function put(string $path, callable $handler): callable {
        // Register a PUT route at the given path.
        return function (callable $handler) {
            return $this->route()($handler);
        };
    }

    public function patch(string $path, callable $handler): callable {
        // Register a PATCH route at the given path.
        return function (callable $handler) {
            return $this->route()($handler);
        };
    }

    public function delete(string $path, callable $handler): callable {
        // Register a DELETE route at the given path.
        return function (callable $handler) {
            return $this->route()($handler);
        };
    }

    public function head(string $path, callable $handler): callable {
        // Register a HEAD route at the given path.
        return function (callable $handler) {
            return $this->route()($handler);
        };
    }

    public function options(string $path, callable $handler): callable {
        // Register an OPTIONS route at the given path.
        return function (callable $handler) {
            return $this->route()($handler);
        };
    }

    public function connect(string $path, callable $handler): callable {
        // Register a CONNECT route at the given path.
        return function (callable $handler) {
            return $this->route()($handler);
        };
    }

    public function trace(string $path, callable $handler): callable {
        // Register a TRACE route at the given path.
        return function (callable $handler) {
            return $this->route()($handler);
        };
    }

    public function run(): void {
        /**
         * Run the HTTP server using the configured routes.
         *
         * # Errors
         *
         * Returns an error if server construction or execution fails.
         */
        app_run($this->registrations);
    }

    public function into_router(): string {
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
