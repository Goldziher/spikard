<?php

declare(strict_types=1);

require_once __DIR__ . '/../../vendor/autoload.php';

use Spikard\App;
use Spikard\Background\BackgroundTask;
use Spikard\Config\ServerConfig;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Test background task functionality with Spikard PHP bindings.
 */

echo "=== Background Task Test ===\n\n";

// Create a temporary file to track background execution
$logFile = tempnam(sys_get_temp_dir(), 'bg_test_');
echo "Log file: $logFile\n\n";

// Test 1: Simple background task
echo "Test 1: Simple background task\n";
try {
    BackgroundTask::run(function() use ($logFile) {
        file_put_contents($logFile, "Task executed at " . date('H:i:s') . "\n", FILE_APPEND);
    });
    echo "✓ Task queued successfully\n";
} catch (Exception $e) {
    echo "✗ Failed to queue task: {$e->getMessage()}\n";
}

// Test 2: Background task with parameters
echo "\nTest 2: Background task with parameters\n";
try {
    $userId = 123;
    BackgroundTask::run(function($id, $file) {
        file_put_contents($file, "User ID: $id\n", FILE_APPEND);
    }, [$userId, $logFile]);
    echo "✓ Parameterized task queued successfully\n";
} catch (Exception $e) {
    echo "✗ Failed to queue parameterized task: {$e->getMessage()}\n";
}

// Test 3: Background task in HTTP handler
echo "\nTest 3: Background task in HTTP handler\n";

$config = new ServerConfig(port: 18001);
$app = new App($config);

$app = $app->addRoute('POST', '/users', new class($logFile) implements HandlerInterface {
    public function __construct(private readonly string $logFile) {}

    public function matches(Request $request): bool {
        return true;
    }

    public function handle(Request $request): Response {
        // Synchronous work
        $data = $request->jsonBody();
        $userId = $data['id'] ?? rand(1000, 9999);

        // Queue background work
        BackgroundTask::run(function($userId, $file) {
            // Simulate sending welcome email
            sleep(1);
            file_put_contents($file, "Welcome email sent to user $userId\n", FILE_APPEND);
        }, [$userId, $this->logFile]);

        return Response::json([
            'id' => $userId,
            'message' => 'User created, welcome email queued'
        ]);
    }
});

echo "Starting server on port 18001...\n";
$app->run($config);

// Give server time to start
sleep(1);

// Make a test request
$ch = curl_init('http://127.0.0.1:18001/users');
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
curl_setopt($ch, CURLOPT_POST, true);
curl_setopt($ch, CURLOPT_POSTFIELDS, json_encode(['name' => 'Test User']));
curl_setopt($ch, CURLOPT_HTTPHEADER, ['Content-Type: application/json']);

$response = curl_exec($ch);
$status = curl_getinfo($ch, CURLINFO_HTTP_CODE);
curl_close($ch);

if ($status === 200) {
    echo "✓ HTTP request successful: $response\n";
} else {
    echo "✗ HTTP request failed with status $status\n";
}

// Stop server
$app->close();

// Wait for background tasks to complete
echo "\nWaiting for background tasks to complete...\n";
sleep(2);

// Check log file
echo "\n=== Background Task Log ===\n";
if (file_exists($logFile)) {
    echo file_get_contents($logFile);
    unlink($logFile);
} else {
    echo "✗ Log file not created\n";
}

echo "\n=== Test Complete ===\n";
