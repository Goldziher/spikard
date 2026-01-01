<?php

declare(strict_types=1);

namespace E2E\Php;

require_once __DIR__ . '/../route_helper.php';

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Config\CompressionConfig;
use Spikard\Config\RateLimitConfig;
use Spikard\Config\ApiKeyConfig;
use Spikard\Config\JwtConfig;
use Spikard\Config\CorsConfig;
use Spikard\Config\OpenApiConfig;
use Spikard\Handlers\HandlerInterface;
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Generated App factory for PHP e2e tests.
 * Routes are registered with schemas and executed via the native Rust stack.
 */
final class AppFactory
{
    public static function create_sse_notifications_1(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_1());
        return $app;
    }

    public static function create_sse_notifications_2(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_2());
        return $app;
    }

    public static function create_sse_notifications_3(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_3());
        return $app;
    }

    public static function create_sse_notifications_4(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_4());
        return $app;
    }

    public static function create_websocket_systemalert_1(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/systemAlert', new WebSocketHandler_1());
        return $app;
    }

    public static function create_websocket_chat_2(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chat', new WebSocketHandler_2());
        return $app;
    }

    public static function create_websocket_chatack_3(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chatAck', new WebSocketHandler_3());
        return $app;
    }

    public static function create_websocket_chat_4(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chat', new WebSocketHandler_4());
        return $app;
    }

    public static function create_websocket_chat_5(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chat', new WebSocketHandler_5());
        return $app;
    }

    public static function create_websocket_usernotification_6(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/userNotification', new WebSocketHandler_6());
        return $app;
    }

    public static function create_websocket_statusupdate_7(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/statusUpdate', new WebSocketHandler_7());
        return $app;
    }

    public static function create_auth_api_key_authentication_invalid_key_1(): App
    {
        $config = new ServerConfig(
            apiKeyAuth: new ApiKeyConfig(keys: ['sk_test_123456', 'sk_test_789012'], headerName: 'X-API-Key')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'Invalid API key', 'status' => 401, 'detail' => 'The provided API key is not valid'], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"type":"string","required":true}}}', true));
        return $app;
    }

    public static function create_auth_api_key_authentication_missing_header_2(): App
    {
        $config = new ServerConfig(
            apiKeyAuth: new ApiKeyConfig(keys: ['sk_test_123456', 'sk_test_789012'], headerName: 'X-API-Key')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'Missing API key', 'status' => 401, 'detail' => 'Expected \'X-API-Key\' header or \'api_key\' query parameter with valid API key'], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{}', true));
        return $app;
    }

    public static function create_auth_api_key_authentication_valid_key_3(): App
    {
        $config = new ServerConfig(
            apiKeyAuth: new ApiKeyConfig(keys: ['sk_test_123456', 'sk_test_789012'], headerName: 'X-API-Key')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Access granted', 'data' => 'sensitive information'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"type":"string","required":true,"description":"API key for authentication"}}}', true));
        return $app;
    }

    public static function create_auth_api_key_in_query_parameter_4(): App
    {
        $config = new ServerConfig(
            apiKeyAuth: new ApiKeyConfig(keys: ['sk_test_123456', 'sk_test_789012'], headerName: 'X-API-Key')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Access granted', 'data' => 'sensitive information'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data?api_key=sk_test_123456', $handler, null, null, null);
        return $app;
    }

    public static function create_auth_api_key_rotation_old_key_still_valid_5(): App
    {
        $config = new ServerConfig(
            apiKeyAuth: new ApiKeyConfig(keys: ['sk_test_old_123456', 'sk_test_new_789012'], headerName: 'X-API-Key')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Access granted', 'data' => 'sensitive information'], 200, ['X-API-Key-Deprecated' => 'true']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"type":"string","required":true,"description":"API key for authentication"}}}', true));
        return $app;
    }

    public static function create_auth_api_key_with_custom_header_name_6(): App
    {
        $config = new ServerConfig(
            apiKeyAuth: new ApiKeyConfig(keys: ['sk_test_123456', 'sk_test_789012'], headerName: 'X-API-Token')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Access granted', 'data' => 'sensitive information'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Token":{"type":"string","required":true,"description":"API token for authentication"}}}', true));
        return $app;
    }

    public static function create_auth_bearer_token_without_prefix_7(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'Invalid Authorization header format', 'status' => 401, 'detail' => 'Authorization header must use Bearer scheme: \'Bearer <token>\''], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true,"description":"JWT token in Bearer format"}}}', true));
        return $app;
    }

    public static function create_auth_jwt_authentication_expired_token_8(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'JWT validation failed', 'status' => 401, 'detail' => 'Token has expired'], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/protected/user', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true}}}', true));
        return $app;
    }

    public static function create_auth_jwt_authentication_invalid_audience_9(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256', audience: ['https://api.example.com'])
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'JWT validation failed', 'status' => 401, 'detail' => 'Token audience is invalid'], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/protected/user', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true}}}', true));
        return $app;
    }

    public static function create_auth_jwt_authentication_invalid_signature_10(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'JWT validation failed', 'status' => 401, 'detail' => 'Token signature is invalid'], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/protected/user', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true}}}', true));
        return $app;
    }

    public static function create_auth_jwt_authentication_missing_authorization_header_11(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'Missing or invalid Authorization header', 'status' => 401, 'detail' => 'Expected \'Authorization: Bearer <token>\''], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/protected/user', $handler, null, null, json_decode('{}', true));
        return $app;
    }

    public static function create_auth_jwt_authentication_valid_token_12(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256', audience: ['https://api.example.com'], issuer: 'https://auth.example.com')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Access granted', 'user_id' => 'user123'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/protected/user', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true,"description":"JWT token in Bearer format"}}}', true));
        return $app;
    }

    public static function create_auth_jwt_invalid_issuer_13(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256', issuer: 'https://auth.example.com', leeway: 0)
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'JWT validation failed', 'status' => 401, 'detail' => 'Token issuer is invalid, expected \'https://auth.example.com\''], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true,"description":"JWT token in Bearer format"}}}', true));
        return $app;
    }

    public static function create_auth_jwt_malformed_token_format_14(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'Malformed JWT token', 'status' => 401, 'detail' => 'Malformed JWT token: expected 3 parts separated by dots, found 2'], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true,"description":"JWT token in Bearer format"}}}', true));
        return $app;
    }

    public static function create_auth_jwt_missing_required_custom_claims_15(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256', audience: ['https://api.example.com'], issuer: 'https://auth.example.com')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/forbidden', 'title' => 'Forbidden', 'status' => 403, 'detail' => 'Required claims \'role\' and \'permissions\' missing from JWT'], 403, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/admin', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true,"description":"JWT token in Bearer format"}}}', true));
        return $app;
    }

    public static function create_auth_jwt_not_before_claim_in_future_16(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256', leeway: 0)
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unauthorized', 'title' => 'JWT validation failed', 'status' => 401, 'detail' => 'JWT not valid yet, not before claim is in the future'], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true,"description":"JWT token in Bearer format"}}}', true));
        return $app;
    }

    public static function create_auth_jwt_with_multiple_audiences_17(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256', audience: ['https://api.example.com'], issuer: 'https://auth.example.com')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Access granted', 'user_id' => 'user123'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":true,"description":"JWT token in Bearer format"}}}', true));
        return $app;
    }

    public static function create_auth_multiple_authentication_schemes_jwt_precedence_18(): App
    {
        $config = new ServerConfig(
            jwtAuth: new JwtConfig(secret: 'test-secret-key-do-not-use-in-production', algorithm: 'HS256', audience: ['https://api.example.com'], issuer: 'https://auth.example.com'),
            apiKeyAuth: new ApiKeyConfig(keys: ['sk_test_123456', 'sk_test_789012'], headerName: 'X-API-Key')
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Access granted', 'user_id' => 'user123', 'auth_method' => 'jwt'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","required":false,"description":"JWT token in Bearer format"},"X-API-Key":{"type":"string","required":false,"description":"API key for authentication"}}}', true));
        return $app;
    }

    public static function create_background_background_event_logging_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 202, ['content-type' => 'application/json']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/background/events', $handler, json_decode('{"type":"object","properties":{"event":{"type":"string"}},"required":["event"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_background_background_event_logging_second_payload_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 202, ['content-type' => 'application/json']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/background/events', $handler, json_decode('{"type":"object","properties":{"event":{"type":"string"}},"required":["event"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_body_limits_body_over_limit_returns_413_1(): App
    {
        $config = new ServerConfig(
            maxBodySize: 64
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 413, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/body-limit/over', $handler, json_decode('{"type":"object","properties":{"note":{"type":"string"}},"required":["note"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_body_limits_body_under_limit_succeeds_2(): App
    {
        $config = new ServerConfig(
            maxBodySize: 64
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['accepted' => true, 'note' => 'small'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/body-limit/under', $handler, json_decode('{"type":"object","properties":{"note":{"type":"string"}},"required":["note"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_compression_compression_gzip_applied_1(): App
    {
        $config = new ServerConfig(
            compression: new CompressionConfig(gzip: true, brotli: false, minSize: 0, quality: 4)
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Compressed payload', 'payload' => 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'], 200, ['content-encoding' => 'gzip', 'vary' => 'Accept-Encoding']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/compression/gzip', $handler, null, null, null);
        return $app;
    }

    public static function create_compression_compression_payload_below_min_size_is_not_compressed_2(): App
    {
        $config = new ServerConfig(
            compression: new CompressionConfig(gzip: true, brotli: false, minSize: 4096, quality: 6)
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Small payload', 'payload' => 'tiny'], 200, ['content-encoding' => '<<absent>>']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/compression/skip', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_13_json_with_charset_utf16_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unsupported-charset', 'title' => 'Unsupported Charset', 'status' => 415, 'detail' => 'Unsupported charset \'utf-16\' for JSON. Only UTF-8 is supported.'], 415, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object","properties":{"value":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_content_types_14_content_type_case_insensitive_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'test'], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object","required":["name"],"properties":{"name":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_content_types_15_multipart_boundary_required_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'multipart/form-data requires \'boundary\' parameter'], 400, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/upload', $handler, null, null, json_decode('{"files":{"document":{"required":true}}}', true));
        return $app;
    }

    public static function create_content_types_16_text_plain_not_accepted_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unsupported-media-type', 'title' => 'Unsupported Media Type', 'status' => 415, 'detail' => 'Unsupported media type'], 415, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object","required":["data"],"properties":{"data":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_content_types_17_vendor_json_accepted_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['data' => 'value'], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/api/v1/resource', $handler, json_decode('{"type":"object","required":["data"],"properties":{"data":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_content_types_18_content_type_with_multiple_params_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['value' => 'test'], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object","properties":{"value":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_content_types_19_missing_content_type_default_json_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'test'], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object","required":["name"],"properties":{"name":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_content_types_20_content_length_mismatch_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Content-Length header does not match actual body size'], 400, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object","properties":{"value":{"type":"string"}}}', true), null, json_decode('{"headers":{"Content-Length":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_content_types_415_unsupported_media_type_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/unsupported-media-type', 'title' => 'Unsupported Media Type', 'status' => 415, 'detail' => 'Unsupported media type'], 415, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"string"}', true), null, null);
        return $app;
    }

    public static function create_content_types_binary_response_application_octet_stream_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('binary_data_placeholder', 200, ['content-type' => 'application/octet-stream', 'content-disposition' => 'attachment; filename=file.bin']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/download/file.bin', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_csv_response_text_csv_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('id,name,price
1,Item A,10.0
2,Item B,20.0', 200, ['content-type' => 'text/csv; charset=utf-8', 'content-disposition' => 'attachment; filename=data.csv']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/export/data.csv', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_content_negotiation_accept_header_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'name' => 'Item'], 200, ['content-type' => 'application/json']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/accept-test/1', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_content_types_html_response_text_html_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('<html><body><h1>Hello</h1></body></html>', 200, ['content-type' => 'text/html; charset=utf-8']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/html', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_jpeg_image_response_image_jpeg_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('jpeg_binary_data', 200, ['content-type' => 'image/jpeg']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/images/photo.jpg', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_json_response_application_json_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Item', 'price' => 42.0], 200, ['content-type' => 'application/json']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/json', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_json_with_utf_8_charset_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Café', 'emoji' => '☕'], 200, ['content-type' => 'application/json; charset=utf-8']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/unicode', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_pdf_response_application_pdf_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('pdf_binary_data', 200, ['content-disposition' => 'attachment; filename=document.pdf', 'content-type' => 'application/pdf']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/download/document.pdf', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_png_image_response_image_png_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('png_binary_data', 200, ['content-type' => 'image/png']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/images/logo.png', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_plain_text_response_text_plain_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('Hello, World!', 200, ['content-type' => 'text/plain; charset=utf-8']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/text', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_xml_response_application_xml_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('<?xml version="1.0"?><item><name>Item</name><price>42.0</price></item>', 200, ['content-type' => 'application/xml']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/xml', $handler, null, null, null);
        return $app;
    }

    public static function create_cookies_24_cookie_samesite_strict_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/secure', $handler, null, null, json_decode('{"cookies":{"session_id":{"type":"string","required":true,"samesite":"Strict"}}}', true));
        return $app;
    }

    public static function create_cookies_25_cookie_samesite_lax_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/data', $handler, null, null, json_decode('{"cookies":{"tracking":{"type":"string","required":true,"samesite":"Lax"}}}', true));
        return $app;
    }

    public static function create_cookies_26_cookie_secure_flag_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/secure', $handler, null, null, json_decode('{"cookies":{"auth_token":{"type":"string","required":true,"secure":true}}}', true));
        return $app;
    }

    public static function create_cookies_27_cookie_httponly_flag_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/secure', $handler, null, null, json_decode('{"cookies":{"session":{"type":"string","required":true,"httponly":true}}}', true));
        return $app;
    }

    public static function create_cookies_apikey_cookie_authentication_missing_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['cookie', 'key'], 'msg' => 'Field required', 'input' => null]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me/auth', $handler, null, null, json_decode('{"cookies":{"key":{"type":"string","required":true}}}', true));
        return $app;
    }

    public static function create_cookies_apikey_cookie_authentication_success_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'secret'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, json_decode('{"cookies":{"key":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cookies_cookie_regex_pattern_validation_fail_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['cookie', 'tracking_id'], 'msg' => 'String should match pattern \'^[A-Z0-9]{8}$\'', 'input' => 'invalid-format', 'ctx' => ['pattern' => '^[A-Z0-9]{8}$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/cookies/pattern', $handler, null, null, json_decode('{"cookies":{"tracking_id":{"type":"string","pattern":"^[A-Z0-9]{8}$"}}}', true));
        return $app;
    }

    public static function create_cookies_cookie_regex_pattern_validation_success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['tracking_id' => 'ABC12345'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/cookies/pattern', $handler, null, null, json_decode('{"cookies":{"tracking_id":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cookies_cookie_validation_max_length_constraint_fail_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_long', 'loc' => ['cookie', 'session_id'], 'msg' => 'String should have at most 20 characters', 'input' => 'this_cookie_value_is_way_too_long', 'ctx' => ['max_length' => 20]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/cookies/validated', $handler, null, null, json_decode('{"cookies":{"session_id":{"type":"string","maxLength":20}}}', true));
        return $app;
    }

    public static function create_cookies_cookie_validation_min_length_constraint_success_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['token' => 'abc'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/cookies/min-length', $handler, null, null, json_decode('{"cookies":{"token":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cookies_cookie_validation_min_length_failure_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['cookie', 'tracking_id'], 'msg' => 'String should have at least 3 characters', 'input' => '']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"cookies":{"tracking_id":{"type":"string","minLength":3}}}', true));
        return $app;
    }

    public static function create_cookies_multiple_cookies_success_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['session_id' => 'session123', 'fatebook_tracker' => 'tracker456', 'googall_tracker' => 'ga789'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"cookies":{"session_id":{"type":"string","optional":true},"fatebook_tracker":{"type":"string","optional":true},"googall_tracker":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cookies_optional_apikey_cookie_missing_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['msg' => 'Create an account first'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, json_decode('{"cookies":{"key":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cookies_optional_cookie_parameter_missing_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['ads_id' => null], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"cookies":{"ads_id":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cookies_optional_cookie_parameter_success_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['ads_id' => 'abc123'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"cookies":{"ads_id":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cookies_required_cookie_missing_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['cookie', 'session_id'], 'msg' => 'Field required', 'input' => '']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/cookies', $handler, null, null, json_decode('{"cookies":{"session_id":{"type":"string"},"fatebook_tracker":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cookies_response_delete_cookie_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Cookie deleted'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/cookies/delete', $handler, null, null, json_decode('{"cookies":{"session":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cookies_response_multiple_cookies_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Multiple cookies set'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/cookies/multiple', $handler, json_decode('{"type":"object","properties":{"user":{"type":"string"},"session":{"type":"string"}},"required":["user","session"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_cookies_response_session_cookie_no_max_age_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Session cookie set'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/cookies/session', $handler, json_decode('{"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_cookies_response_cookie_with_samesite_lax_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Cookie set with SameSite=Lax'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/cookies/samesite-lax', $handler, json_decode('{"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_cookies_response_cookie_with_samesite_none_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Cookie set with SameSite=None'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/cookies/samesite-none', $handler, json_decode('{"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_cookies_response_cookie_with_samesite_strict_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Cookie set with SameSite=Strict'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/cookies/samesite-strict', $handler, json_decode('{"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_cookies_response_cookie_with_attributes_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Cookie set'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/cookie/set', $handler, null, null, null);
        return $app;
    }

    public static function create_cookies_response_cookie_with_domain_attribute_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Cookie set with domain'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/cookies/set-with-domain', $handler, json_decode('{"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_cookies_response_cookie_with_path_attribute_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Cookie set with path'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/cookies/set-with-path', $handler, json_decode('{"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_cookies_response_set_cookie_basic_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Come to the dark side, we have cookies'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/cookie/', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_06_cors_preflight_method_not_allowed_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 403, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'OPTIONS', '/api/data', $handler, null, null, json_decode('{"headers":{"Origin":{"type":"string","optional":true},"Access-Control-Request-Method":{"type":"string","optional":true},"Access-Control-Request-Headers":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cors_07_cors_preflight_header_not_allowed_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 403, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'OPTIONS', '/api/data', $handler, null, null, json_decode('{"headers":{"Origin":{"type":"string","optional":true},"Access-Control-Request-Method":{"type":"string","optional":true},"Access-Control-Request-Headers":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cors_08_cors_max_age_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 204, ['Access-Control-Allow-Origin' => 'https://example.com', 'Access-Control-Allow-Headers' => 'Content-Type', 'Access-Control-Allow-Methods' => 'POST', 'Access-Control-Max-Age' => '3600']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'OPTIONS', '/api/data', $handler, null, null, json_decode('{"headers":{"Origin":{"type":"string","optional":true},"Access-Control-Request-Method":{"type":"string","optional":true},"Access-Control-Request-Headers":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cors_09_cors_expose_headers_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, ['X-Request-Id' => 'abc123', 'X-Total-Count' => '42', 'Access-Control-Expose-Headers' => 'X-Total-Count, X-Request-Id', 'Access-Control-Allow-Origin' => 'https://example.com']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{"headers":{"Origin":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cors_10_cors_origin_null_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Origin \'null\' is not allowed'], 403, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{"headers":{"Origin":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cors_cors_private_network_access_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 204, ['Vary' => 'Origin', 'Access-Control-Allow-Private-Network' => 'true', 'Access-Control-Allow-Methods' => 'GET, POST', 'Access-Control-Allow-Origin' => 'https://public.example.com']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'OPTIONS', '/api/local-resource', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_vary_header_for_proper_caching_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['data' => 'cacheable resource'], 200, ['Vary' => 'Origin', 'Cache-Control' => 'public, max-age=3600', 'Access-Control-Allow-Origin' => 'https://app.example.com']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/cached-resource', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_multiple_allowed_origins_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['data' => 'resource data'], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://admin.example.com']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_origin_case_sensitivity_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, ['Vary' => 'Origin']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_preflight_for_delete_method_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 204, ['Access-Control-Allow-Methods' => 'GET, POST, PUT, PATCH, DELETE', 'Access-Control-Max-Age' => '3600', 'Access-Control-Allow-Origin' => 'https://app.example.com', 'Vary' => 'Origin']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'OPTIONS', '/api/resource/456', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_preflight_for_put_method_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 204, ['Access-Control-Allow-Methods' => 'GET, POST, PUT, PATCH, DELETE', 'Access-Control-Allow-Headers' => 'Content-Type, X-Custom-Header', 'Access-Control-Allow-Origin' => 'https://app.example.com', 'Vary' => 'Origin', 'Access-Control-Max-Age' => '3600']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'OPTIONS', '/api/resource/123', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_preflight_request_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, ['Access-Control-Max-Age' => '600', 'Access-Control-Allow-Headers' => 'Content-Type, X-Custom-Header', 'Access-Control-Allow-Origin' => 'https://example.com', 'Access-Control-Allow-Methods' => 'GET, POST, PUT, DELETE, OPTIONS']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'OPTIONS', '/items/', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_regex_pattern_matching_for_origins_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['data' => 'resource data'], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://subdomain.example.com']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_request_blocked_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'CORS request from origin \'https://malicious-site.com\' not allowed'], 403, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"headers":{"Origin":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_cors_cors_safelisted_headers_without_preflight_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Success'], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://app.example.com']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/api/form', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_wildcard_origin_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['data' => 'public'], 200, ['Access-Control-Allow-Origin' => '*']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/public/data', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_with_credentials_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'john'], 200, ['Access-Control-Allow-Origin' => 'https://app.example.com', 'Access-Control-Allow-Credentials' => 'true', 'Vary' => 'Origin']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/user/profile', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_simple_cors_request_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['items' => []], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://example.com']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, null);
        return $app;
    }

    public static function create_di_async_factory_dependency_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['pool_status' => 'connected', 'max_size' => 10], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/db-status', $handler, null, null, null);
        return $app;
    }

    public static function create_di_circular_dependency_detection_error_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/dependency-error', 'title' => 'Dependency Resolution Failed', 'status' => 500, 'detail' => 'Circular dependency detected', 'errors' => [['type' => 'circular_dependency', 'msg' => 'Circular dependency detected in dependency graph', 'cycle' => ['service_a', 'service_b', 'service_a']]]], 500, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/circular', $handler, null, null, null);
        return $app;
    }

    public static function create_di_dependency_injection_in_lifecycle_hooks_success_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['authenticated' => true, 'logged' => true], 200, ['X-Log-Level' => 'debug', 'X-Auth-Mode' => 'strict']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/hook-di-test', $handler, null, null, null);
        return $app;
    }

    public static function create_di_factory_dependency_success_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['timestamp' => '<<present>>'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/timestamp', $handler, null, null, null);
        return $app;
    }

    public static function create_di_missing_dependency_error_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/dependency-error', 'title' => 'Dependency Resolution Failed', 'status' => 500, 'detail' => 'Required dependency not found', 'errors' => [['type' => 'missing_dependency', 'msg' => 'Dependency \'non_existent_service\' is not registered', 'dependency_key' => 'non_existent_service']]], 500, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/missing-dep', $handler, null, null, null);
        return $app;
    }

    public static function create_di_mixed_singleton_and_per_request_caching_success_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['app_name' => 'MyApp', 'pool_id' => '<<uuid>>', 'context_id' => '<<uuid>>'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/mixed-caching', $handler, null, null, null);
        return $app;
    }

    public static function create_di_multiple_dependencies_with_cleanup_success_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['session_active' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/multi-cleanup-test', $handler, null, null, null);
        return $app;
    }

    public static function create_di_nested_dependencies_3_levels_success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['auth_enabled' => true, 'has_db' => true, 'has_cache' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/auth-status', $handler, null, null, null);
        return $app;
    }

    public static function create_di_node_js_object_destructuring_injection_success_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['db_name' => 'PostgreSQL', 'log_level' => 'info'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/node-destructure', $handler, null, null, null);
        return $app;
    }

    public static function create_di_per_request_dependency_caching_success_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['first_id' => '<<uuid>>', 'second_id' => '<<same_as:first_id>>'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/request-id', $handler, null, null, null);
        return $app;
    }

    public static function create_di_python_parameter_name_based_injection_success_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['db_status' => 'connected', 'cache_status' => 'ready'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/python-name-inject', $handler, null, null, null);
        return $app;
    }

    public static function create_di_python_type_annotation_based_injection_success_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['pool_type' => 'PostgreSQL', 'cache_type' => 'Redis'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/python-type-inject', $handler, null, null, null);
        return $app;
    }

    public static function create_di_resource_cleanup_after_request_success_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['session_id' => '<<uuid>>', 'status' => 'completed'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/cleanup-test', $handler, null, null, null);
        return $app;
    }

    public static function create_di_route_level_dependency_override_success_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['mode' => 'test', 'strict' => false], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/override-test', $handler, null, null, null);
        return $app;
    }

    public static function create_di_ruby_keyword_argument_injection_success_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['adapter' => 'postgresql', 'user_id' => 42], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/ruby-kwargs', $handler, null, null, null);
        return $app;
    }

    public static function create_di_singleton_dependency_caching_success_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['counter_id' => '<<uuid>>', 'count' => 1], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/app-counter', $handler, null, null, null);
        return $app;
    }

    public static function create_di_type_mismatch_in_dependency_resolution_error_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/dependency-error', 'title' => 'Dependency Resolution Failed', 'status' => 500, 'detail' => 'Dependency type mismatch', 'errors' => [['type' => 'type_mismatch', 'msg' => 'Dependency \'config\' type mismatch: expected object, got string', 'dependency_key' => 'config', 'expected_type' => 'object', 'actual_type' => 'string']]], 500, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/type-mismatch', $handler, null, null, null);
        return $app;
    }

    public static function create_di_value_dependency_injection_success_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['app_name' => 'SpikardApp', 'version' => '1.0.0', 'max_connections' => 100], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/config', $handler, null, null, null);
        return $app;
    }

    public static function create_edge_cases_11_utf8_query_parameter_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['term' => 'café'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/search', $handler, null, null, json_decode('{"query":{"term":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_edge_cases_12_percent_encoded_special_chars_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['term' => 'hi there'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/search?term=hi%20there', $handler, null, null, json_decode('{"query":{"term":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_edge_cases_13_empty_string_query_param_preserved_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['filter' => ''], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items?filter=', $handler, null, null, json_decode('{"query":{"filter":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_edge_cases_14_large_integer_boundary_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 9007199254740991], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"id":{"type":"integer"}}}', true));
        return $app;
    }

    public static function create_edge_cases_15_float_precision_preservation_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['value' => 3.141592653589793], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/calculate', $handler, json_decode('{"type":"object","required":["value"],"properties":{"value":{"type":"number"}}}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_16_negative_zero_handling_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['offset' => 0], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object","required":["offset"],"properties":{"offset":{"type":"number"}}}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_17_extremely_long_string_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_long', 'loc' => ['body', 'content'], 'msg' => 'String should have at most 10000 characters', 'input' => 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa', 'ctx' => ['max_length' => 10000]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/text', $handler, json_decode('{"type":"object","required":["content"],"properties":{"content":{"type":"string","maxLength":10000}}}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_18_unicode_normalization_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'café'], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/users', $handler, json_decode('{"type":"object","required":["name"],"properties":{"name":{"type":"string","minLength":1}}}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_19_emoji_in_strings_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['text' => 'Hello 👋 World 🌍'], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/messages', $handler, json_decode('{"type":"object","required":["text"],"properties":{"text":{"type":"string","minLength":1,"maxLength":100}}}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_20_null_byte_in_string_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['body', 'filename'], 'msg' => 'String should match pattern \'^[^\\x00]+$\'', 'input' => 'file .txt', 'ctx' => ['pattern' => '^[^\\x00]+$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files', $handler, json_decode('{"type":"object","required":["filename"],"properties":{"filename":{"type":"string","pattern":"^[^\\\\x00]+$"}}}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_21_scientific_notation_number_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['value' => 123000], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/calculate', $handler, json_decode('{"type":"object","required":["value"],"properties":{"value":{"type":"number","minimum":0}}}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_22_leading_zeros_integer_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['value' => 123], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/data', $handler, null, null, json_decode('{"query":{"value":{"type":"integer","annotation":"int"}}}', true));
        return $app;
    }

    public static function create_edge_cases_23_deeply_nested_json_limit_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Request body exceeds maximum nesting depth of 32'], 400, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_24_array_with_holes_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['items' => ['first', 'third', 'sixth']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items', $handler, json_decode('{"type":"object","required":["items"],"properties":{"items":{"type":"array","items":{"type":"string"}}}}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_deeply_nested_structure_10_levels_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Processed deeply nested structure', 'max_depth' => 10, 'value_found' => 'deep'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/nested/', $handler, json_decode('{"type":"object","properties":{"level1":{"type":"object","properties":{"level2":{"type":"object","properties":{"level3":{"type":"object","properties":{"level4":{"type":"object","properties":{"level5":{"type":"object","properties":{"level6":{"type":"object","properties":{"level7":{"type":"object","properties":{"level8":{"type":"object","properties":{"level9":{"type":"object","properties":{"level10":{"type":"object","properties":{"value":{"type":"string"},"depth":{"type":"integer"}},"additionalProperties":false,"required":["value","depth"]}},"additionalProperties":false,"required":["level10"]}},"additionalProperties":false,"required":["level9"]}},"additionalProperties":false,"required":["level8"]}},"additionalProperties":false,"required":["level7"]}},"additionalProperties":false,"required":["level6"]}},"additionalProperties":false,"required":["level5"]}},"additionalProperties":false,"required":["level4"]}},"additionalProperties":false,"required":["level3"]}},"additionalProperties":false,"required":["level2"]}},"additionalProperties":false,"required":["level1"]}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_empty_and_null_value_handling_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['explicit_null_is_null' => true, 'empty_string_length' => 0, 'empty_array_length' => 0, 'empty_object_keys' => 0, 'zero_is_falsy' => true, 'false_is_false' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/nulls/', $handler, json_decode('{"type":"object","properties":{"explicit_null":{"type":"null"},"empty_string":{"type":"string"},"empty_array":{"type":"array","items":{}},"empty_object":{"type":"object","properties":{},"additionalProperties":false},"zero_number":{"type":"integer"},"false_boolean":{"type":"boolean"}},"additionalProperties":false,"required":["explicit_null","empty_string","empty_array","empty_object","zero_number","false_boolean"]}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_float_precision_and_rounding_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['sum' => 0.30000000000000004, 'precise_value' => 3.141592653589793, 'very_small' => 1e-10, 'very_large' => 1.7976931348623157e+308], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/calculations/', $handler, json_decode('{"type":"object","properties":{"value1":{"type":"number"},"value2":{"type":"number"},"expected_sum":{"type":"number"},"precise_value":{"type":"number"},"very_small":{"type":"number"},"very_large":{"type":"number"}},"additionalProperties":false,"required":["value1","value2","expected_sum","precise_value","very_small","very_large"]}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_large_integer_boundary_values_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['max_safe_int' => 9007199254740991, 'large_int' => 9223372036854775807, 'negative_large' => -9223372036854775808], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/numbers/', $handler, json_decode('{"type":"object","properties":{"max_safe_int":{"type":"integer"},"large_int":{"type":"integer"},"negative_large":{"type":"integer"}},"additionalProperties":false,"required":["max_safe_int","large_int","negative_large"]}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_special_string_values_and_escaping_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['empty_string' => '', 'whitespace' => '   ', 'tabs_newlines' => 'line1
	line2
line3', 'quotes' => 'He said "hello" and \'goodbye\'', 'backslashes' => 'C:\\\\Users\\\\Path', 'unicode_escapes' => 'Hello', 'special_chars' => '!@#$%^&*()_+-=[]{}|;\':",./<>?'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/strings/', $handler, json_decode('{"type":"object","properties":{"empty_string":{"type":"string"},"whitespace":{"type":"string"},"tabs_newlines":{"type":"string"},"quotes":{"type":"string"},"backslashes":{"type":"string"},"unicode_escapes":{"type":"string"},"special_chars":{"type":"string"}},"additionalProperties":false,"required":["empty_string","whitespace","tabs_newlines","quotes","backslashes","unicode_escapes","special_chars"]}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_unicode_and_emoji_handling_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'name' => 'Coffee Shop ☕', 'description' => 'Best café in München 🇩🇪', 'tags' => ['食べ物', '音楽', '💰'], 'emoji_reactions' => '👍❤️😂🎉'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"description":{"type":"string"},"tags":{"type":"array","items":{"type":"string"}},"emoji_reactions":{"type":"string"}},"additionalProperties":false,"required":["name","description","tags","emoji_reactions"]}', true), null, null);
        return $app;
    }

    public static function create_headers_30_bearer_token_format_valid_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","required":true}}}', true));
        return $app;
    }

    public static function create_headers_31_bearer_token_format_invalid_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['headers', 'authorization'], 'msg' => 'Invalid Bearer token format', 'ctx' => ['pattern' => '^Bearer [A-Za-z0-9-._~+/]+=*$', 'value' => 'Bearer invalid token with spaces']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","required":true}}}', true));
        return $app;
    }

    public static function create_headers_32_bearer_token_missing_prefix_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['headers', 'authorization'], 'msg' => 'Invalid Bearer token format', 'ctx' => ['pattern' => '^Bearer [A-Za-z0-9-._~+/]+=*$', 'value' => 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","required":true}}}', true));
        return $app;
    }

    public static function create_headers_33_api_key_header_valid_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"type":"string","pattern":"^[a-f0-9]{32}$","required":true}}}', true));
        return $app;
    }

    public static function create_headers_34_api_key_header_invalid_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['headers', 'x-api-key'], 'msg' => 'Invalid API key format', 'ctx' => ['pattern' => '^[a-f0-9]{32}$', 'value' => 'invalid-key']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"type":"string","pattern":"^[a-f0-9]{32}$","required":true}}}', true));
        return $app;
    }

    public static function create_headers_accept_header_json_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['accept' => 'application/json'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/accept', $handler, null, null, json_decode('{"headers":{"Accept":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_accept_encoding_header_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['accept_encoding' => 'gzip, deflate, br'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/accept-encoding', $handler, null, null, json_decode('{"headers":{"Accept-Encoding":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_accept_language_header_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['accept_language' => 'en-US,en;q=0.9'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/accept-language', $handler, null, null, json_decode('{"headers":{"Accept-Language":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_authorization_header_missing_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['headers', 'authorization'], 'msg' => 'Field required', 'input' => null]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","annotation":"str","required":true}}}', true));
        return $app;
    }

    public static function create_headers_authorization_header_success_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['scheme' => 'Digest', 'credentials' => 'foobar'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_authorization_header_wrong_scheme_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['headers', 'authorization'], 'msg' => 'String should match pattern \'^Digest .+\'', 'input' => 'Other invalidauthorization']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","annotation":"str","required":true,"pattern":"^Digest .+"}}}', true));
        return $app;
    }

    public static function create_headers_basic_authentication_success_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'username', 'password' => 'password'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/basic-auth', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_bearer_token_authentication_missing_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['headers', 'authorization'], 'msg' => 'Field required', 'input' => null]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/bearer-auth', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","annotation":"str","required":true,"pattern":"^Bearer .+"}}}', true));
        return $app;
    }

    public static function create_headers_bearer_token_authentication_success_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['token' => 'valid_token_123'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/bearer-auth', $handler, null, null, json_decode('{"headers":{"Authorization":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_content_type_header_application_json_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['content_type' => 'application/json'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/content-type', $handler, null, null, json_decode('{"headers":{"Content-Type":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_header_case_insensitivity_access_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['content_type_lower' => 'application/json', 'content_type_upper' => 'application/json', 'content_type_mixed' => 'application/json'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/echo', $handler, json_decode('{"type":"object","properties":{"test":{"type":"string"}},"additionalProperties":false,"required":["test"]}', true), null, null);
        return $app;
    }

    public static function create_headers_header_regex_validation_fail_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['headers', 'x-request-id'], 'msg' => 'String should match pattern \'^[0-9]{3,}$\'', 'input' => 'invalid-format', 'ctx' => ['pattern' => '^[0-9]{3,}$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/pattern', $handler, null, null, json_decode('{"headers":{"X-Request-Id":{"type":"string","annotation":"str","pattern":"^[0-9]{3,}$"}}}', true));
        return $app;
    }

    public static function create_headers_header_regex_validation_success_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['x_request_id' => '12345'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/pattern', $handler, null, null, json_decode('{"headers":{"X-Request-Id":{"type":"string","annotation":"str","pattern":"^[0-9]{3,}$"}}}', true));
        return $app;
    }

    public static function create_headers_header_validation_max_length_constraint_fail_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_long', 'loc' => ['headers', 'x-session-id'], 'msg' => 'String should have at most 20 characters', 'input' => 'this_is_way_too_long_for_validation', 'ctx' => ['max_length' => 20]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/max-length', $handler, null, null, json_decode('{"headers":{"X-Session-Id":{"type":"string","annotation":"str","maxLength":20}}}', true));
        return $app;
    }

    public static function create_headers_header_validation_min_length_constraint_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['headers', 'x-token'], 'msg' => 'String should have at least 3 characters', 'input' => 'ab', 'ctx' => ['min_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/validated', $handler, null, null, json_decode('{"headers":{"X-Token":{"type":"string","annotation":"str","minLength":3}}}', true));
        return $app;
    }

    public static function create_headers_header_with_underscore_conversion_explicit_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['x_token' => 'secret123'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/underscore', $handler, null, null, json_decode('{"headers":{"X-Token":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_host_header_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['host' => 'example.com:8080'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/host', $handler, null, null, json_decode('{"headers":{"Host":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_multiple_custom_headers_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['x_request_id' => 'req-12345', 'x_client_version' => '1.2.3', 'x_trace_id' => 'trace-abc'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/multiple', $handler, null, null, json_decode('{"headers":{"X-Request-Id":{"type":"string","annotation":"str"},"X-Client-Version":{"type":"string","annotation":"str"},"X-Trace-Id":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_multiple_header_values_x_token_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['X-Token values' => ['foo', 'bar']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"headers":{"x-token":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_optional_header_with_none_default_missing_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['strange_header' => null], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"headers":{"strange-header":{"type":"string","annotation":"str","optional":true,"default":null}}}', true));
        return $app;
    }

    public static function create_headers_origin_header_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['origin' => 'https://example.com'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/origin', $handler, null, null, json_decode('{"headers":{"Origin":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_referer_header_27(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['referer' => 'https://example.com/page'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/headers/referer', $handler, null, null, json_decode('{"headers":{"Referer":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_user_agent_header_custom_value_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['User-Agent' => 'Mozilla/5.0 Custom Browser'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"headers":{"User-Agent":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_headers_user_agent_header_default_value_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['User-Agent' => 'testclient'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"headers":{"User-Agent":{"type":"string","annotation":"str","optional":true,"default":"testclient"}}}', true));
        return $app;
    }

    public static function create_headers_x_api_key_optional_header_missing_30(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['msg' => 'Hello World'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, json_decode('{"headers":{"key":{"type":"string","annotation":"str","optional":true}}}', true));
        return $app;
    }

    public static function create_headers_x_api_key_optional_header_success_31(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['msg' => 'Hello secret'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, json_decode('{"headers":{"key":{"type":"string","annotation":"str","optional":true}}}', true));
        return $app;
    }

    public static function create_headers_x_api_key_required_header_missing_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['headers', 'x-api-key'], 'msg' => 'Field required', 'input' => null]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"type":"string","annotation":"str","required":true}}}', true));
        return $app;
    }

    public static function create_headers_x_api_key_required_header_success_33(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'secret'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, json_decode('{"headers":{"key":{"type":"string","annotation":"str","required":true}}}', true));
        return $app;
    }

    public static function create_http_methods_delete_remove_resource_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response((object)[], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'DELETE', '/items/1', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_delete_resource_not_found_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response((object)[], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'DELETE', '/items/999', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_delete_with_response_body_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'name' => 'Deleted Item', 'message' => 'Item deleted successfully'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'DELETE', '/items/1', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_head_get_metadata_without_body_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, ['Content-Length' => '85', 'Content-Type' => 'application/json']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'HEAD', '/items/1', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_options_cors_preflight_request_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, ['Access-Control-Allow-Methods' => 'GET, POST, PUT, DELETE, OPTIONS', 'Access-Control-Allow-Headers' => 'Content-Type', 'Access-Control-Allow-Origin' => 'https://example.com', 'Access-Control-Max-Age' => '86400']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'OPTIONS', '/items/', $handler, null, null, null);
        return $app;
    }

    public static function create_http_methods_patch_partial_update_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'name' => 'Existing Item', 'price' => 79.99, 'in_stock' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'PATCH', '/items/1', $handler, json_decode('{"type":"object","properties":{"price":{"type":"number"}},"required":["price"]}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_patch_update_multiple_fields_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'name' => 'Updated Name', 'price' => 89.99, 'in_stock' => false], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'PATCH', '/items/1', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"in_stock":{"type":"boolean"}},"required":["in_stock","name","price"]}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_put_complete_resource_replacement_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'name' => 'Updated Item', 'description' => 'Completely replaced', 'price' => 99.99, 'in_stock' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'PUT', '/items/1', $handler, json_decode('{"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"},"description":{"type":"string"},"price":{"type":"number"},"in_stock":{"type":"boolean"}},"required":["description","id","in_stock","name","price"]}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_put_create_resource_if_doesn_t_exist_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 999, 'name' => 'New Item', 'price' => 49.99], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'PUT', '/items/999', $handler, json_decode('{"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"number"}},"required":["id","name","price"]}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_put_idempotent_operation_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'name' => 'Fixed Name', 'price' => 50.0], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'PUT', '/items/1', $handler, json_decode('{"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"number"}},"required":["id","name","price"]}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_put_missing_required_field_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['body', 'price'], 'msg' => 'Field required', 'input' => ['id' => 1, 'name' => 'Item Name']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'PUT', '/items/1', $handler, json_decode('{"type":"object","properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"string"}},"required":["price"]}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_put_validation_error_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '2 validation errors in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'input' => 'X', 'ctx' => ['min_length' => 3]], ['type' => 'greater_than', 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than 0', 'input' => -10, 'ctx' => ['gt' => 0]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'PUT', '/items/1', $handler, json_decode('{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","required":["id","name","price"],"properties":{"id":{"type":"integer"},"name":{"type":"string","minLength":3},"price":{"type":"number","exclusiveMinimum":0}}}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_json_bodies_29_nested_object_validation_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/users', $handler, json_decode('{"type":"object","required":["profile"],"properties":{"profile":{"type":"object","required":["name","email"],"properties":{"name":{"type":"string","minLength":1},"email":{"type":"string","format":"email"}}}}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_30_nested_object_missing_field_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['body', 'profile', 'email'], 'msg' => 'Field required', 'input' => ['name' => 'John Doe']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/users', $handler, json_decode('{"type":"object","required":["profile"],"properties":{"profile":{"type":"object","required":["name","email"],"properties":{"name":{"type":"string","minLength":1},"email":{"type":"string","format":"email"}}}}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_31_nullable_property_null_value_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/users', $handler, json_decode('{"type":"object","required":["name"],"properties":{"name":{"type":"string"},"description":{"type":["string","null"]}}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_32_schema_ref_definitions_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/products', $handler, json_decode('{"type":"object","required":["product"],"properties":{"product":{"$ref":"#/definitions/Product"}},"definitions":{"Product":{"type":"object","required":["name","price"],"properties":{"name":{"type":"string"},"price":{"type":"number","minimum":0}}}}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_33_allof_schema_composition_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items', $handler, json_decode('{"allOf":[{"type":"object","required":["name"],"properties":{"name":{"type":"string"}}},{"type":"object","required":["price"],"properties":{"price":{"type":"number","minimum":0}}}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_34_additional_properties_false_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body', 'extra_field'], 'msg' => 'Additional properties are not allowed', 'ctx' => ['additional_properties' => false, 'unexpected_field' => 'extra_field'], 'input' => ['name' => 'John', 'email' => 'john@example.com', 'extra_field' => 'should fail']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/users', $handler, json_decode('{"type":"object","required":["name"],"properties":{"name":{"type":"string"},"email":{"type":"string"}},"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_35_oneof_schema_success_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/payment', $handler, json_decode('{"oneOf":[{"type":"object","required":["credit_card"],"properties":{"credit_card":{"type":"string","pattern":"^[0-9]{16}$"}}},{"type":"object","required":["paypal_email"],"properties":{"paypal_email":{"type":"string","format":"email"}}}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_36_oneof_schema_multiple_match_failure_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body'], 'msg' => '{"credit_card":"1234567812345678","paypal_email":"user@example.com"} is valid under more than one of the schemas listed in the \'oneOf\' keyword', 'input' => ['credit_card' => '1234567812345678', 'paypal_email' => 'user@example.com']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/payment', $handler, json_decode('{"oneOf":[{"type":"object","required":["credit_card"],"properties":{"credit_card":{"type":"string","pattern":"^[0-9]{16}$"}}},{"type":"object","required":["paypal_email"],"properties":{"paypal_email":{"type":"string","format":"email"}}}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_37_oneof_schema_no_match_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body'], 'msg' => '{"bitcoin_address":"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"} is not valid under any of the schemas listed in the \'oneOf\' keyword', 'input' => ['bitcoin_address' => '1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/payment', $handler, json_decode('{"oneOf":[{"type":"object","required":["credit_card"],"properties":{"credit_card":{"type":"string","pattern":"^[0-9]{16}$"}}},{"type":"object","required":["paypal_email"],"properties":{"paypal_email":{"type":"string","format":"email"}}}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_38_anyof_schema_success_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/contact', $handler, json_decode('{"type":"object","required":["name"],"properties":{"name":{"type":"string"}},"anyOf":[{"required":["email"]},{"required":["phone"]}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_39_anyof_schema_multiple_match_success_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/contact', $handler, json_decode('{"type":"object","required":["name"],"properties":{"name":{"type":"string"},"email":{"type":"string","format":"email"},"phone":{"type":"string"}},"anyOf":[{"required":["email"]},{"required":["phone"]}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_40_anyof_schema_failure_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body'], 'msg' => '{"name":"John Doe"} is not valid under any of the schemas listed in the \'anyOf\' keyword', 'input' => ['name' => 'John Doe']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/contact', $handler, json_decode('{"type":"object","required":["name"],"properties":{"name":{"type":"string"},"email":{"type":"string","format":"email"},"phone":{"type":"string"}},"anyOf":[{"required":["email"]},{"required":["phone"]}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_41_not_schema_success_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/users', $handler, json_decode('{"type":"object","required":["username"],"properties":{"username":{"type":"string","not":{"enum":["admin","root","system"]}}}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_42_not_schema_failure_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body', 'username'], 'msg' => '{"enum":["admin","root","system"]} is not allowed for "admin"', 'input' => 'admin']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/users', $handler, json_decode('{"type":"object","required":["username"],"properties":{"username":{"type":"string","not":{"enum":["admin","root","system"]}}}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_43_const_validation_success_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/api/v1/data', $handler, json_decode('{"type":"object","required":["version","data"],"properties":{"version":{"type":"string","const":"1.0"},"data":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_44_const_validation_failure_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body', 'version'], 'msg' => '"1.0" was expected', 'input' => '2.0']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/api/v1/data', $handler, json_decode('{"type":"object","required":["version","data"],"properties":{"version":{"type":"string","const":"1.0"},"data":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_45_minproperties_validation_success_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/config', $handler, json_decode('{"type":"object","minProperties":2}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_46_minproperties_validation_failure_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body'], 'msg' => '{"host":"localhost"} has less than 2 properties', 'input' => ['host' => 'localhost']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/config', $handler, json_decode('{"type":"object","minProperties":2}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_47_maxproperties_validation_failure_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body'], 'msg' => '{"debug":false,"host":"localhost","port":8080,"ssl":true} has more than 3 properties', 'input' => ['host' => 'localhost', 'port' => 8080, 'ssl' => true, 'debug' => false]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/config', $handler, json_decode('{"type":"object","maxProperties":3}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_48_dependencies_validation_success_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/billing', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"credit_card":{"type":"string"},"billing_address":{"type":"string"}},"dependencies":{"credit_card":["billing_address"]}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_49_dependencies_validation_failure_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body'], 'msg' => '"billing_address" is a required property', 'input' => ['name' => 'John Doe', 'credit_card' => '1234567812345678']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/billing', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"credit_card":{"type":"string"},"billing_address":{"type":"string"}},"dependencies":{"credit_card":["billing_address"]}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_50_deep_nesting_4_levels_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object","required":["user"],"properties":{"user":{"type":"object","required":["profile"],"properties":{"profile":{"type":"object","required":["contact"],"properties":{"contact":{"type":"object","required":["address"],"properties":{"address":{"type":"object","required":["street"],"properties":{"street":{"type":"string"}}}}}}}}}}}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_array_of_objects_success_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Product Bundle', 'tags' => ['electronics', 'gadget'], 'images' => [['url' => 'https://example.com/img1.jpg', 'name' => 'Front'], ['url' => 'https://example.com/img2.jpg', 'name' => 'Back']]], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/list', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"tags":{"type":"array","items":{"type":"string"}},"images":{"type":"array","items":{"type":"object","properties":{"url":{"type":"string"},"name":{"type":"string"}},"additionalProperties":false,"required":["url","name"]}}},"additionalProperties":false,"required":["name","tags","images"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_array_of_primitive_values_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Product', 'tags' => ['electronics', 'gadget', 'new'], 'ratings' => [4.5, 4.8, 5.0, 4.2]], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"tags":{"type":"array","items":{"type":"string"}},"ratings":{"type":"array","items":{"type":"number"}}},"additionalProperties":false,"required":["name","tags","ratings"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_body_with_query_parameters_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item' => ['name' => 'Item', 'price' => 42.0], 'limit' => 10], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/?limit=10', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]}', true), null, json_decode('{"query":{"limit":{"type":"integer"}}}', true));
        return $app;
    }

    public static function create_json_bodies_boolean_field_success_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Item', 'price' => 42.0, 'in_stock' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"in_stock":{"type":"boolean"}},"additionalProperties":false,"required":["name","price","in_stock"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_date_field_success_27(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Conference', 'event_date' => '2024-03-15'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/events/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"event_date":{"type":"string"}},"additionalProperties":false,"required":["name","event_date"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_datetime_field_success_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Meeting', 'created_at' => '2024-03-15T10:30:00Z'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/events/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"created_at":{"type":"string","format":"date-time"}},"additionalProperties":false,"required":["name","created_at"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_deeply_nested_objects_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Product', 'price' => 100.0, 'seller' => ['name' => 'John Doe', 'address' => ['street' => '123 Main St', 'city' => 'Springfield', 'country' => ['name' => 'USA', 'code' => 'US']]]], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/nested', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"seller":{"type":"object","properties":{"name":{"type":"string"},"address":{"type":"object","properties":{"street":{"type":"string"},"city":{"type":"string"},"country":{"type":"object","properties":{"name":{"type":"string"},"code":{"type":"string"}},"additionalProperties":false,"required":["name","code"]}},"additionalProperties":false,"required":["street","city","country"]}},"additionalProperties":false,"required":["name","address"]}},"additionalProperties":false,"required":["name","price","seller"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_empty_json_object_30(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => null, 'description' => null, 'price' => null, 'tax' => null], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/optional-all', $handler, json_decode('{"type":"object","properties":{},"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_empty_array_validation_fail_31(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'too_short', 'loc' => ['body', 'tags'], 'msg' => 'List should have at least 1 item after validation', 'input' => [], 'ctx' => ['min_length' => 1]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/list-validated', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"tags":{"type":"array","items":{},"minItems":1}},"additionalProperties":false,"required":["name","tags"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_enum_field_invalid_value_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'enum', 'loc' => ['body', 'category'], 'msg' => 'Input should be \'electronics\', \'clothing\' or \'books\'', 'input' => 'furniture', 'ctx' => ['expected' => '\'electronics\', \'clothing\' or \'books\'']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"category":{"type":"string","enum":["electronics","clothing","books"]}},"additionalProperties":false,"required":["name","category"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_enum_field_success_33(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Item', 'category' => 'electronics'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"category":{"type":"string"}},"additionalProperties":false,"required":["name","category"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_extra_fields_ignored_no_additionalproperties_34(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Item', 'price' => 42.0], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"extra_field":{"type":"string"},"another_extra":{"type":"integer"}},"additionalProperties":false,"required":["name","price","extra_field","another_extra"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_field_type_validation_invalid_type_35(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'float_parsing', 'loc' => ['body', 'price'], 'msg' => 'Input should be a valid number, unable to parse string as a number', 'input' => 'not a number']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"description":{"type":"string"},"price":{"type":"number"},"tax":{"type":"number"}},"additionalProperties":false,"required":["name","description","price","tax"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_nested_object_success_36(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Foo', 'price' => 42.0, 'image' => ['url' => 'https://example.com/image.jpg', 'name' => 'Product Image']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/nested', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"image":{"type":"object","properties":{"url":{"type":"string"},"name":{"type":"string"}},"additionalProperties":false,"required":["url","name"]}},"additionalProperties":false,"required":["name","price","image"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_null_value_for_optional_field_37(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Item', 'price' => 42.0, 'description' => null, 'tax' => null], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"description":{"type":"null"},"tax":{"type":"null"}},"additionalProperties":false,"required":["name","price","description","tax"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_numeric_ge_validation_fail_38(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'greater_than_equal', 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than or equal to 1', 'input' => 0.5, 'ctx' => ['ge' => 1]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/validated', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number","minimum":1}},"additionalProperties":false,"required":["name","price"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_numeric_le_validation_success_39(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Item', 'price' => 100.0], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/validated', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_optional_fields_omitted_40(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Foo', 'price' => 35.4, 'description' => null, 'tax' => null], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_patch_partial_update_41(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Original Item', 'price' => 45.0, 'description' => 'Original description'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'PATCH', '/items/1', $handler, json_decode('{"type":"object","properties":{"price":{"type":"number"}},"required":["price"]}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_json_bodies_required_field_missing_validation_error_42(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['body', 'name'], 'msg' => 'Field required', 'input' => ['description' => 'A very nice Item', 'price' => 35.4]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"description":{"type":"string"},"price":{"type":"number"},"name":{"type":"string"}},"additionalProperties":false,"required":["description","price","name"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_simple_json_object_success_43(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Foo', 'description' => 'A very nice Item', 'price' => 35.4, 'tax' => 3.2], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"description":{"type":"string"},"price":{"type":"number"},"tax":{"type":"number"}},"additionalProperties":false,"required":["name","description","price","tax"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_string_max_length_validation_fail_44(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_long', 'loc' => ['body', 'name'], 'msg' => 'String should have at most 50 characters', 'input' => 'This is a very long name that exceeds the maximum length', 'ctx' => ['max_length' => 50]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/validated', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string","maxLength":50},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_string_min_length_validation_fail_45(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'input' => 'ab', 'ctx' => ['min_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/validated', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string","minLength":3},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_string_pattern_validation_fail_46(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['body', 'sku'], 'msg' => 'String should match pattern \'^[A-Z]{3}[0-9]{4}$\'', 'input' => 'ABC-123', 'ctx' => ['pattern' => '^[A-Z]{3}[0-9]{4}$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/validated', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"sku":{"type":"string","pattern":"^[A-Z]{3}[0-9]{4}$"}},"additionalProperties":false,"required":["name","sku"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_string_pattern_validation_success_47(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Item', 'sku' => 'ABC1234'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/validated', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"sku":{"type":"string"}},"additionalProperties":false,"required":["name","sku"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_uuid_field_invalid_format_48(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'uuid_parsing', 'loc' => ['body', 'item_id'], 'msg' => 'Input should be a valid UUID', 'input' => 'not-a-valid-uuid']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"item_id":{"type":"string","format":"uuid"}},"additionalProperties":false,"required":["name","item_id"]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_uuid_field_success_49(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Item', 'item_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"item_id":{"type":"string","format":"uuid"}},"additionalProperties":false,"required":["name","item_id"]}', true), null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_hook_execution_order_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Hooks executed in order', 'execution_order' => ['first_hook', 'second_hook', 'third_hook']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/test-hook-order', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_multiple_hooks_all_phases_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Action completed successfully', 'user_id' => 'user-123', 'action' => 'update_profile', 'request_id' => '.*'], 200, ['X-Request-ID' => '.*', 'X-Frame-Options' => 'DENY', 'X-Response-Time' => '.*ms', 'X-Content-Type-Options' => 'nosniff']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/api/full-lifecycle', $handler, json_decode('{"type":"object","properties":{"user_id":{"type":"string"},"action":{"type":"string"}},"required":["user_id","action"]}', true), null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_onerror_error_logging_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Internal Server Error', 'message' => 'An unexpected error occurred', 'error_id' => '.*'], 500, ['Content-Type' => 'application/json']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/test-error', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_onrequest_request_logging_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'onRequest hooks executed', 'request_logged' => true, 'has_request_id' => true], 200, ['X-Request-ID' => '.*']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/test-on-request', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_onresponse_response_timing_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Response with timing info'], 200, ['X-Response-Time' => '.*ms']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/test-timing', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_onresponse_security_headers_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Response with security headers'], 200, ['X-Content-Type-Options' => 'nosniff', 'X-Frame-Options' => 'DENY', 'X-XSS-Protection' => '1; mode=block', 'Strict-Transport-Security' => 'max-age=31536000; includeSubDomains']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/test-security-headers', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_prehandler_authentication_failed_short_circuit_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Unauthorized', 'message' => 'Invalid or expired authentication token'], 401, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/protected-resource-fail', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_prehandler_authentication_success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Access granted', 'user_id' => 'user-123', 'authenticated' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/protected-resource', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_prehandler_authorization_check_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Admin access granted', 'user_id' => 'admin-456', 'role' => 'admin'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/admin-only', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Forbidden', 'message' => 'Admin role required for this endpoint'], 403, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/admin-only-forbidden', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Rate limit exceeded', 'message' => 'Too many requests, please try again later'], 429, ['Retry-After' => '60']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/api/test-rate-limit-exceeded', $handler, json_decode('{"type":"object","properties":{"data":{"type":"string"}},"required":["data"]}', true), null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_prevalidation_rate_limiting_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Request accepted', 'rate_limit_checked' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/api/test-rate-limit', $handler, json_decode('{"type":"object","properties":{"data":{"type":"string"}},"required":["data"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_17_file_magic_number_png_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/upload', $handler, null, null, json_decode('{"files":{"image":{"required":true,"content_type":["image/png"],"validate_magic_numbers":true}}}', true));
        return $app;
    }

    public static function create_multipart_18_file_magic_number_jpeg_success_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/upload', $handler, null, null, json_decode('{"files":{"image":{"required":true,"content_type":["image/jpeg"],"validate_magic_numbers":true}}}', true));
        return $app;
    }

    public static function create_multipart_19_file_mime_spoofing_png_as_jpeg_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['files', 'image'], 'msg' => 'File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png', 'ctx' => ['declared_mime' => 'image/jpeg', 'detected_type' => 'image/png', 'magic_bytes' => '89504e470d0a1a0a']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/upload', $handler, null, null, json_decode('{"files":{"image":{"required":true,"content_type":["image/jpeg"],"validate_magic_numbers":true}}}', true));
        return $app;
    }

    public static function create_multipart_20_file_mime_spoofing_jpeg_as_png_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['files', 'image'], 'msg' => 'File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg', 'ctx' => ['declared_mime' => 'image/png', 'detected_type' => 'image/jpeg', 'magic_bytes' => 'ffd8ffe0']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/upload', $handler, null, null, json_decode('{"files":{"image":{"required":true,"content_type":["image/png"],"validate_magic_numbers":true}}}', true));
        return $app;
    }

    public static function create_multipart_21_file_pdf_magic_number_success_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/upload', $handler, null, null, json_decode('{"files":{"document":{"required":true,"content_type":["application/pdf"],"validate_magic_numbers":true}}}', true));
        return $app;
    }

    public static function create_multipart_22_file_empty_buffer_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['files', 'file'], 'msg' => 'File buffer is empty', 'ctx' => ['buffer_size' => 0]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/upload', $handler, null, null, json_decode('{"files":{"file":{"required":true,"validate_magic_numbers":true}}}', true));
        return $app;
    }

    public static function create_multipart_content_type_validation_invalid_type_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files/images-only', $handler, json_decode('{"type":"object","properties":{"file":{"type":"string","format":"binary"}},"additionalProperties":false}', true), null, json_decode('{"files":{"file":{"required":true,"content_type":["image/jpeg","image/png","image/gif"]}}}', true));
        return $app;
    }

    public static function create_multipart_empty_file_upload_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['filename' => 'empty.txt', 'size' => 0], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files/upload', $handler, json_decode('{"type":"object","properties":{"file":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["file"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_file_list_upload_array_of_files_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['filenames' => ['file1.txt', 'file2.txt'], 'total_size' => 35], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files/list', $handler, json_decode('{"type":"object","properties":{"files":{"type":"array","items":{"type":"string","format":"binary"}}},"additionalProperties":false,"required":["files"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_file_size_validation_too_large_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'File too large. Maximum size is 1MB'], 413, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files/validated', $handler, json_decode('{"type":"object","properties":{"file":{"type":"string","format":"binary"}},"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_multipart_file_upload_with_custom_headers_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['test2' => ['filename' => 'test2.txt', 'size' => 15, 'content' => '<file2 content>', 'content_type' => 'text/plain', 'headers' => [['content-disposition', 'form-data; name="test2"; filename="test2.txt"'], ['content-type', 'text/plain'], ['x-custom', 'f2']]]], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/', $handler, json_decode('{"type":"object","properties":{"test2":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["test2"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_file_upload_without_filename_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['test1' => '<file1 content>'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/', $handler, json_decode('{"type":"object","properties":{"test1":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["test1"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_form_data_without_files_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['some' => 'data'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/', $handler, json_decode('{"type":"object","properties":{"some":{"type":"string"}},"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_multipart_image_file_upload_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['filename' => 'photo.jpg', 'content_type' => 'image/jpeg', 'size' => 22], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files/image', $handler, json_decode('{"type":"object","properties":{"image":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["image"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_mixed_files_and_form_data_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['file' => ['filename' => 'upload.txt', 'size' => 14, 'content' => 'file data here', 'content_type' => 'text/plain'], 'username' => 'testuser', 'age' => '25', 'active' => 'true'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/', $handler, json_decode('{"type":"object","properties":{"file":{"type":"string","format":"binary"},"username":{"type":"string"},"age":{"type":"string"},"active":{"type":"string"}},"additionalProperties":false,"required":["file"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_multiple_file_uploads_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['test1' => ['filename' => 'test1.txt', 'size' => 15, 'content' => '<file1 content>', 'content_type' => 'text/plain'], 'test2' => ['filename' => 'test2.txt', 'size' => 15, 'content' => '<file2 content>', 'content_type' => 'text/plain']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/', $handler, json_decode('{"type":"object","properties":{"test1":{"type":"string","format":"binary"},"test2":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["test1","test2"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_multiple_values_for_same_field_name_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['files' => [['filename' => 'file1.txt', 'size' => 10, 'content' => 'first file', 'content_type' => 'text/plain'], ['filename' => 'file2.txt', 'size' => 11, 'content' => 'second file', 'content_type' => 'text/plain']], 'tags' => ['python', 'rust', 'web']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/', $handler, json_decode('{"type":"object","properties":{"files":{"type":"array","items":{"type":"string","format":"binary"}},"tags":{"type":"array","items":{"type":"string"}}},"additionalProperties":false,"required":["files"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_optional_file_upload_missing_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['file' => null], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files/optional', $handler, json_decode('{"type":"object","properties":{},"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_multipart_optional_file_upload_provided_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['filename' => 'optional.txt', 'content_type' => 'text/plain', 'size' => 21], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files/optional', $handler, json_decode('{"type":"object","properties":{"file":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["file"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_pdf_file_upload_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['filename' => 'report.pdf', 'content_type' => 'application/pdf', 'size' => 16], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files/document', $handler, json_decode('{"type":"object","properties":{"document":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["document"]}', true), null, null);
        return $app;
    }

    public static function create_multipart_required_file_upload_missing_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['body', 'file'], 'msg' => 'Field required', 'input' => []]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/files/required', $handler, json_decode('{"type":"object","properties":{"file":{"type":"string","format":"binary"}},"required":["file"],"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_multipart_simple_file_upload_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['test' => ['filename' => 'test.txt', 'size' => 14, 'content' => '<file content>', 'content_type' => 'text/plain']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/', $handler, json_decode('{"type":"object","properties":{"test":{"type":"string","format":"binary"}},"additionalProperties":false,"required":["test"]}', true), null, null);
        return $app;
    }

    public static function create_path_params_20_uuid_v3_path_param_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 'e8b5a51d-11c8-3310-a6ab-367563f20686'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/e8b5a51d-11c8-3310-a6ab-367563f20686', $handler, null, null, json_decode('{"path":{"id":{"type":"string","format":"uuid","uuidVersion":"3"}}}', true));
        return $app;
    }

    public static function create_path_params_21_uuid_v5_path_param_success_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => '630eb68f-e0fa-5ecc-887a-7c7a62614681'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/630eb68f-e0fa-5ecc-887a-7c7a62614681', $handler, null, null, json_decode('{"path":{"id":{"type":"string","format":"uuid","uuidVersion":"5"}}}', true));
        return $app;
    }

    public static function create_path_params_24_date_format_path_param_success_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['date' => '2025-10-30'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/events/2025-10-30', $handler, null, null, json_decode('{"path":{"date":{"type":"string","format":"date"}}}', true));
        return $app;
    }

    public static function create_path_params_25_date_format_invalid_failure_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['path', 'date'], 'msg' => 'Invalid date format', 'ctx' => ['format' => 'date', 'value' => '2025-13-45']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/events/2025-13-45', $handler, null, null, json_decode('{"path":{"date":{"type":"string","format":"date","required":true}}}', true));
        return $app;
    }

    public static function create_path_params_27_datetime_format_path_param_success_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['timestamp' => '2025-10-30T14:30:00Z'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/bookings/2025-10-30T14:30:00Z', $handler, null, null, json_decode('{"path":{"timestamp":{"type":"string","format":"date-time"}}}', true));
        return $app;
    }

    public static function create_path_params_28_duration_format_path_param_success_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['duration' => 'P1DT2H30M'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/delays/P1DT2H30M', $handler, null, null, json_decode('{"path":{"duration":{"type":"string","format":"duration"}}}', true));
        return $app;
    }

    public static function create_path_params_29_decimal_path_param_success_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['amount' => '19.99'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/prices/19.99', $handler, null, null, json_decode('{"path":{"amount":{"type":"string","format":"decimal"}}}', true));
        return $app;
    }

    public static function create_path_params_30_string_minlength_path_success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'alice'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/alice', $handler, null, null, json_decode('{"path":{"username":{"type":"string","minLength":3}}}', true));
        return $app;
    }

    public static function create_path_params_31_string_minlength_path_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['path', 'username'], 'msg' => 'String length must be at least 3', 'ctx' => ['min_length' => 3, 'actual_length' => 2]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/ab', $handler, null, null, json_decode('{"path":{"username":{"type":"string","minLength":3,"required":true}}}', true));
        return $app;
    }

    public static function create_path_params_32_string_maxlength_path_failure_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['path', 'username'], 'msg' => 'String length must not exceed 20', 'ctx' => ['max_length' => 20, 'actual_length' => 42]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/this_username_is_way_too_long_to_be_valid', $handler, null, null, json_decode('{"path":{"username":{"type":"string","maxLength":20,"required":true}}}', true));
        return $app;
    }

    public static function create_path_params_33_string_pattern_path_success_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['owner' => 'spikard-labs', 'repo' => 'spikard-http'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/repos/spikard-labs/spikard-http', $handler, null, null, json_decode('{"path":{"owner":{"type":"string","pattern":"^[a-zA-Z0-9-]+$"},"repo":{"type":"string","pattern":"^[a-zA-Z0-9-_]+$"}}}', true));
        return $app;
    }

    public static function create_path_params_34_string_pattern_path_failure_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['path', 'owner'], 'msg' => 'String does not match pattern', 'ctx' => ['pattern' => '^[a-zA-Z0-9-]+$', 'value' => 'invalid@owner']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/repos/invalid@owner', $handler, null, null, json_decode('{"path":{"owner":{"type":"string","pattern":"^[a-zA-Z0-9-]+$","required":true}}}', true));
        return $app;
    }

    public static function create_path_params_35_negative_integer_path_param_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['value' => -100], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/offset/-100', $handler, null, null, json_decode('{"path":{"value":{"type":"integer"}}}', true));
        return $app;
    }

    public static function create_path_params_boolean_path_parameter_true_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/bool/True', $handler, null, null, json_decode('{"path":{"item_id":{"type":"boolean"}}}', true));
        return $app;
    }

    public static function create_path_params_boolean_path_parameter_numeric_1_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/bool/1', $handler, null, null, json_decode('{"path":{"item_id":{"type":"boolean"}}}', true));
        return $app;
    }

    public static function create_path_params_date_path_parameter_success_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['date_param' => '2023-07-15'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/date/2023-07-15', $handler, null, null, json_decode('{"path":{"date_param":{"type":"string","format":"date"}}}', true));
        return $app;
    }

    public static function create_path_params_enum_path_parameter_invalid_value_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'enum', 'loc' => ['path', 'model_name'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'input' => 'foo', 'ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\'']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/models/foo', $handler, null, null, json_decode('{"path":{"model_name":{"type":"string","enum":["alexnet","resnet","lenet"]}}}', true));
        return $app;
    }

    public static function create_path_params_enum_path_parameter_success_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['model_name' => 'alexnet'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/models/alexnet', $handler, null, null, json_decode('{"path":{"model_name":{"type":"string","enum":["alexnet","lenet","resnet"]}}}', true));
        return $app;
    }

    public static function create_path_params_float_path_parameter_success_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 42.5], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/float/42.5', $handler, null, null, json_decode('{"path":{"item_id":{"type":"number"}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_invalid_string_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'int_parsing', 'loc' => ['path', 'item_id'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'input' => 'foobar']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/int/foobar', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer"}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_success_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 42], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/int/42', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer"}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 2], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/param-lt-gt/2', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer","exclusiveMinimum":1,"exclusiveMaximum":3}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_with_ge_constraint_success_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 3], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/param-ge/3', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer","minimum":3}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_with_gt_constraint_failure_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'greater_than', 'loc' => ['path', 'item_id'], 'msg' => 'Input should be greater than 3', 'input' => 2, 'ctx' => ['gt' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/param-gt/2', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer","exclusiveMinimum":3}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_with_gt_constraint_success_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 42], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/param-gt/42', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer","exclusiveMinimum":3}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_with_le_constraint_success_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 3], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/param-le/3', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer","maximum":3}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_with_lt_constraint_success_27(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 2], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/param-lt/2', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer","exclusiveMaximum":3}}}', true));
        return $app;
    }

    public static function create_path_params_multiple_path_parameters_success_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['version' => 1.0, 'service_id' => 1, 'user_id' => 'abc', 'order_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716', $handler, null, null, json_decode('{"path":{"version":{"type":"number"},"service_id":{"type":"integer"},"user_id":{"type":"string"},"order_id":{"type":"string","format":"uuid"}}}', true));
        return $app;
    }

    public static function create_path_params_path_parameter_type_syntax_invalid_uuid_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'uuid_parsing', 'loc' => ['path', 'id'], 'msg' => 'Input should be a valid UUID', 'input' => 'not-a-uuid']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/type-syntax/items/not-a-uuid', $handler, null, null, null);
        return $app;
    }

    public static function create_path_params_path_parameter_type_syntax_with_override_30(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['count' => '50'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/type-syntax/items-count/50', $handler, null, null, json_decode('{"path":{"count":{"type":"integer","minimum":1,"maximum":100}}}', true));
        return $app;
    }

    public static function create_path_params_path_parameter_with_type_syntax_uuid_31(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => '550e8400-e29b-41d4-a716-446655440000'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/type-syntax/items/550e8400-e29b-41d4-a716-446655440000', $handler, null, null, null);
        return $app;
    }

    public static function create_path_params_path_parameter_with_type_syntax_integer_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['user_id' => '42'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/type-syntax/users/42', $handler, null, null, null);
        return $app;
    }

    public static function create_path_params_path_type_parameter_file_path_33(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['file_path' => 'home/johndoe/myfile.txt'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/files/home/johndoe/myfile.txt', $handler, null, null, json_decode('{"path":{"file_path":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_path_params_string_path_parameter_success_34(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 'foobar'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/str/foobar', $handler, null, null, json_decode('{"path":{"item_id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_path_params_string_path_parameter_with_max_length_failure_35(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_long', 'loc' => ['path', 'item_id'], 'msg' => 'String should have at most 3 characters', 'input' => 'foobar', 'ctx' => ['max_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/param-maxlength/foobar', $handler, null, null, json_decode('{"path":{"item_id":{"type":"string","maxLength":3}}}', true));
        return $app;
    }

    public static function create_path_params_string_path_parameter_with_min_length_failure_36(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['path', 'item_id'], 'msg' => 'String should have at least 3 characters', 'input' => 'fo', 'ctx' => ['min_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/path/param-minlength/fo', $handler, null, null, json_decode('{"path":{"item_id":{"type":"string","minLength":3}}}', true));
        return $app;
    }

    public static function create_path_params_uuid_path_parameter_success_37(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 'ec38df32-ceda-4cfa-9b4a-1aeb94ad551a'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a', $handler, null, null, json_decode('{"path":{"item_id":{"type":"string","format":"uuid"}}}', true));
        return $app;
    }

    public static function create_query_params_42_negative_integer_query_param_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['offset' => -10], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/negative', $handler, null, null, json_decode('{"query":{"offset":{"type":"integer","annotation":"int"}}}', true));
        return $app;
    }

    public static function create_query_params_43_scientific_notation_float_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['threshold' => 0.0015], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/stats', $handler, null, null, json_decode('{"query":{"threshold":{"type":"number","annotation":"float"}}}', true));
        return $app;
    }

    public static function create_query_params_44_string_minlength_validation_success_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['term' => 'foo'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/search', $handler, null, null, json_decode('{"query":{"term":{"type":"string","minLength":3}}}', true));
        return $app;
    }

    public static function create_query_params_45_string_minlength_validation_failure_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'term'], 'msg' => 'String length must be at least 3', 'ctx' => ['min_length' => 3, 'actual_length' => 2]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/search', $handler, null, null, json_decode('{"query":{"term":{"type":"string","minLength":3,"required":true}}}', true));
        return $app;
    }

    public static function create_query_params_46_string_maxlength_validation_failure_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'term'], 'msg' => 'String length must not exceed 10', 'ctx' => ['max_length' => 10, 'actual_length' => 21]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/search', $handler, null, null, json_decode('{"query":{"term":{"type":"string","maxLength":10,"required":true}}}', true));
        return $app;
    }

    public static function create_query_params_47_pattern_validation_email_success_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['email' => 'user@example.com'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/subscribe', $handler, null, null, json_decode('{"query":{"email":{"type":"string","pattern":"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$"}}}', true));
        return $app;
    }

    public static function create_query_params_48_pattern_validation_email_failure_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'email'], 'msg' => 'String does not match pattern', 'ctx' => ['pattern' => '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$', 'value' => 'invalid-email']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/subscribe', $handler, null, null, json_decode('{"query":{"email":{"type":"string","pattern":"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$","required":true}}}', true));
        return $app;
    }

    public static function create_query_params_49_integer_gt_constraint_success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['limit' => 5], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"limit":{"type":"integer","exclusiveMinimum":0}}}', true));
        return $app;
    }

    public static function create_query_params_50_integer_gt_constraint_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'limit'], 'msg' => 'Value must be greater than 0', 'ctx' => ['exclusive_minimum' => 0, 'value' => 0]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"limit":{"type":"integer","exclusiveMinimum":0,"required":true}}}', true));
        return $app;
    }

    public static function create_query_params_51_integer_ge_constraint_boundary_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['offset' => 0], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"offset":{"type":"integer","minimum":0}}}', true));
        return $app;
    }

    public static function create_query_params_52_integer_le_constraint_boundary_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['limit' => 100], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"limit":{"type":"integer","maximum":100}}}', true));
        return $app;
    }

    public static function create_query_params_53_integer_le_constraint_failure_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'limit'], 'msg' => 'Value must not exceed 100', 'ctx' => ['maximum' => 100, 'value' => 101]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"limit":{"type":"integer","maximum":100,"required":true}}}', true));
        return $app;
    }

    public static function create_query_params_54_array_minitems_constraint_success_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['ids' => [1, 2, 3]], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"ids":{"type":"array","items":{"type":"integer"},"minItems":2}}}', true));
        return $app;
    }

    public static function create_query_params_55_array_minitems_constraint_failure_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'ids'], 'msg' => 'Array must contain at least 2 items', 'ctx' => ['min_items' => 2, 'actual_items' => 1]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"ids":{"type":"array","items":{"type":"integer"},"minItems":2,"required":true}}}', true));
        return $app;
    }

    public static function create_query_params_56_array_maxitems_constraint_failure_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'tags'], 'msg' => 'Array must not contain more than 5 items', 'ctx' => ['max_items' => 5, 'actual_items' => 6]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"tags":{"type":"array","items":{"type":"string"},"maxItems":5,"required":true}}}', true));
        return $app;
    }

    public static function create_query_params_57_boolean_empty_string_coercion_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['active' => false], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"active":{"type":"boolean"}}}', true));
        return $app;
    }

    public static function create_query_params_58_format_email_success_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['email' => 'user@example.com'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/subscribe', $handler, null, null, json_decode('{"query":{"email":{"type":"string","format":"email"}}}', true));
        return $app;
    }

    public static function create_query_params_59_format_email_failure_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'email'], 'msg' => 'Invalid email format', 'ctx' => ['format' => 'email', 'value' => 'not-an-email']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/subscribe', $handler, null, null, json_decode('{"query":{"email":{"type":"string","format":"email","required":true}}}', true));
        return $app;
    }

    public static function create_query_params_60_format_ipv4_success_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['ip' => '192.168.1.1'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/network', $handler, null, null, json_decode('{"query":{"ip":{"type":"string","format":"ipv4"}}}', true));
        return $app;
    }

    public static function create_query_params_61_format_ipv4_failure_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'ip'], 'msg' => 'Invalid IPv4 address format', 'ctx' => ['format' => 'ipv4', 'value' => '999.999.999.999']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/network', $handler, null, null, json_decode('{"query":{"ip":{"type":"string","format":"ipv4","required":true}}}', true));
        return $app;
    }

    public static function create_query_params_62_format_ipv6_success_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['ip' => '2001:0db8:85a3:0000:0000:8a2e:0370:7334'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/network/ipv6', $handler, null, null, json_decode('{"query":{"ip":{"type":"string","format":"ipv6"}}}', true));
        return $app;
    }

    public static function create_query_params_63_format_uri_success_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['url' => 'https://example.com/path?query=value'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/redirect', $handler, null, null, json_decode('{"query":{"url":{"type":"string","format":"uri"}}}', true));
        return $app;
    }

    public static function create_query_params_64_format_uri_failure_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'url'], 'msg' => 'Invalid URI format', 'ctx' => ['format' => 'uri', 'value' => 'not a uri']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/redirect', $handler, null, null, json_decode('{"query":{"url":{"type":"string","format":"uri","required":true}}}', true));
        return $app;
    }

    public static function create_query_params_65_format_hostname_success_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['host' => 'api.example.com'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/dns', $handler, null, null, json_decode('{"query":{"host":{"type":"string","format":"hostname"}}}', true));
        return $app;
    }

    public static function create_query_params_66_multipleof_constraint_success_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['quantity' => 15], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"quantity":{"type":"integer","multipleOf":5}}}', true));
        return $app;
    }

    public static function create_query_params_67_multipleof_constraint_failure_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'quantity'], 'msg' => 'Value must be a multiple of 5', 'ctx' => ['multiple_of' => 5, 'value' => 17]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"quantity":{"type":"integer","multipleOf":5,"required":true}}}', true));
        return $app;
    }

    public static function create_query_params_68_array_uniqueitems_success_27(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['ids' => [1, 2, 3, 4]], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"ids":{"type":"array","items":{"type":"integer"},"uniqueItems":true}}}', true));
        return $app;
    }

    public static function create_query_params_69_array_uniqueitems_failure_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['query', 'ids'], 'msg' => 'Array items must be unique', 'ctx' => ['unique_items' => true, 'duplicate_value' => 2, 'duplicate_index' => 2]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items', $handler, null, null, json_decode('{"query":{"ids":{"type":"array","items":{"type":"integer"},"uniqueItems":true,"required":true}}}', true));
        return $app;
    }

    public static function create_query_params_70_array_separator_pipe_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['tags' => ['python', 'rust', 'typescript']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items?tags=python|rust|typescript', $handler, null, null, json_decode('{"query":{"tags":{"type":"array","items":{"type":"string"},"separator":"|"}}}', true));
        return $app;
    }

    public static function create_query_params_71_array_separator_semicolon_30(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['colors' => ['red', 'green', 'blue']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items?colors=red;green;blue', $handler, null, null, json_decode('{"query":{"colors":{"type":"array","items":{"type":"string"},"separator":";"}}}', true));
        return $app;
    }

    public static function create_query_params_72_array_separator_space_31(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['keywords' => ['rust', 'web', 'framework']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/search?keywords=rust%20web%20framework', $handler, null, null, json_decode('{"query":{"keywords":{"type":"array","items":{"type":"string"},"separator":" "}}}', true));
        return $app;
    }

    public static function create_query_params_array_query_parameter_empty_array_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response([], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/list-default', $handler, null, null, json_decode('{"query":{"tags":{"type":"array","annotation":"list[str]","items":{"type":"string"},"optional":true,"default":[]}}}', true));
        return $app;
    }

    public static function create_query_params_array_query_parameter_single_value_33(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['apple'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/list-default', $handler, null, null, json_decode('{"query":{"tags":{"type":"array","annotation":"list[str]","items":{"type":"string"},"optional":true,"default":[]}}}', true));
        return $app;
    }

    public static function create_query_params_boolean_query_parameter_numeric_1_34(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['flag' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/bool', $handler, null, null, json_decode('{"query":{"flag":{"type":"boolean","annotation":"bool"}}}', true));
        return $app;
    }

    public static function create_query_params_boolean_query_parameter_true_35(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['flag' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/bool', $handler, null, null, json_decode('{"query":{"flag":{"type":"boolean","annotation":"bool"}}}', true));
        return $app;
    }

    public static function create_query_params_date_query_parameter_success_36(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['event_date' => '2024-01-15'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/date', $handler, null, null, json_decode('{"query":{"event_date":{"type":"string","annotation":"str","format":"date"}}}', true));
        return $app;
    }

    public static function create_query_params_datetime_query_parameter_success_37(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['timestamp' => '2024-01-15T10:30:00Z'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/datetime', $handler, null, null, json_decode('{"query":{"timestamp":{"type":"string","annotation":"str","format":"date-time"}}}', true));
        return $app;
    }

    public static function create_query_params_enum_query_parameter_invalid_value_38(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'enum', 'loc' => ['query', 'model'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'input' => 'vgg16', 'ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\'']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/enum', $handler, null, null, json_decode('{"query":{"model":{"type":"string","annotation":"str","enum":["alexnet","resnet","lenet"]}}}', true));
        return $app;
    }

    public static function create_query_params_enum_query_parameter_success_39(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['model' => 'alexnet'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/enum', $handler, null, null, json_decode('{"query":{"model":{"type":"string","annotation":"str","enum":["alexnet","resnet","lenet"]}}}', true));
        return $app;
    }

    public static function create_query_params_float_query_param_with_ge_constraint_success_40(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['price' => 0.01], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/float-ge', $handler, null, null, json_decode('{"query":{"price":{"type":"number","annotation":"float","minimum":0.01}}}', true));
        return $app;
    }

    public static function create_query_params_integer_query_param_with_ge_constraint_boundary_41(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['value' => 10], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int-ge', $handler, null, null, json_decode('{"query":{"value":{"type":"integer","annotation":"int","minimum":10}}}', true));
        return $app;
    }

    public static function create_query_params_integer_query_param_with_gt_constraint_valid_42(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['value' => 1], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int-gt', $handler, null, null, json_decode('{"query":{"value":{"type":"integer","annotation":"int","exclusiveMinimum":0}}}', true));
        return $app;
    }

    public static function create_query_params_integer_query_param_with_le_constraint_boundary_43(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['value' => 100], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int-le', $handler, null, null, json_decode('{"query":{"value":{"type":"integer","annotation":"int","maximum":100}}}', true));
        return $app;
    }

    public static function create_query_params_integer_query_param_with_lt_constraint_valid_44(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['value' => 49], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int-lt', $handler, null, null, json_decode('{"query":{"value":{"type":"integer","annotation":"int","exclusiveMaximum":50}}}', true));
        return $app;
    }

    public static function create_query_params_integer_with_default_value_not_provided_45(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('foo bar 10', 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int/default', $handler, null, null, json_decode('{"query":{"query":{"type":"integer","annotation":"int","optional":true,"default":10}}}', true));
        return $app;
    }

    public static function create_query_params_integer_with_default_value_override_46(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('foo bar 50', 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int/default', $handler, null, null, json_decode('{"query":{"query":{"type":"integer","annotation":"int","optional":true,"default":10}}}', true));
        return $app;
    }

    public static function create_query_params_list_of_integers_multiple_values_47(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response([1, 2], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/list', $handler, null, null, json_decode('{"query":{"device_ids":{"type":"array","annotation":"list[int]","items":{"type":"integer"}}}}', true));
        return $app;
    }

    public static function create_query_params_list_of_strings_multiple_values_48(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['q' => ['foo', 'bar']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"query":{"q":{"type":"array","annotation":"list[str]","items":{"type":"string"},"optional":true}}}', true));
        return $app;
    }

    public static function create_query_params_list_query_parameter_required_but_missing_49(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['query', 'device_ids'], 'msg' => 'Field required', 'input' => null]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/list', $handler, null, null, json_decode('{"query":{"device_ids":{"type":"array","annotation":"list[int]","items":{"type":"integer"}}}}', true));
        return $app;
    }

    public static function create_query_params_list_with_default_empty_array_no_values_provided_50(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response([], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/list-default', $handler, null, null, json_decode('{"query":{"tags":{"type":"array","annotation":"list[str]","items":{"type":"string"},"optional":true,"default":[]}}}', true));
        return $app;
    }

    public static function create_query_params_multiple_query_parameters_with_different_types_51(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'john', 'age' => 30, 'active' => true, 'score' => 95.5], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/multi-type', $handler, null, null, json_decode('{"query":{"name":{"type":"string","annotation":"str"},"age":{"type":"integer","annotation":"int"},"active":{"type":"boolean","annotation":"bool"},"score":{"type":"number","annotation":"float"}}}', true));
        return $app;
    }

    public static function create_query_params_optional_integer_query_parameter_missing_52(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('foo bar None', 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int/optional', $handler, null, null, json_decode('{"query":{"query":{"type":"integer","annotation":"int","optional":true}}}', true));
        return $app;
    }

    public static function create_query_params_optional_query_parameter_with_default_value_53(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['limit' => 10], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/optional-default', $handler, null, null, json_decode('{"query":{"limit":{"type":"integer","annotation":"int","optional":true,"default":10}}}', true));
        return $app;
    }

    public static function create_query_params_optional_string_query_parameter_missing_54(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('foo bar None', 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/optional', $handler, null, null, json_decode('{"query":{"query":{"type":"string","annotation":"str","optional":true}}}', true));
        return $app;
    }

    public static function create_query_params_optional_string_query_parameter_provided_55(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('foo bar baz', 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/optional', $handler, null, null, json_decode('{"query":{"query":{"type":"string","annotation":"str","optional":true}}}', true));
        return $app;
    }

    public static function create_query_params_query_parameter_with_url_encoded_space_56(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'hello world'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/basic', $handler, null, null, json_decode('{"query":{"name":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_query_params_query_parameter_with_url_encoded_special_characters_57(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'test&value=123'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/basic', $handler, null, null, json_decode('{"query":{"name":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_query_params_query_parameter_with_special_characters_url_encoding_58(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['email' => 'x@test.com', 'special' => '&@A.ac'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/test', $handler, null, null, json_decode('{"query":{"email":{"type":"string","annotation":"str"},"special":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_query_params_required_integer_query_parameter_float_value_59(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'int_parsing', 'loc' => ['query', 'query'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'input' => 42.5]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int', $handler, null, null, json_decode('{"query":{"query":{"type":"integer","annotation":"int"}}}', true));
        return $app;
    }

    public static function create_query_params_required_integer_query_parameter_invalid_type_60(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'int_parsing', 'loc' => ['query', 'query'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'input' => 'baz']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int', $handler, null, null, json_decode('{"query":{"query":{"type":"integer","annotation":"int"}}}', true));
        return $app;
    }

    public static function create_query_params_required_integer_query_parameter_missing_61(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['query', 'query'], 'msg' => 'Field required', 'input' => null]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int', $handler, null, null, json_decode('{"query":{"query":{"type":"integer","annotation":"int"}}}', true));
        return $app;
    }

    public static function create_query_params_required_integer_query_parameter_success_62(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('foo bar 42', 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/int', $handler, null, null, json_decode('{"query":{"query":{"type":"integer","annotation":"int"}}}', true));
        return $app;
    }

    public static function create_query_params_required_string_query_parameter_missing_63(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['query', 'query'], 'msg' => 'Field required', 'input' => null]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query', $handler, null, null, json_decode('{"query":{"query":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_query_params_required_string_query_parameter_success_64(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('foo bar baz', 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query', $handler, null, null, json_decode('{"query":{"query":{"type":"string","annotation":"str"}}}', true));
        return $app;
    }

    public static function create_query_params_string_query_param_with_max_length_constraint_fail_65(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_long', 'loc' => ['query', 'name'], 'msg' => 'String should have at most 10 characters', 'input' => 'this_is_way_too_long', 'ctx' => ['max_length' => 10]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/str-max-length', $handler, null, null, json_decode('{"query":{"name":{"type":"string","annotation":"str","maxLength":10}}}', true));
        return $app;
    }

    public static function create_query_params_string_query_param_with_min_length_constraint_fail_66(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['query', 'name'], 'msg' => 'String should have at least 3 characters', 'input' => 'ab', 'ctx' => ['min_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/str-min-length', $handler, null, null, json_decode('{"query":{"name":{"type":"string","annotation":"str","minLength":3}}}', true));
        return $app;
    }

    public static function create_query_params_string_query_param_with_regex_pattern_fail_67(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['query', 'code'], 'msg' => 'String should match pattern \'^[0-9]{3,}$\'', 'input' => 'abc123', 'ctx' => ['pattern' => '^[0-9]{3,}$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/pattern', $handler, null, null, json_decode('{"query":{"code":{"type":"string","annotation":"str","pattern":"^[0-9]{3,}$"}}}', true));
        return $app;
    }

    public static function create_query_params_string_validation_with_regex_failure_68(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['query', 'item_query'], 'msg' => 'String should match pattern \'^fixedquery$\'', 'input' => 'nonregexquery', 'ctx' => ['pattern' => '^fixedquery$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"query":{"item_query":{"type":"string","annotation":"str","pattern":"^fixedquery$"}}}', true));
        return $app;
    }

    public static function create_query_params_string_validation_with_regex_success_69(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_query' => 'fixedquery'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"query":{"item_query":{"type":"string","annotation":"str","pattern":"^fixedquery$"}}}', true));
        return $app;
    }

    public static function create_query_params_uuid_query_parameter_invalid_format_70(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'uuid_parsing', 'loc' => ['query', 'item_id'], 'msg' => 'Input should be a valid UUID', 'input' => 'not-a-uuid']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/uuid', $handler, null, null, json_decode('{"query":{"item_id":{"type":"string","annotation":"str","format":"uuid"}}}', true));
        return $app;
    }

    public static function create_query_params_uuid_query_parameter_success_71(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/query/uuid', $handler, null, null, json_decode('{"query":{"item_id":{"type":"string","annotation":"str","format":"uuid"}}}', true));
        return $app;
    }

    public static function create_rate_limit_rate_limit_below_threshold_succeeds_1(): App
    {
        $config = new ServerConfig(
            rateLimit: new RateLimitConfig(perSecond: 5, burst: 5, ipBased: false)
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['status' => 'ok', 'request' => 'under-limit'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/rate-limit/basic', $handler, null, null, null);
        return $app;
    }

    public static function create_rate_limit_rate_limit_exceeded_returns_429_2(): App
    {
        $config = new ServerConfig(
            rateLimit: new RateLimitConfig(perSecond: 1, burst: 1, ipBased: false)
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 429, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/rate-limit/exceeded', $handler, null, null, null);
        return $app;
    }

    public static function create_request_id_request_id_header_is_preserved_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['status' => 'preserved', 'echo' => 'trace-123'], 200, ['x-request-id' => 'trace-123']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/request-id/preserved', $handler, null, null, null);
        return $app;
    }

    public static function create_request_id_request_id_is_generated_when_not_provided_2(): App
    {
        $config = new ServerConfig(
            enableRequestId: true
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['status' => 'generated'], 200, ['x-request-id' => '<<uuid>>']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/request-id/generated', $handler, null, null, null);
        return $app;
    }

    public static function create_request_id_request_id_middleware_can_be_disabled_3(): App
    {
        $config = new ServerConfig(
            enableRequestId: false
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['status' => 'no-request-id'], 200, ['x-request-id' => '<<absent>>']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/request-id/disabled', $handler, null, null, null);
        return $app;
    }

    public static function create_request_timeout_request_completes_before_timeout_1(): App
    {
        $config = new ServerConfig(
            requestTimeout: 2
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['status' => 'ok', 'duration' => 'fast'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/timeouts/fast', $handler, null, null, null);
        return $app;
    }

    public static function create_request_timeout_request_exceeds_timeout_2(): App
    {
        $config = new ServerConfig(
            requestTimeout: 1
        );
        $app = new App($config);
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 408, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/timeouts/slow', $handler, null, null, null);
        return $app;
    }

    public static function create_static_files_static_file_server_returns_text_file_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('Hello from static storage', 200, ['cache-control' => 'public, max-age=60', 'content-type' => 'text/plain']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/public/hello.txt', $handler, null, null, null);
        return $app;
    }

    public static function create_static_files_static_server_returns_index_html_for_directory_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('<!doctype html><h1>Welcome</h1>', 200, ['content-type' => 'text/html']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/app/', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_19_413_payload_too_large_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Payload Too Large', 'message' => 'Request body size exceeds maximum allowed size of 1024 bytes'], 413, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/upload', $handler, json_decode('{"type":"object","properties":{"data":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_status_codes_200_ok_success_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'name' => 'Item 1'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/status-test/200', $handler, null, null, json_decode('{"path":{"code":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_status_codes_201_created_resource_created_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'name' => 'New Item'], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"}},"additionalProperties":false,"required":["name"]}', true), null, null);
        return $app;
    }

    public static function create_status_codes_202_accepted_request_accepted_for_processing_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Task accepted for processing', 'task_id' => 'abc123'], 202, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/tasks/', $handler, json_decode('{"type":"object","properties":{"task":{"type":"string"}},"additionalProperties":false,"required":["task"]}', true), null, null);
        return $app;
    }

    public static function create_status_codes_204_no_content_success_with_no_body_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 204, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'DELETE', '/status-test/204', $handler, null, null, json_decode('{"path":{"code":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_status_codes_206_partial_content_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('binary_data_1024_bytes', 206, ['Content-Range' => 'bytes 0-1023/5000', 'Content-Type' => 'application/pdf', 'Content-Length' => '1024', 'Accept-Ranges' => 'bytes']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/files/document.pdf', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_20_414_uri_too_long_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response((object)[], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/data?skip_template_expansion=true', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_21_431_request_header_fields_too_large_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Request Header Fields Too Large', 'message' => 'Request headers exceed maximum allowed size of 8192 bytes'], 431, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/data', $handler, null, null, json_decode('{"headers":{"X-Large-Header":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_status_codes_22_501_not_implemented_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 405, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'TRACE', '/data', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_23_503_service_unavailable_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Service Unavailable', 'message' => 'The service is temporarily unavailable. Please try again later.'], 503, ['Retry-After' => '60']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/data', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_301_moved_permanently_permanent_redirect_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 301, ['location' => '/new-path']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/old-path', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_302_found_temporary_redirect_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 302, ['location' => '/target-path']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/temp-redirect', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_304_not_modified_cached_content_valid_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 304, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/status-test/304', $handler, null, null, json_decode('{"path":{"code":{"type":"string"}},"headers":{"If-None-Match":{"type":"string","optional":true}}}', true));
        return $app;
    }

    public static function create_status_codes_307_temporary_redirect_method_preserved_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response((object)[], 307, ['location' => '/target-post']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/redirect-post', $handler, json_decode('{"type":"object","properties":{},"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_status_codes_400_bad_request_invalid_request_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Invalid request format'], 400, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"string"}', true), null, null);
        return $app;
    }

    public static function create_status_codes_401_unauthorized_missing_authentication_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Not authenticated'], 401, ['www-authenticate' => 'Bearer']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/users/me', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_403_forbidden_insufficient_permissions_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Not enough permissions'], 403, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/admin/users', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_404_not_found_resource_not_found_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Item not found'], 404, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/status-test/404', $handler, null, null, json_decode('{"path":{"code":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_status_codes_408_request_timeout_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Request timeout'], 408, ['Connection' => 'close']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/slow-endpoint', $handler, json_decode('{"type":"object","properties":{"data":{"type":"string"}},"additionalProperties":false,"required":["data"]}', true), null, null);
        return $app;
    }

    public static function create_status_codes_422_unprocessable_entity_validation_error_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['body', 'name'], 'msg' => 'Field required', 'input' => ['price' => 'not a number']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"price":{"type":"string"},"name":{"type":"string"}},"additionalProperties":false,"required":["price","name"]}', true), null, null);
        return $app;
    }

    public static function create_status_codes_429_too_many_requests_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Rate limit exceeded. Try again in 60 seconds.'], 429, ['Retry-After' => '60', 'X-RateLimit-Limit' => '100', 'X-RateLimit-Remaining' => '0', 'X-RateLimit-Reset' => '1609459200']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/api/resource', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_500_internal_server_error_server_error_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/internal-server-error', 'title' => 'Internal Server Error', 'status' => 500, 'detail' => 'Internal server error'], 500, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/error', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_503_service_unavailable_server_overload_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Service temporarily unavailable'], 503, ['retry-after' => '120']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/health', $handler, null, null, null);
        return $app;
    }

    public static function create_streaming_binary_log_download_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('LOG:\\u0000\\u0001\\u0002\\u0003|TAIL|\\u0007\\n', 200, ['content-type' => 'application/octet-stream']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/stream/logfile', $handler, null, null, null);
        return $app;
    }

    public static function create_streaming_chunked_csv_export_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('id,name,value\\n1,Alice,42\\n2,Bob,7\\n', 200, ['content-type' => 'text/csv']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/stream/csv-report', $handler, null, null, null);
        return $app;
    }

    public static function create_streaming_stream_json_lines_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('{"index":0,"payload":"alpha"}\\n{"index":1,"payload":"beta"}\\n{"index":2,"payload":"gamma"}\\n', 200, ['content-type' => 'application/x-ndjson']);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/stream/json-lines', $handler, null, null, null);
        return $app;
    }

    public static function create_url_encoded_13_array_field_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['tags' => ['python', 'rust', 'typescript']], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/register', $handler, json_decode('{"type":"object","required":["tags"],"properties":{"tags":{"type":"array","items":{"type":"string"},"minItems":1}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_14_nested_object_bracket_notation_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['user' => ['name' => 'John Doe', 'email' => 'john@example.com', 'age' => 30]], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/profile', $handler, json_decode('{"type":"object","required":["user"],"properties":{"user":{"type":"object","required":["name","email"],"properties":{"name":{"type":"string","minLength":1},"email":{"type":"string","format":"email"},"age":{"type":"integer","minimum":0}}}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_15_special_characters_field_names_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['user-name' => 'JohnDoe', 'contact.email' => 'john@example.com'], 201, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/data', $handler, json_decode('{"type":"object","properties":{"user-name":{"type":"string"},"contact.email":{"type":"string","format":"email"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_16_minlength_validation_failure_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['body', 'username'], 'msg' => 'String should have at least 3 characters', 'input' => 'ab', 'ctx' => ['min_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/users', $handler, json_decode('{"type":"object","required":["username"],"properties":{"username":{"type":"string","minLength":3}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_17_pattern_validation_failure_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['body', 'account_id'], 'msg' => 'String should match pattern \'^ACC-[0-9]{6}$\'', 'input' => 'INVALID123', 'ctx' => ['pattern' => '^ACC-[0-9]{6}$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/accounts', $handler, json_decode('{"type":"object","required":["account_id"],"properties":{"account_id":{"type":"string","pattern":"^ACC-[0-9]{6}$"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_18_integer_minimum_validation_failure_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'greater_than_equal', 'loc' => ['body', 'quantity'], 'msg' => 'Input should be greater than or equal to 1', 'input' => 0, 'ctx' => ['ge' => 1]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/products', $handler, json_decode('{"type":"object","required":["quantity"],"properties":{"quantity":{"type":"integer","minimum":1}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_19_array_minitems_validation_failure_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'too_short', 'loc' => ['body', 'tags'], 'msg' => 'List should have at least 2 item after validation', 'input' => ['single'], 'ctx' => ['min_length' => 2]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/tags', $handler, json_decode('{"type":"object","required":["tags"],"properties":{"tags":{"type":"array","items":{"type":"string"},"minItems":2}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_20_format_email_validation_failure_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['body', 'email'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'', 'input' => 'not-an-email', 'ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/subscribe', $handler, json_decode('{"type":"object","required":["email"],"properties":{"email":{"type":"string","format":"email"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_21_integer_type_coercion_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'int_parsing', 'loc' => ['body', 'price'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'input' => 'not-a-number']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/products', $handler, json_decode('{"type":"object","required":["price"],"properties":{"price":{"type":"integer"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_22_additional_properties_strict_failure_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'validation_error', 'loc' => ['body', 'unknown_field'], 'msg' => 'Additional properties are not allowed', 'input' => ['theme' => 'dark', 'unknown_field' => 'value'], 'ctx' => ['additional_properties' => false, 'unexpected_field' => 'unknown_field']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/settings', $handler, json_decode('{"type":"object","required":["theme"],"properties":{"theme":{"type":"string","enum":["light","dark"]}},"additionalProperties":false}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_boolean_field_conversion_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'johndoe', 'subscribe' => true], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/form/', $handler, json_decode('{"type":"object","required":["username"],"properties":{"username":{"type":"string"},"subscribe":{"type":"boolean"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_empty_string_value_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'johndoe', 'description' => ''], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/form/', $handler, json_decode('{"type":"object","required":["username"],"properties":{"username":{"type":"string"},"description":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_multiple_values_for_same_field_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['tags' => ['python', 'fastapi', 'web']], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/form/tags', $handler, json_decode('{"type":"object","required":["tags"],"properties":{"tags":{"type":"array","items":{"type":"string"}}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_numeric_field_type_conversion_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'johndoe', 'age' => 30], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/form/', $handler, json_decode('{"type":"object","required":["username"],"properties":{"username":{"type":"string"},"age":{"type":"integer"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_oauth2_password_grant_flow_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['access_token' => 'johndoe', 'token_type' => 'bearer'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/token', $handler, json_decode('{"type":"object","required":["username","password","grant_type"],"properties":{"username":{"type":"string"},"password":{"type":"string"},"grant_type":{"type":"string"},"scope":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_optional_field_missing_success_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'johndoe', 'email' => null], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/register/', $handler, json_decode('{"type":"object","required":["username","password"],"properties":{"username":{"type":"string"},"password":{"type":"string"},"email":{"type":["string","null"],"format":"email"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_pattern_validation_fail_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['body', 'username'], 'msg' => 'String should match pattern \'^[a-z0-9_]+$\'', 'input' => 'john doe', 'ctx' => ['pattern' => '^[a-z0-9_]+$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/form/validated', $handler, json_decode('{"type":"object","required":["username"],"properties":{"username":{"type":"string","pattern":"^[a-z0-9_]+$"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_required_field_missing_validation_error_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['body', 'username'], 'msg' => 'Field required', 'input' => ['password' => 'secret']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/login/', $handler, json_decode('{"type":"object","required":["username","password"],"properties":{"username":{"type":"string"},"password":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_simple_form_submission_success_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'johndoe'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/login/', $handler, json_decode('{"type":"object","required":["username","password"],"properties":{"username":{"type":"string"},"password":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_special_characters_encoding_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'John Doe', 'description' => 'Test & Development'], 200, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/form/', $handler, json_decode('{"type":"object","required":["name"],"properties":{"name":{"type":"string"},"description":{"type":"string"}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_string_max_length_validation_fail_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_long', 'loc' => ['body', 'username'], 'msg' => 'String should have at most 20 characters', 'input' => 'this_is_a_very_long_username_that_exceeds_limit', 'ctx' => ['max_length' => 20]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/form/validated', $handler, json_decode('{"type":"object","required":["username"],"properties":{"username":{"type":"string","maxLength":20}}}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_string_min_length_validation_fail_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['body', 'username'], 'msg' => 'String should have at least 3 characters', 'input' => 'ab', 'ctx' => ['min_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/form/validated', $handler, json_decode('{"type":"object","required":["username"],"properties":{"username":{"type":"string","minLength":3}}}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_09_multiple_validation_errors_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '3 validation errors in request', 'errors' => [['type' => 'greater_than_equal', 'loc' => ['body', 'age'], 'msg' => 'Input should be greater than or equal to 18', 'input' => 15, 'ctx' => ['ge' => 18]], ['type' => 'string_pattern_mismatch', 'loc' => ['body', 'email'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'', 'input' => 'invalid-email', 'ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$']], ['type' => 'string_too_short', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'input' => 'ab', 'ctx' => ['min_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/users', $handler, json_decode('{"type":"object","required":["name","email","age"],"properties":{"name":{"type":"string","minLength":3},"email":{"type":"string","format":"email"},"age":{"type":"integer","minimum":18}}}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_10_nested_error_path_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['body', 'profile', 'contact', 'email'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'', 'input' => 'invalid', 'ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/profiles', $handler, json_decode('{"type":"object","required":["profile"],"properties":{"profile":{"type":"object","required":["contact"],"properties":{"contact":{"type":"object","required":["email"],"properties":{"email":{"type":"string","format":"email"}}}}}}}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_array_item_validation_error_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'type_error', 'loc' => ['body', 'tags', '2'], 'msg' => 'Input should be a valid unknown', 'input' => 123]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"type":"array","items":{"type":"string"}}},"additionalProperties":false,"required":["name","price","tags"]}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_array_max_items_constraint_violation_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'too_long', 'loc' => ['body', 'tags'], 'msg' => 'List should have at most 10 items after validation', 'input' => ['tag1', 'tag2', 'tag3', 'tag4', 'tag5', 'tag6', 'tag7', 'tag8', 'tag9', 'tag10', 'tag11'], 'ctx' => ['max_length' => 10]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"type":"array","items":{"type":"string"},"maxItems":10}},"additionalProperties":false,"required":["name","price","tags"]}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_array_min_items_constraint_violation_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'too_short', 'loc' => ['body', 'tags'], 'msg' => 'List should have at least 1 item after validation', 'input' => [], 'ctx' => ['min_length' => 1]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"type":"array","items":{},"minItems":1}},"additionalProperties":false,"required":["name","price","tags"]}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_body_field_type_error_string_for_float_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'float_parsing', 'loc' => ['body', 'price'], 'msg' => 'Input should be a valid number, unable to parse string as a number', 'input' => 'not_a_float']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"}},"additionalProperties":false,"required":["name","price"]}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_header_validation_error_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['headers', 'x-token'], 'msg' => 'Field required', 'input' => null]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/?q=test', $handler, null, null, json_decode('{"query":{"q":{"type":"string"}},"headers":{"x-token":{"type":"string","required":true}}}', true));
        return $app;
    }

    public static function create_validation_errors_invalid_uuid_format_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'uuid_parsing', 'loc' => ['path', 'item_id'], 'msg' => 'Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0', 'input' => 'not-a-uuid']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/not-a-uuid', $handler, null, null, json_decode('{"path":{"item_id":{"type":"string","format":"uuid"}}}', true));
        return $app;
    }

    public static function create_validation_errors_invalid_boolean_value_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'bool_parsing', 'loc' => ['query', 'is_active'], 'msg' => 'Input should be a valid boolean, unable to interpret input', 'input' => 'maybe']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/?q=test&is_active=maybe', $handler, null, null, json_decode('{"query":{"q":{"type":"string"},"is_active":{"type":"boolean"}}}', true));
        return $app;
    }

    public static function create_validation_errors_invalid_datetime_format_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'datetime_parsing', 'loc' => ['body', 'created_at'], 'msg' => 'Input should be a valid datetime', 'input' => 'not-a-datetime']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"created_at":{"type":"string","format":"date-time"}},"additionalProperties":false,"required":["name","price","created_at"]}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_invalid_enum_value_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'enum', 'loc' => ['path', 'model_name'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'input' => 'invalid_model', 'ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\'']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/models/invalid_model', $handler, null, null, json_decode('{"path":{"model_name":{"type":"string","enum":["alexnet","resnet","lenet"]}}}', true));
        return $app;
    }

    public static function create_validation_errors_malformed_json_body_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Invalid request format'], 400, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"string"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_missing_required_body_field_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['body', 'price'], 'msg' => 'Field required', 'input' => ['name' => 'Item']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"string"}},"additionalProperties":false,"required":["name","price"]}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_missing_required_query_parameter_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'missing', 'loc' => ['query', 'q'], 'msg' => 'Field required', 'input' => null]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/', $handler, null, null, json_decode('{"query":{"q":{"type":"string","required":true}}}', true));
        return $app;
    }

    public static function create_validation_errors_multiple_validation_errors_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '3 validation errors in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'input' => 'X', 'ctx' => ['min_length' => 3]], ['type' => 'greater_than', 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than 0', 'input' => -10, 'ctx' => ['gt' => 0]], ['type' => 'int_parsing', 'loc' => ['body', 'quantity'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'input' => 'not_a_number']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string","minLength":3},"price":{"type":"integer","exclusiveMinimum":0},"quantity":{"type":"integer"}},"additionalProperties":false,"required":["name","price","quantity"]}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_nested_object_validation_error_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '3 validation errors in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['body', 'seller', 'address', 'city'], 'msg' => 'String should have at least 3 characters', 'input' => 'SF', 'ctx' => ['min_length' => 3]], ['type' => 'string_too_short', 'loc' => ['body', 'seller', 'address', 'zip_code'], 'msg' => 'String should have at least 5 characters', 'input' => '123', 'ctx' => ['min_length' => 5]], ['type' => 'string_too_short', 'loc' => ['body', 'seller', 'name'], 'msg' => 'String should have at least 3 characters', 'input' => 'Jo', 'ctx' => ['min_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'POST', '/items/', $handler, json_decode('{"type":"object","properties":{"name":{"type":"string"},"price":{"type":"number"},"seller":{"type":"object","properties":{"name":{"type":"string","minLength":3},"address":{"type":"object","properties":{"city":{"type":"string","minLength":3},"zip_code":{"type":"string","minLength":5}},"additionalProperties":false,"required":["city","zip_code"]}},"additionalProperties":false,"required":["name","address"]}},"additionalProperties":false,"required":["name","price","seller"]}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_numeric_constraint_violation_gt_greater_than_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'greater_than', 'loc' => ['query', 'price'], 'msg' => 'Input should be greater than 0', 'input' => '0', 'ctx' => ['gt' => 0]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/?q=test&price=0', $handler, null, null, json_decode('{"query":{"q":{"type":"string"},"price":{"type":"number","exclusiveMinimum":0}}}', true));
        return $app;
    }

    public static function create_validation_errors_numeric_constraint_violation_le_less_than_or_equal_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'less_than_equal', 'loc' => ['query', 'limit'], 'msg' => 'Input should be less than or equal to 100', 'input' => '101', 'ctx' => ['le' => 100]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/?q=test&limit=101', $handler, null, null, json_decode('{"query":{"q":{"type":"string"},"limit":{"type":"integer","maximum":100}}}', true));
        return $app;
    }

    public static function create_validation_errors_query_param_type_error_string_provided_for_int_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'int_parsing', 'loc' => ['query', 'skip'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'input' => 'not_a_number']]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/?q=test&skip=not_a_number', $handler, null, null, json_decode('{"query":{"q":{"type":"string"},"skip":{"type":"integer"}}}', true));
        return $app;
    }

    public static function create_validation_errors_string_max_length_constraint_violation_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_long', 'loc' => ['query', 'q'], 'msg' => 'String should have at most 50 characters', 'input' => 'this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', 'ctx' => ['max_length' => 50]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', $handler, null, null, json_decode('{"query":{"q":{"type":"string","maxLength":50}}}', true));
        return $app;
    }

    public static function create_validation_errors_string_min_length_constraint_violation_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_too_short', 'loc' => ['query', 'q'], 'msg' => 'String should have at least 3 characters', 'input' => 'ab', 'ctx' => ['min_length' => 3]]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/?q=ab', $handler, null, null, json_decode('{"query":{"q":{"type":"string","minLength":3}}}', true));
        return $app;
    }

    public static function create_validation_errors_string_regex_pattern_mismatch_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['type' => 'https://spikard.dev/errors/validation-error', 'title' => 'Request Validation Failed', 'status' => 422, 'detail' => '1 validation error in request', 'errors' => [['type' => 'string_pattern_mismatch', 'loc' => ['query', 'q'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_-]+$\'', 'input' => 'invalid!', 'ctx' => ['pattern' => '^[a-zA-Z0-9_-]+$']]]], 422, []);
            }
            public function __invoke(Request $request): Response {
                return $this->handle($request);
            }
        };
        $app = \register_route_with_schemas($app, 'GET', '/items/?q=invalid!', $handler, null, null, json_decode('{"query":{"q":{"type":"string","pattern":"^[a-zA-Z0-9_-]+$"}}}', true));
        return $app;
    }

}

final class SseProducer_1 implements SseEventProducerInterface
{
    /** @return \Generator<int, string, mixed, void> */
    public function __invoke(): \Generator
    {
        $events = [['level' => 'critical', 'message' => 'Database connection pool exhausted', 'source' => 'database-service', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'system_alert']];
        foreach ($events as $event) {
            yield json_encode(['data' => $event]);
        }
    }
}

final class SseProducer_2 implements SseEventProducerInterface
{
    /** @return \Generator<int, string, mixed, void> */
    public function __invoke(): \Generator
    {
        $events = [[['message' => 'example_message', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'example_type'], ['message' => 'example_message', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'example_type']]];
        foreach ($events as $event) {
            yield json_encode(['data' => $event]);
        }
    }
}

final class SseProducer_3 implements SseEventProducerInterface
{
    /** @return \Generator<int, string, mixed, void> */
    public function __invoke(): \Generator
    {
        $events = [['body' => 'You have received a new direct message', 'priority' => 'high', 'timestamp' => '2024-01-15T10:30:00Z', 'title' => 'New message from John', 'type' => 'user_notification', 'userId' => 'user_12345']];
        foreach ($events as $event) {
            yield json_encode(['data' => $event]);
        }
    }
}

final class SseProducer_4 implements SseEventProducerInterface
{
    /** @return \Generator<int, string, mixed, void> */
    public function __invoke(): \Generator
    {
        $events = [['message' => 'All systems operational', 'metadata' => ['region' => 'us-east-1', 'uptime' => 99.99], 'service' => 'payment-gateway', 'status' => 'operational', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'status_update']];
        foreach ($events as $event) {
            yield json_encode(['data' => $event]);
        }
    }
}

final class WebSocketHandler_1 implements WebSocketHandlerInterface
{
    private array $messages = [];
    private int $messageIndex = 0;

    public function __construct()
    {
        $this->messages = [['level' => 'example_level', 'message' => 'example_message', 'source' => 'example_source', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'system_alert']];
    }

    public function onConnect(): void
    {
        // Connection established
    }

    public function onMessage(string $message): void
    {
        // Handle incoming message
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        // Connection closed
    }

    public function getNextMessage(): ?array
    {
        if ($this->messageIndex < count($this->messages)) {
            return $this->messages[$this->messageIndex++];
        }
        return null;
    }
}

final class WebSocketHandler_2 implements WebSocketHandlerInterface
{
    private array $messages = [];
    private int $messageIndex = 0;

    public function __construct()
    {
        $this->messages = [['text' => 'Hello, everyone!', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'message', 'user' => 'alice']];
    }

    public function onConnect(): void
    {
        // Connection established
    }

    public function onMessage(string $message): void
    {
        // Handle incoming message
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        // Connection closed
    }

    public function getNextMessage(): ?array
    {
        if ($this->messageIndex < count($this->messages)) {
            return $this->messages[$this->messageIndex++];
        }
        return null;
    }
}

final class WebSocketHandler_3 implements WebSocketHandlerInterface
{
    private array $messages = [];
    private int $messageIndex = 0;

    public function __construct()
    {
        $this->messages = [['messageId' => 'ack-123', 'status' => 'delivered', 'timestamp' => '2024-01-15T10:31:00Z', 'type' => 'chatAck']];
    }

    public function onConnect(): void
    {
        // Connection established
    }

    public function onMessage(string $message): void
    {
        // Handle incoming message
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        // Connection closed
    }

    public function getNextMessage(): ?array
    {
        if ($this->messageIndex < count($this->messages)) {
            return $this->messages[$this->messageIndex++];
        }
        return null;
    }
}

final class WebSocketHandler_4 implements WebSocketHandlerInterface
{
    private array $messages = [];
    private int $messageIndex = 0;

    public function __construct()
    {
        $this->messages = [['timestamp' => '2024-01-15T10:35:00Z', 'type' => 'userLeft', 'user' => 'charlie']];
    }

    public function onConnect(): void
    {
        // Connection established
    }

    public function onMessage(string $message): void
    {
        // Handle incoming message
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        // Connection closed
    }

    public function getNextMessage(): ?array
    {
        if ($this->messageIndex < count($this->messages)) {
            return $this->messages[$this->messageIndex++];
        }
        return null;
    }
}

final class WebSocketHandler_5 implements WebSocketHandlerInterface
{
    private array $messages = [];
    private int $messageIndex = 0;

    public function __construct()
    {
        $this->messages = [['timestamp' => '2024-01-15T10:29:55Z', 'type' => 'userJoined', 'user' => 'bob']];
    }

    public function onConnect(): void
    {
        // Connection established
    }

    public function onMessage(string $message): void
    {
        // Handle incoming message
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        // Connection closed
    }

    public function getNextMessage(): ?array
    {
        if ($this->messageIndex < count($this->messages)) {
            return $this->messages[$this->messageIndex++];
        }
        return null;
    }
}

final class WebSocketHandler_6 implements WebSocketHandlerInterface
{
    private array $messages = [];
    private int $messageIndex = 0;

    public function __construct()
    {
        $this->messages = [['body' => 'example_body', 'priority' => 'example_priority', 'timestamp' => '2024-01-15T10:30:00Z', 'title' => 'example_title', 'type' => 'user_notification', 'userId' => 'example_userId']];
    }

    public function onConnect(): void
    {
        // Connection established
    }

    public function onMessage(string $message): void
    {
        // Handle incoming message
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        // Connection closed
    }

    public function getNextMessage(): ?array
    {
        if ($this->messageIndex < count($this->messages)) {
            return $this->messages[$this->messageIndex++];
        }
        return null;
    }
}

final class WebSocketHandler_7 implements WebSocketHandlerInterface
{
    private array $messages = [];
    private int $messageIndex = 0;

    public function __construct()
    {
        $this->messages = [['message' => 'example_message', 'metadata' => (object)[], 'service' => 'example_service', 'status' => 'example_status', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'status_update']];
    }

    public function onConnect(): void
    {
        // Connection established
    }

    public function onMessage(string $message): void
    {
        // Handle incoming message
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        // Connection closed
    }

    public function getNextMessage(): ?array
    {
        if ($this->messageIndex < count($this->messages)) {
            return $this->messages[$this->messageIndex++];
        }
        return null;
    }
}

