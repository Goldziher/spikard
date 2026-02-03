<?php

declare(strict_types=1);

namespace Spikard\TestApp;

use Spikard\App as SpikardApp;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Attributes\Put;
use Spikard\Attributes\Delete;
use Spikard\Attributes\Patch;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

/**
 * PHP test application for Spikard
 *
 * Tests core functionality:
 * - Health check endpoint
 * - Query parameter handling
 * - JSON request/response
 * - Path parameter extraction
 * - HTTP methods (GET, POST, PUT, DELETE, PATCH)
 * - Header and cookie extraction
 * - Error handling
 */
final class App
{
    /**
     * Create and configure the test application
     */
    public static function createApp(): TestClient
    {
        // Create controller with routes
        $controller = new class {
            #[Get('/health')]
            public function health(): Response
            {
                return Response::json(['status' => 'ok']);
            }

            #[Get('/query')]
            public function query(Request $request): Response
            {
                $query = $request->queryParams ?? [];
                // Query params are arrays from the parser, get the first value
                $name = isset($query['name']) ? ($query['name'][0] ?? null) : null;
                $age = isset($query['age']) ? (int) ($query['age'][0] ?? 0) : null;
                return Response::json([
                    'name' => $name,
                    'age' => $age,
                ]);
            }

            #[Post('/echo')]
            public function echo(Request $request): Response
            {
                // Body is already a parsed array from JSON
                $body = $request->body ?? [];
                return Response::json([
                    'received' => $body,
                    'method' => $request->method,
                ]);
            }

            #[Get('/users/:id')]
            public function user(Request $request): Response
            {
                $userId = $request->pathParams['id'] ?? null;
                return Response::json([
                    'userId' => $userId,
                    'type' => get_debug_type($userId),
                ]);
            }

            #[Put('/items/:id')]
            public function putItem(Request $request): Response
            {
                $itemId = $request->pathParams['id'] ?? null;
                $body = $request->body ?? [];
                return Response::json([
                    'itemId' => $itemId,
                    'updated' => $body,
                    'method' => $request->method,
                ]);
            }

            #[Delete('/items/:id')]
            public function deleteItem(Request $request): Response
            {
                $itemId = $request->pathParams['id'] ?? null;
                return Response::json([
                    'itemId' => $itemId,
                    'deleted' => true,
                    'method' => $request->method,
                ]);
            }

            #[Patch('/items/:id')]
            public function patchItem(Request $request): Response
            {
                $itemId = $request->pathParams['id'] ?? null;
                $body = $request->body ?? [];
                return Response::json([
                    'itemId' => $itemId,
                    'patched' => $body,
                    'method' => $request->method,
                ]);
            }

            #[Get('/headers')]
            public function headers(Request $request): Response
            {
                $customHeader = $request->headers['x-custom-header'] ?? '';
                return Response::json([
                    'x-custom-header' => $customHeader,
                ]);
            }

            #[Get('/cookies')]
            public function cookies(Request $request): Response
            {
                $session = $request->cookies['session'] ?? '';
                return Response::json([
                    'session' => $session,
                ]);
            }

            #[Get('/error')]
            public function error(): Response
            {
                throw new \RuntimeException('Intentional error');
            }
        };

        // Create and configure the app
        $config = new ServerConfig(host: '127.0.0.1', port: 0);
        $app = (new SpikardApp($config))->registerController($controller);

        return TestClient::create($app);
    }
}
