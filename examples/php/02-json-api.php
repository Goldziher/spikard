<?php

declare(strict_types=1);

require_once __DIR__ . '/../../vendor/autoload.php';

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * JSON API Example
 *
 * Demonstrates:
 * - Returning JSON responses
 * - Parsing JSON request bodies
 * - Different HTTP methods (GET, POST)
 */

final class UserController
{
    #[Get('/users')]
    public function list(): Response
    {
        return Response::json([
            'users' => [
                ['id' => 1, 'name' => 'Alice', 'email' => 'alice@example.com'],
                ['id' => 2, 'name' => 'Bob', 'email' => 'bob@example.com'],
            ]
        ]);
    }

    #[Post('/users')]
    public function create(Request $request): Response
    {
        $data = $request->body;

        // Validate required fields
        if (!isset($data['name'], $data['email'])) {
            return Response::json([
                'error' => 'Missing required fields: name, email'
            ], 400);
        }

        // Create user (simulated)
        $user = [
            'id' => 3,
            'name' => $data['name'],
            'email' => $data['email'],
        ];

        return Response::json($user, 201);
    }
}

$config = new ServerConfig(port: 8000);
$app = (new App($config))->registerController(new UserController());

echo "Starting JSON API server on http://127.0.0.1:8000\n";
echo "Try:\n";
echo "  curl http://127.0.0.1:8000/users\n";
echo "  curl -X POST http://127.0.0.1:8000/users -H 'Content-Type: application/json' -d '{\"name\":\"Charlie\",\"email\":\"charlie@example.com\"}'\n\n";

$app->run();
