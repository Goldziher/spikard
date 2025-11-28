<?php

declare(strict_types=1);

require_once __DIR__ . '/../../packages/php/vendor/autoload.php';

use Spikard\App;
use Spikard\Background\BackgroundTask;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Background Tasks Example
 *
 * Demonstrates how to offload work to background tasks without blocking
 * the HTTP response. Perfect for sending emails, logging, analytics, etc.
 */

$config = new ServerConfig(port: 8000);
$app = new App($config);

// Simulated email sending function
function sendWelcomeEmail(array $user): void {
    // Simulate slow email operation
    sleep(2);
    error_log("Sent welcome email to {$user['email']}");
}

// Simulated analytics logging
function logUserCreation(array $user): void {
    sleep(1);
    error_log("Logged user creation: {$user['name']}");
}

$app = $app->addRoute('POST', '/users', function (Request $request) {
    $data = $request->body;

    // Synchronous work (fast)
    $user = [
        'id' => rand(1000, 9999),
        'name' => $data['name'] ?? 'Anonymous',
        'email' => $data['email'] ?? 'unknown@example.com',
        'created_at' => date('Y-m-d H:i:s'),
    ];

    // Background work (slow) - doesn't block the response
    BackgroundTask::run(function () use ($user) {
        sendWelcomeEmail($user);
    });

    BackgroundTask::run(function () use ($user) {
        logUserCreation($user);
    });

    // Return immediately (user doesn't wait for email/logging)
    return Response::json([
        'user' => $user,
        'message' => 'User created. Welcome email will be sent shortly.',
    ], 201);
});

echo "Starting server with background tasks on http://127.0.0.1:8000\n";
echo "POST to /users to see background tasks in action\n";
echo "The response returns immediately while tasks run in background\n\n";

$app->run();
