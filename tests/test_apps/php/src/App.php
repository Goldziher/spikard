<?php

declare(strict_types=1);

namespace Spikard\TestApp;

use Spikard\Server;
use Spikard\Request;
use Spikard\Response;

/**
 * PHP test application for Spikard
 *
 * Tests core functionality:
 * - Health check endpoint
 * - Query parameter handling
 * - JSON request/response
 * - Path parameter extraction
 */
final class App
{
    public static function createApp(): Server
    {
        $server = new Server(['host' => '127.0.0.1', 'port' => 0]);

        // Health check
        $server->get('/health', static function (Request $req): Response {
            return new Response(
                status: 200,
                headers: ['Content-Type' => 'application/json'],
                body: json_encode(['status' => 'ok'], JSON_THROW_ON_ERROR)
            );
        });

        // Query parameters
        $server->get('/query', static function (Request $req): Response {
            $params = $req->queryParams() ?? [];
            return new Response(
                status: 200,
                headers: ['Content-Type' => 'application/json'],
                body: json_encode([
                    'name' => $params['name'] ?? null,
                    'age' => isset($params['age']) ? (int)$params['age'] : null,
                ], JSON_THROW_ON_ERROR)
            );
        });

        // JSON echo
        $server->post('/echo', static function (Request $req): Response {
            $body = $req->body() !== null
                ? json_decode($req->body(), true, 512, JSON_THROW_ON_ERROR)
                : [];

            return new Response(
                status: 200,
                headers: ['Content-Type' => 'application/json'],
                body: json_encode([
                    'received' => $body,
                    'method' => $req->method(),
                ], JSON_THROW_ON_ERROR)
            );
        });

        // Path parameters
        $server->get('/users/:id', static function (Request $req): Response {
            $userId = $req->pathParams()['id'] ?? null;
            return new Response(
                status: 200,
                headers: ['Content-Type' => 'application/json'],
                body: json_encode([
                    'userId' => $userId,
                    'type' => get_debug_type($userId),
                ], JSON_THROW_ON_ERROR)
            );
        });

        return $server;
    }
}
