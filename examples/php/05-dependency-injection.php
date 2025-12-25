<?php

declare(strict_types=1);

require_once __DIR__ . '/../../vendor/autoload.php';

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\DI\DependencyContainer;
use Spikard\DI\Provide;
use Spikard\Http\Response;

/**
 * Dependency Injection Example
 *
 * Demonstrates how to register and use dependencies in handlers.
 * Shows both value dependencies (singletons) and factory dependencies.
 */

// Simulated database class
class Database {
    public function __construct(
        public readonly string $host,
        public readonly int $port
    ) {
        error_log("Database connected to {$host}:{$port}");
    }

    public function query(string $sql): array {
        // Simulated query
        return [
            ['id' => 1, 'name' => 'Alice'],
            ['id' => 2, 'name' => 'Bob'],
        ];
    }
}

// Simulated cache class
class Cache {
    public function __construct(
        private readonly Database $db
    ) {
        error_log("Cache initialized with database");
    }

    public function get(string $key): mixed {
        error_log("Cache lookup: {$key}");
        return null; // Simulated miss
    }

    public function set(string $key, mixed $value): void {
        error_log("Cache set: {$key}");
    }
}

$config = new ServerConfig(port: 8000);

// Configure dependency container
$container = new DependencyContainer(
    // Value dependencies (singletons)
    values: [
        'app_name' => 'My Spikard App',
        'db_config' => ['host' => 'localhost', 'port' => 5432],
    ],
    // Factory dependencies (created on-demand)
    factories: [
        'database' => new Provide(
            factory: function (array $db_config): Database {
                return new Database($db_config['host'], $db_config['port']);
            },
            dependsOn: ['db_config'],
            singleton: true
        ),
        'cache' => new Provide(
            factory: function (Database $database): Cache {
                return new Cache($database);
            },
            dependsOn: ['database'],
            singleton: true
        ),
    ]
);

$app = (new App($config))->withDependencies($container);

final class UsersController
{
    #[Get('/users')]
    public function list(): Response
    {
        // For now, dependencies are prepared in Rust
        // Future: function(Database $db) with auto-injection
        return Response::json([
            'users' => [
                ['id' => 1, 'name' => 'Alice'],
                ['id' => 2, 'name' => 'Bob'],
            ]
        ]);
    }
}

// Note: Currently dependencies are registered but not auto-injected
// Full parameter injection is planned for P1
$app = $app->registerController(new UsersController());

echo "Starting server with dependency injection on http://127.0.0.1:8000\n";
echo "Dependencies registered: app_name, db_config, database, cache\n";
echo "Note: P0.2 complete (DI system), P1.4 pending (auto-injection)\n\n";

$app->run();
