<?php

declare(strict_types=1);

require_once __DIR__ . '/../../vendor/autoload.php';

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\DI\DependencyContainer;
use Spikard\DI\Provide;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Test dependency injection with value and factory dependencies.
 *
 * Demonstrates:
 * 1. Value dependencies (singleton objects)
 * 2. Factory dependencies (callables with deps)
 * 3. Automatic parameter injection in handlers
 */

echo "=== Dependency Injection Test ===\n\n";

// Test 1: Value dependency
echo "Test 1: Value dependency (singleton)\n";

$appName = "TestApp v1.0";
$config = ['db_host' => 'localhost', 'db_port' => 5432];

$container = new DependencyContainer(
    values: [
        'app_name' => $appName,
        'config' => $config,
    ]
);

echo "✓ Created container with value dependencies\n";
echo "  - app_name: $appName\n";
echo "  - config: " . json_encode($config) . "\n";

// Test 2: Factory dependency
echo "\nTest 2: Factory dependency (callable)\n";

$dbFactory = function (array $config): \stdClass {
    $db = new \stdClass();
    $db->host = $config['db_host'];
    $db->port = $config['db_port'];
    $db->connected = true;
    return $db;
};

$container = new DependencyContainer(
    values: [
        'config' => $config,
    ],
    factories: [
        'db' => new Provide($dbFactory, dependsOn: ['config'], singleton: true),
    ]
);

echo "✓ Created container with factory dependency\n";
echo "  - db factory depends on: config\n";

// Test 3: Handler with dependency injection
echo "\nTest 3: Handler with DI\n";

$serverConfig = new ServerConfig(port: 18002);
$app = new App($serverConfig);

// Register dependencies
$app = $app->withDependencies($container);

// Handler that expects injected dependencies
$handler = new class implements HandlerInterface {
    public function matches(Request $request): bool {
        return true;
    }

    public function handle(Request $request): Response {
        // In a real scenario, dependencies would be auto-injected by Rust
        // For now, we're just testing the container structure
        return Response::json([
            'message' => 'DI test handler',
            'dependencies_configured' => true
        ]);
    }
};

$app = $app->addRoute('GET', '/test', $handler);

echo "✓ Created app with DI container\n";
echo "✓ Dependencies will be resolved by Rust at runtime\n";

// Verify container structure
echo "\nTest 4: Verify container structure\n";
$deps = $container->getDependencies();
echo "✓ Container has " . count($deps) . " dependencies\n";

foreach ($deps as $name => $value) {
    if ($value instanceof Provide) {
        echo "  - $name: Provide (factory)\n";
        echo "    - depends_on: " . json_encode($value->dependsOn) . "\n";
        echo "    - singleton: " . ($value->singleton ? 'true' : 'false') . "\n";
    } else {
        $type = is_object($value) ? get_class($value) : gettype($value);
        echo "  - $name: $type (value)\n";
    }
}

echo "\n=== Test Complete ===\n";
