<?php

declare(strict_types=1);

namespace E2E\Php;

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
    public static function create_sse_systemalert_1(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_1());
        return $app;
    }

    public static function create_sse_notificationbatch_2(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_2());
        return $app;
    }

    public static function create_sse_usernotification_3(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_3());
        return $app;
    }

    public static function create_sse_statusupdate_4(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_4());
        return $app;
    }

    public static function create_websocket_systemalert_1(): App
    {
        $app = new App();
        $app = $app->addWebSocket('systemAlert', new WebSocketHandler_1());
        return $app;
    }

    public static function create_websocket_chatmessage_2(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chat', new WebSocketHandler_2());
        return $app;
    }

    public static function create_websocket_chatack_3(): App
    {
        $app = new App();
        $app = $app->addWebSocket('chatAck', new WebSocketHandler_3());
        return $app;
    }

    public static function create_websocket_userleft_4(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chat', new WebSocketHandler_4());
        return $app;
    }

    public static function create_websocket_userjoined_5(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chat', new WebSocketHandler_5());
        return $app;
    }

    public static function create_websocket_usernotification_6(): App
    {
        $app = new App();
        $app = $app->addWebSocket('userNotification', new WebSocketHandler_6());
        return $app;
    }

    public static function create_websocket_statusupdate_7(): App
    {
        $app = new App();
        $app = $app->addWebSocket('statusUpdate', new WebSocketHandler_7());
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
                return new Response(['detail' => 'The provided API key is not valid', 'status' => 401, 'title' => 'Invalid API key', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'Expected \'X-API-Key\' header or \'api_key\' query parameter with valid API key', 'status' => 401, 'title' => 'Missing API key', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{}', true));
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
                return new Response(['data' => 'sensitive information', 'message' => 'Access granted'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"description":"API key for authentication","required":true,"type":"string"}}}', true));
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
                return new Response(['data' => 'sensitive information', 'message' => 'Access granted'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data?api_key=sk_test_123456', $handler, null, null, null);
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
                return new Response(['data' => 'sensitive information', 'message' => 'Access granted'], 200, ['X-API-Key-Deprecated' => 'true']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"description":"API key for authentication","required":true,"type":"string"}}}', true));
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
                return new Response(['data' => 'sensitive information', 'message' => 'Access granted'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Token":{"description":"API token for authentication","required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'Authorization header must use Bearer scheme: \'Bearer <token>\'', 'status' => 401, 'title' => 'Invalid Authorization header format', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"description":"JWT token in Bearer format","required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'Token has expired', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, null, null, json_decode('{"headers":{"Authorization":{"required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'Token audience is invalid', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, null, null, json_decode('{"headers":{"Authorization":{"required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'Token signature is invalid', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, null, null, json_decode('{"headers":{"Authorization":{"required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'Expected \'Authorization: Bearer <token>\'', 'status' => 401, 'title' => 'Missing or invalid Authorization header', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, null, null, json_decode('{}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, null, null, json_decode('{"headers":{"Authorization":{"description":"JWT token in Bearer format","required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'Token issuer is invalid, expected \'https://auth.example.com\'', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"description":"JWT token in Bearer format","required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'Malformed JWT token: expected 3 parts separated by dots, found 2', 'status' => 401, 'title' => 'Malformed JWT token', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"description":"JWT token in Bearer format","required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'Required claims \'role\' and \'permissions\' missing from JWT', 'status' => 403, 'title' => 'Forbidden', 'type' => 'https://spikard.dev/errors/forbidden'], 403, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/admin', $handler, null, null, json_decode('{"headers":{"Authorization":{"description":"JWT token in Bearer format","required":true,"type":"string"}}}', true));
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
                return new Response(['detail' => 'JWT not valid yet, not before claim is in the future', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], 401, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"description":"JWT token in Bearer format","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"description":"JWT token in Bearer format","required":true,"type":"string"}}}', true));
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
                return new Response(['auth_method' => 'jwt', 'message' => 'Access granted', 'user_id' => 'user123'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{"headers":{"Authorization":{"description":"JWT token in Bearer format","required":false,"type":"string"},"X-API-Key":{"description":"API key for authentication","required":false,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/background/events', $handler, json_decode('{"additionalProperties":false,"properties":{"event":{"type":"string"}},"required":["event"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/background/events', $handler, json_decode('{"additionalProperties":false,"properties":{"event":{"type":"string"}},"required":["event"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/body-limit/over', $handler, json_decode('{"additionalProperties":false,"properties":{"note":{"type":"string"}},"required":["note"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/body-limit/under', $handler, json_decode('{"additionalProperties":false,"properties":{"note":{"type":"string"}},"required":["note"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/compression/gzip', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/compression/skip', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_13_json_with_charset_utf16_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Unsupported charset \'utf-16\' for JSON. Only UTF-8 is supported.', 'status' => 415, 'title' => 'Unsupported Charset', 'type' => 'https://spikard.dev/errors/unsupported-charset'], 415, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"properties":{"value":{"type":"string"}},"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, null, null, json_decode('{"files":{"document":{"required":true}}}', true));
        return $app;
    }

    public static function create_content_types_16_text_plain_not_accepted_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Unsupported media type', 'status' => 415, 'title' => 'Unsupported Media Type', 'type' => 'https://spikard.dev/errors/unsupported-media-type'], 415, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"properties":{"data":{"type":"string"}},"required":["data"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/api/v1/resource', $handler, json_decode('{"properties":{"data":{"type":"string"}},"required":["data"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"properties":{"value":{"type":"string"}},"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"properties":{"value":{"type":"string"}},"type":"object"}', true), null, json_decode('{"headers":{"Content-Length":{"optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_content_types_415_unsupported_media_type_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Unsupported media type'], 415, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"type":"string"}', true), null, null);
        return $app;
    }

    public static function create_content_types_binary_response_application_octet_stream_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('binary_data_placeholder', 200, ['content-disposition' => 'attachment; filename=file.bin', 'content-type' => 'application/octet-stream']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/download/file.bin', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/export/data.csv', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/accept-test/1', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/html', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/images/photo.jpg', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/json', $handler, null, null, null);
        return $app;
    }

    public static function create_content_types_json_with_utf_8_charset_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['emoji' => '☕', 'name' => 'Café'], 200, ['content-type' => 'application/json; charset=utf-8']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/unicode', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/download/document.pdf', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/images/logo.png', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/text', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/xml', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/secure', $handler, null, null, json_decode('{"cookies":{"session_id":{"required":true,"samesite":"Strict","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/data', $handler, null, null, json_decode('{"cookies":{"tracking":{"required":true,"samesite":"Lax","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/secure', $handler, null, null, json_decode('{"cookies":{"auth_token":{"required":true,"secure":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/secure', $handler, null, null, json_decode('{"cookies":{"session":{"httponly":true,"required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cookies_apikey_cookie_authentication_missing_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['cookie', 'key'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me/auth', $handler, null, null, json_decode('{"cookies":{"key":{"required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, json_decode('{"cookies":{"key":{"optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cookies_cookie_regex_pattern_validation_fail_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[A-Z0-9]{8}$'], 'input' => 'invalid-format', 'loc' => ['cookie', 'tracking_id'], 'msg' => 'String should match pattern \'^[A-Z0-9]{8}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/cookies/pattern', $handler, null, null, json_decode('{"cookies":{"tracking_id":{"pattern":"^[A-Z0-9]{8}$","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/cookies/pattern', $handler, null, null, json_decode('{"cookies":{"tracking_id":{"optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cookies_cookie_validation_max_length_constraint_fail_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 20], 'input' => 'this_cookie_value_is_way_too_long', 'loc' => ['cookie', 'session_id'], 'msg' => 'String should have at most 20 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/cookies/validated', $handler, null, null, json_decode('{"cookies":{"session_id":{"maxLength":20,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/cookies/min-length', $handler, null, null, json_decode('{"cookies":{"token":{"optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cookies_cookie_validation_min_length_failure_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['cookie', 'tracking_id'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"cookies":{"tracking_id":{"minLength":3,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cookies_multiple_cookies_success_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['fatebook_tracker' => 'tracker456', 'googall_tracker' => 'ga789', 'session_id' => 'session123'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"cookies":{"fatebook_tracker":{"optional":true,"type":"string"},"googall_tracker":{"optional":true,"type":"string"},"session_id":{"optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, json_decode('{"cookies":{"key":{"optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"cookies":{"ads_id":{"optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"cookies":{"ads_id":{"optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cookies_required_cookie_missing_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['cookie', 'session_id'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/cookies', $handler, null, null, json_decode('{"cookies":{"fatebook_tracker":{"optional":true,"type":"string"},"session_id":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/delete', $handler, null, null, json_decode('{"cookies":{"session":{"optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/multiple', $handler, json_decode('{"additionalProperties":false,"properties":{"session":{"type":"string"},"user":{"type":"string"}},"required":["user","session"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/session', $handler, json_decode('{"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/samesite-lax', $handler, json_decode('{"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/samesite-none', $handler, json_decode('{"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/samesite-strict', $handler, json_decode('{"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/cookie/set', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/set-with-domain', $handler, json_decode('{"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/set-with-path', $handler, json_decode('{"additionalProperties":false,"properties":{"value":{"type":"string"}},"required":["value"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/cookie/', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/data', $handler, null, null, json_decode('{"headers":{"Access-Control-Request-Headers":{"optional":true,"type":"string"},"Access-Control-Request-Method":{"optional":true,"type":"string"},"Origin":{"optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/data', $handler, null, null, json_decode('{"headers":{"Access-Control-Request-Headers":{"optional":true,"type":"string"},"Access-Control-Request-Method":{"optional":true,"type":"string"},"Origin":{"optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cors_08_cors_max_age_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 204, ['Access-Control-Allow-Methods' => 'POST', 'Access-Control-Max-Age' => '3600', 'Access-Control-Allow-Origin' => 'https://example.com', 'Access-Control-Allow-Headers' => 'Content-Type']);
            }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/data', $handler, null, null, json_decode('{"headers":{"Access-Control-Request-Headers":{"optional":true,"type":"string"},"Access-Control-Request-Method":{"optional":true,"type":"string"},"Origin":{"optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cors_09_cors_expose_headers_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, ['Access-Control-Allow-Origin' => 'https://example.com', 'X-Total-Count' => '42', 'Access-Control-Expose-Headers' => 'X-Total-Count, X-Request-Id', 'X-Request-Id' => 'abc123']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{"headers":{"Origin":{"optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{"headers":{"Origin":{"optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cors_cors_private_network_access_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 204, ['Access-Control-Allow-Private-Network' => 'true', 'Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://public.example.com', 'Access-Control-Allow-Methods' => 'GET, POST']);
            }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/local-resource', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_vary_header_for_proper_caching_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['data' => 'cacheable resource'], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://app.example.com', 'Cache-Control' => 'public, max-age=3600']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/cached-resource', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_multiple_allowed_origins_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['data' => 'resource data'], 200, ['Access-Control-Allow-Origin' => 'https://admin.example.com', 'Vary' => 'Origin']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_preflight_for_delete_method_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 204, ['Vary' => 'Origin', 'Access-Control-Allow-Methods' => 'GET, POST, PUT, PATCH, DELETE', 'Access-Control-Allow-Origin' => 'https://app.example.com', 'Access-Control-Max-Age' => '3600']);
            }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/resource/456', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_preflight_for_put_method_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 204, ['Access-Control-Allow-Methods' => 'GET, POST, PUT, PATCH, DELETE', 'Access-Control-Allow-Headers' => 'Content-Type, X-Custom-Header', 'Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://app.example.com', 'Access-Control-Max-Age' => '3600']);
            }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/resource/123', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/items/', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_regex_pattern_matching_for_origins_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['data' => 'resource data'], 200, ['Access-Control-Allow-Origin' => 'https://subdomain.example.com', 'Vary' => 'Origin']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"headers":{"Origin":{"optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_cors_cors_safelisted_headers_without_preflight_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Success'], 200, ['Access-Control-Allow-Origin' => 'https://app.example.com', 'Vary' => 'Origin']);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/form', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/public/data', $handler, null, null, null);
        return $app;
    }

    public static function create_cors_cors_with_credentials_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['username' => 'john'], 200, ['Vary' => 'Origin', 'Access-Control-Allow-Origin' => 'https://app.example.com', 'Access-Control-Allow-Credentials' => 'true']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/user/profile', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, null);
        return $app;
    }

    public static function create_di_async_factory_dependency_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['max_size' => 10, 'pool_status' => 'connected'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/db-status', $handler, null, null, null);
        return $app;
    }

    public static function create_di_circular_dependency_detection_error_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Circular dependency detected', 'errors' => [['cycle' => ['service_a', 'service_b', 'service_a'], 'msg' => 'Circular dependency detected in dependency graph', 'type' => 'circular_dependency']], 'status' => 500, 'title' => 'Dependency Resolution Failed', 'type' => 'https://spikard.dev/errors/dependency-error'], 500, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/circular', $handler, null, null, null);
        return $app;
    }

    public static function create_di_dependency_injection_in_lifecycle_hooks_success_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['authenticated' => true, 'logged' => true], 200, ['X-Auth-Mode' => 'strict', 'X-Log-Level' => 'debug']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/hook-di-test', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/timestamp', $handler, null, null, null);
        return $app;
    }

    public static function create_di_missing_dependency_error_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Required dependency not found', 'errors' => [['dependency_key' => 'non_existent_service', 'msg' => 'Dependency \'non_existent_service\' is not registered', 'type' => 'missing_dependency']], 'status' => 500, 'title' => 'Dependency Resolution Failed', 'type' => 'https://spikard.dev/errors/dependency-error'], 500, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/missing-dep', $handler, null, null, null);
        return $app;
    }

    public static function create_di_mixed_singleton_and_per_request_caching_success_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['app_name' => 'MyApp', 'context_id' => '<<uuid>>', 'pool_id' => '<<uuid>>'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/mixed-caching', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/multi-cleanup-test', $handler, null, null, null);
        return $app;
    }

    public static function create_di_nested_dependencies_3_levels_success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['auth_enabled' => true, 'has_cache' => true, 'has_db' => true], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/auth-status', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/node-destructure', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/request-id', $handler, null, null, null);
        return $app;
    }

    public static function create_di_python_parameter_name_based_injection_success_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['cache_status' => 'ready', 'db_status' => 'connected'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/python-name-inject', $handler, null, null, null);
        return $app;
    }

    public static function create_di_python_type_annotation_based_injection_success_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['cache_type' => 'Redis', 'pool_type' => 'PostgreSQL'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/python-type-inject', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/cleanup-test', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/override-test', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/ruby-kwargs', $handler, null, null, null);
        return $app;
    }

    public static function create_di_singleton_dependency_caching_success_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['count' => 1, 'counter_id' => '<<uuid>>'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/app-counter', $handler, null, null, null);
        return $app;
    }

    public static function create_di_type_mismatch_in_dependency_resolution_error_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Dependency type mismatch', 'errors' => [['actual_type' => 'string', 'dependency_key' => 'config', 'expected_type' => 'object', 'msg' => 'Dependency \'config\' type mismatch: expected object, got string', 'type' => 'type_mismatch']], 'status' => 500, 'title' => 'Dependency Resolution Failed', 'type' => 'https://spikard.dev/errors/dependency-error'], 500, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/type-mismatch', $handler, null, null, null);
        return $app;
    }

    public static function create_di_value_dependency_injection_success_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['app_name' => 'SpikardApp', 'max_connections' => 100, 'version' => '1.0.0'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/config', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/search', $handler, null, null, json_decode('{"query":{"term":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/search?term=hi%20there', $handler, null, null, json_decode('{"query":{"term":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items?filter=', $handler, null, null, json_decode('{"query":{"filter":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"id":{"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/calculate', $handler, json_decode('{"properties":{"value":{"type":"number"}},"required":["value"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"properties":{"offset":{"type":"number"}},"required":["offset"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_17_extremely_long_string_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 10001, 'max_length' => 10000], 'loc' => ['body', 'content'], 'msg' => 'String length must not exceed 10000', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/text', $handler, json_decode('{"properties":{"content":{"maxLength":10000,"type":"string"}},"required":["content"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode('{"properties":{"name":{"minLength":1,"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/messages', $handler, json_decode('{"properties":{"text":{"maxLength":100,"minLength":1,"type":"string"}},"required":["text"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_20_null_byte_in_string_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['value' => 'file\\u0000.txt'], 'loc' => ['body', 'filename'], 'msg' => 'String contains null byte character', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/files', $handler, json_decode('{"properties":{"filename":{"pattern":"^[^\\\\x00]+$","type":"string"}},"required":["filename"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/calculate', $handler, json_decode('{"properties":{"value":{"minimum":0,"type":"number"}},"required":["value"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/data', $handler, null, null, json_decode('{"query":{"value":{"annotation":"int","type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/items', $handler, json_decode('{"properties":{"items":{"items":{"type":"string"},"type":"array"}},"required":["items"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_deeply_nested_structure_10_levels_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['max_depth' => 10, 'message' => 'Processed deeply nested structure', 'value_found' => 'deep'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/nested/', $handler, json_decode('{"additionalProperties":false,"properties":{"level1":{"additionalProperties":false,"properties":{"level2":{"additionalProperties":false,"properties":{"level3":{"additionalProperties":false,"properties":{"level4":{"additionalProperties":false,"properties":{"level5":{"additionalProperties":false,"properties":{"level6":{"additionalProperties":false,"properties":{"level7":{"additionalProperties":false,"properties":{"level8":{"additionalProperties":false,"properties":{"level9":{"additionalProperties":false,"properties":{"level10":{"additionalProperties":false,"properties":{"depth":{"type":"integer"},"value":{"type":"string"}},"required":["value","depth"],"type":"object"}},"required":["level10"],"type":"object"}},"required":["level9"],"type":"object"}},"required":["level8"],"type":"object"}},"required":["level7"],"type":"object"}},"required":["level6"],"type":"object"}},"required":["level5"],"type":"object"}},"required":["level4"],"type":"object"}},"required":["level3"],"type":"object"}},"required":["level2"],"type":"object"}},"required":["level1"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_empty_and_null_value_handling_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['empty_array_length' => 0, 'empty_object_keys' => 0, 'empty_string_length' => 0, 'explicit_null_is_null' => true, 'false_is_false' => true, 'zero_is_falsy' => true], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/nulls/', $handler, json_decode('{"additionalProperties":false,"properties":{"empty_array":{"items":{},"type":"array"},"empty_object":{"additionalProperties":false,"properties":{},"type":"object"},"empty_string":{"type":"string"},"explicit_null":{"type":"null"},"false_boolean":{"type":"boolean"},"zero_number":{"type":"integer"}},"required":["explicit_null","empty_string","empty_array","empty_object","zero_number","false_boolean"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_float_precision_and_rounding_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['precise_value' => 3.141592653589793, 'sum' => 0.30000000000000004, 'very_large' => 1.7976931348623157e308, 'very_small' => 1e-10], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/calculations/', $handler, json_decode('{"additionalProperties":false,"properties":{"expected_sum":{"type":"number"},"precise_value":{"type":"number"},"value1":{"type":"number"},"value2":{"type":"number"},"very_large":{"type":"number"},"very_small":{"type":"number"}},"required":["value1","value2","expected_sum","precise_value","very_small","very_large"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_large_integer_boundary_values_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['large_int' => 9223372036854775807, 'max_safe_int' => 9007199254740991, 'negative_large' => -9223372036854775808], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/numbers/', $handler, json_decode('{"additionalProperties":false,"properties":{"large_int":{"type":"integer"},"max_safe_int":{"type":"integer"},"negative_large":{"type":"integer"}},"required":["max_safe_int","large_int","negative_large"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_special_string_values_and_escaping_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['backslashes' => 'C:\\\\Users\\\\Path', 'empty_string' => '', 'quotes' => 'He said "hello" and \'goodbye\'', 'special_chars' => '!@#$%^&*()_+-=[]{}|;\':",./<>?', 'tabs_newlines' => 'line1
	line2
line3', 'unicode_escapes' => 'Hello', 'whitespace' => '   '], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/strings/', $handler, json_decode('{"additionalProperties":false,"properties":{"backslashes":{"type":"string"},"empty_string":{"type":"string"},"quotes":{"type":"string"},"special_chars":{"type":"string"},"tabs_newlines":{"type":"string"},"unicode_escapes":{"type":"string"},"whitespace":{"type":"string"}},"required":["empty_string","whitespace","tabs_newlines","quotes","backslashes","unicode_escapes","special_chars"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_edge_cases_unicode_and_emoji_handling_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['description' => 'Best café in München 🇩🇪', 'emoji_reactions' => '👍❤️😂🎉', 'id' => 1, 'name' => 'Coffee Shop ☕', 'tags' => ['食べ物', '音楽', '💰']], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"description":{"type":"string"},"emoji_reactions":{"type":"string"},"name":{"type":"string"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["name","description","tags","emoji_reactions"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_31_bearer_token_format_invalid_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^Bearer [A-Za-z0-9-._~+/]+=*$', 'value' => 'Bearer invalid token with spaces'], 'loc' => ['headers', 'authorization'], 'msg' => 'Invalid Bearer token format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_32_bearer_token_missing_prefix_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^Bearer [A-Za-z0-9-._~+/]+=*$', 'value' => 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9'], 'loc' => ['headers', 'authorization'], 'msg' => 'Invalid Bearer token format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected', $handler, null, null, json_decode('{"headers":{"Authorization":{"pattern":"^Bearer [A-Za-z0-9-._~+/]+=*$","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"pattern":"^[a-f0-9]{32}$","required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_34_api_key_header_invalid_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-f0-9]{32}$', 'value' => 'invalid-key'], 'loc' => ['headers', 'x-api-key'], 'msg' => 'Invalid API key format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"pattern":"^[a-f0-9]{32}$","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/accept', $handler, null, null, json_decode('{"headers":{"Accept":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/accept-encoding', $handler, null, null, json_decode('{"headers":{"Accept-Encoding":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/accept-language', $handler, null, null, json_decode('{"headers":{"Accept-Language":{"annotation":"str","type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_authorization_header_missing_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'authorization'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, json_decode('{"headers":{"Authorization":{"annotation":"str","required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_authorization_header_success_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['credentials' => 'foobar', 'scheme' => 'Digest'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, json_decode('{"headers":{"Authorization":{"annotation":"str","type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_authorization_header_wrong_scheme_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'Other invalidauthorization', 'loc' => ['headers', 'authorization'], 'msg' => 'String should match pattern \'^Digest .+\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, json_decode('{"headers":{"Authorization":{"annotation":"str","pattern":"^Digest .+","required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_basic_authentication_success_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['password' => 'password', 'username' => 'username'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/basic-auth', $handler, null, null, json_decode('{"headers":{"Authorization":{"annotation":"str","type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_bearer_token_authentication_missing_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'authorization'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/bearer-auth', $handler, null, null, json_decode('{"headers":{"Authorization":{"annotation":"str","pattern":"^Bearer .+","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/bearer-auth', $handler, null, null, json_decode('{"headers":{"Authorization":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/content-type', $handler, null, null, json_decode('{"headers":{"Content-Type":{"annotation":"str","type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_header_case_insensitivity_access_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['content_type_lower' => 'application/json', 'content_type_mixed' => 'application/json', 'content_type_upper' => 'application/json'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/echo', $handler, json_decode('{"additionalProperties":false,"properties":{"test":{"type":"string"}},"required":["test"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_headers_header_regex_validation_fail_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[0-9]{3,}$'], 'input' => 'invalid-format', 'loc' => ['headers', 'x-request-id'], 'msg' => 'String should match pattern \'^[0-9]{3,}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/pattern', $handler, null, null, json_decode('{"headers":{"X-Request-Id":{"annotation":"str","pattern":"^[0-9]{3,}$","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/pattern', $handler, null, null, json_decode('{"headers":{"X-Request-Id":{"annotation":"str","pattern":"^[0-9]{3,}$","type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_header_validation_max_length_constraint_fail_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 20], 'input' => 'this_is_way_too_long_for_validation', 'loc' => ['headers', 'x-session-id'], 'msg' => 'String should have at most 20 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/max-length', $handler, null, null, json_decode('{"headers":{"X-Session-Id":{"annotation":"str","maxLength":20,"type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_header_validation_min_length_constraint_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['headers', 'x-token'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/validated', $handler, null, null, json_decode('{"headers":{"X-Token":{"annotation":"str","minLength":3,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/underscore', $handler, null, null, json_decode('{"headers":{"X-Token":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/host', $handler, null, null, json_decode('{"headers":{"Host":{"annotation":"str","type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_multiple_custom_headers_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['x_client_version' => '1.2.3', 'x_request_id' => 'req-12345', 'x_trace_id' => 'trace-abc'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/multiple', $handler, null, null, json_decode('{"headers":{"X-Client-Version":{"annotation":"str","type":"string"},"X-Request-Id":{"annotation":"str","type":"string"},"X-Trace-Id":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"headers":{"x-token":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"headers":{"strange-header":{"annotation":"str","default":null,"optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/origin', $handler, null, null, json_decode('{"headers":{"Origin":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/referer', $handler, null, null, json_decode('{"headers":{"Referer":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"headers":{"User-Agent":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"headers":{"User-Agent":{"annotation":"str","default":"testclient","optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, json_decode('{"headers":{"key":{"annotation":"str","optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, json_decode('{"headers":{"key":{"annotation":"str","optional":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_headers_x_api_key_required_header_missing_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'x-api-key'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, json_decode('{"headers":{"X-API-Key":{"annotation":"str","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, json_decode('{"headers":{"key":{"annotation":"str","required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_delete_remove_resource_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response([], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('DELETE', '/items/1', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_delete_resource_not_found_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response([], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('DELETE', '/items/999', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_delete_with_response_body_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'message' => 'Item deleted successfully', 'name' => 'Deleted Item'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('DELETE', '/items/1', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('HEAD', '/items/1', $handler, null, null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_options_cors_preflight_request_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(null, 200, ['Access-Control-Max-Age' => '86400', 'Access-Control-Allow-Headers' => 'Content-Type', 'Access-Control-Allow-Methods' => 'GET, POST, PUT, DELETE, OPTIONS', 'Access-Control-Allow-Origin' => 'https://example.com']);
            }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/items/', $handler, null, null, null);
        return $app;
    }

    public static function create_http_methods_patch_partial_update_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'in_stock' => true, 'name' => 'Existing Item', 'price' => 79.99], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('PATCH', '/items/1', $handler, json_decode('{"properties":{"price":{"type":"number"}},"required":["price"],"type":"object"}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_patch_update_multiple_fields_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['id' => 1, 'in_stock' => false, 'name' => 'Updated Name', 'price' => 89.99], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('PATCH', '/items/1', $handler, json_decode('{"properties":{"in_stock":{"type":"boolean"},"name":{"type":"string"},"price":{"type":"number"}},"required":["in_stock","name","price"],"type":"object"}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_put_complete_resource_replacement_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['description' => 'Completely replaced', 'id' => 1, 'in_stock' => true, 'name' => 'Updated Item', 'price' => 99.99], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/1', $handler, json_decode('{"properties":{"description":{"type":"string"},"id":{"type":"integer"},"in_stock":{"type":"boolean"},"name":{"type":"string"},"price":{"type":"number"}},"required":["description","id","in_stock","name","price"],"type":"object"}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/999', $handler, json_decode('{"properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"number"}},"required":["id","name","price"],"type":"object"}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/1', $handler, json_decode('{"properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"number"}},"required":["id","name","price"],"type":"object"}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_put_missing_required_field_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '1', 'loc' => ['body', 'price'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/1', $handler, json_decode('{"properties":{"id":{"type":"integer"},"name":{"type":"string"},"price":{"type":"string"}},"required":["price"],"type":"object"}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_http_methods_put_validation_error_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '2 validation errors in request', 'errors' => [['input' => 'X', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short'], ['input' => -10, 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than 0', 'type' => 'greater_than']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/1', $handler, json_decode('{"$schema":"https://json-schema.org/draft/2020-12/schema","properties":{"id":{"type":"integer"},"name":{"minLength":3,"type":"string"},"price":{"exclusiveMinimum":0,"type":"number"}},"required":["id","name","price"],"type":"object"}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode('{"properties":{"profile":{"properties":{"email":{"format":"email","type":"string"},"name":{"minLength":1,"type":"string"}},"required":["name","email"],"type":"object"}},"required":["profile"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_30_nested_object_missing_field_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['required' => true], 'loc' => ['body', 'profile', 'email'], 'msg' => 'Field required', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode('{"properties":{"profile":{"properties":{"email":{"format":"email","type":"string"},"name":{"minLength":1,"type":"string"}},"required":["name","email"],"type":"object"}},"required":["profile"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode('{"properties":{"description":{"type":["string","null"]},"name":{"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/products', $handler, json_decode('{"definitions":{"Product":{"properties":{"name":{"type":"string"},"price":{"minimum":0,"type":"number"}},"required":["name","price"],"type":"object"}},"properties":{"product":{"$ref":"#/definitions/Product"}},"required":["product"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/items', $handler, json_decode('{"allOf":[{"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"},{"properties":{"price":{"minimum":0,"type":"number"}},"required":["price"],"type":"object"}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_34_additional_properties_false_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['additional_properties' => false, 'unexpected_field' => 'extra_field'], 'loc' => ['body', 'extra_field'], 'msg' => 'Additional properties are not allowed', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode('{"additionalProperties":false,"properties":{"email":{"type":"string"},"name":{"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/payment', $handler, json_decode('{"oneOf":[{"properties":{"credit_card":{"pattern":"^[0-9]{16}$","type":"string"}},"required":["credit_card"],"type":"object"},{"properties":{"paypal_email":{"format":"email","type":"string"}},"required":["paypal_email"],"type":"object"}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_36_oneof_schema_multiple_match_failure_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['matched_schemas' => 2], 'loc' => ['body'], 'msg' => 'Must match exactly one schema (oneOf), but matched 2', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/payment', $handler, json_decode('{"oneOf":[{"properties":{"credit_card":{"pattern":"^[0-9]{16}$","type":"string"}},"required":["credit_card"],"type":"object"},{"properties":{"paypal_email":{"format":"email","type":"string"}},"required":["paypal_email"],"type":"object"}]}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_37_oneof_schema_no_match_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['matched_schemas' => 0], 'loc' => ['body'], 'msg' => 'Must match exactly one schema (oneOf), but matched 0', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/payment', $handler, json_decode('{"oneOf":[{"properties":{"credit_card":{"pattern":"^[0-9]{16}$","type":"string"}},"required":["credit_card"],"type":"object"},{"properties":{"paypal_email":{"format":"email","type":"string"}},"required":["paypal_email"],"type":"object"}]}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/contact', $handler, json_decode('{"anyOf":[{"required":["email"]},{"required":["phone"]}],"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/contact', $handler, json_decode('{"anyOf":[{"required":["email"]},{"required":["phone"]}],"properties":{"email":{"format":"email","type":"string"},"name":{"type":"string"},"phone":{"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_40_anyof_schema_failure_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['matched_schemas' => 0], 'loc' => ['body'], 'msg' => 'Must match at least one schema (anyOf), but matched 0', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/contact', $handler, json_decode('{"anyOf":[{"required":["email"]},{"required":["phone"]}],"properties":{"email":{"format":"email","type":"string"},"name":{"type":"string"},"phone":{"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode('{"properties":{"username":{"not":{"enum":["admin","root","system"]},"type":"string"}},"required":["username"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_42_not_schema_failure_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['prohibited_value' => 'admin'], 'loc' => ['body', 'username'], 'msg' => 'Must not match the schema', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode('{"properties":{"username":{"not":{"enum":["admin","root","system"]},"type":"string"}},"required":["username"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/api/v1/data', $handler, json_decode('{"properties":{"data":{"type":"string"},"version":{"const":"1.0","type":"string"}},"required":["version","data"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_44_const_validation_failure_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['const' => '1.0', 'value' => '2.0'], 'loc' => ['body', 'version'], 'msg' => 'Value must be exactly \'1.0\'', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/v1/data', $handler, json_decode('{"properties":{"data":{"type":"string"},"version":{"const":"1.0","type":"string"}},"required":["version","data"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/config', $handler, json_decode('{"minProperties":2,"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_46_minproperties_validation_failure_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_properties' => 1, 'min_properties' => 2], 'loc' => ['body'], 'msg' => 'Object must have at least 2 properties', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/config', $handler, json_decode('{"minProperties":2,"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_47_maxproperties_validation_failure_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_properties' => 4, 'max_properties' => 3], 'loc' => ['body'], 'msg' => 'Object must have at most 3 properties', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/config', $handler, json_decode('{"maxProperties":3,"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/billing', $handler, json_decode('{"dependencies":{"credit_card":["billing_address"]},"properties":{"billing_address":{"type":"string"},"credit_card":{"type":"string"},"name":{"type":"string"}},"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_49_dependencies_validation_failure_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['dependency' => 'credit_card', 'required_fields' => ['billing_address']], 'loc' => ['body'], 'msg' => 'When \'credit_card\' is present, \'billing_address\' is required', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/billing', $handler, json_decode('{"dependencies":{"credit_card":["billing_address"]},"properties":{"billing_address":{"type":"string"},"credit_card":{"type":"string"},"name":{"type":"string"}},"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"properties":{"user":{"properties":{"profile":{"properties":{"contact":{"properties":{"address":{"properties":{"street":{"type":"string"}},"required":["street"],"type":"object"}},"required":["address"],"type":"object"}},"required":["contact"],"type":"object"}},"required":["profile"],"type":"object"}},"required":["user"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_array_of_objects_success_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['images' => [['name' => 'Front', 'url' => 'https://example.com/img1.jpg'], ['name' => 'Back', 'url' => 'https://example.com/img2.jpg']], 'name' => 'Product Bundle', 'tags' => ['electronics', 'gadget']], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/list', $handler, json_decode('{"additionalProperties":false,"properties":{"images":{"items":{"additionalProperties":false,"properties":{"name":{"type":"string"},"url":{"type":"string"}},"required":["url","name"],"type":"object"},"type":"array"},"name":{"type":"string"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["name","tags","images"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_array_of_primitive_values_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Product', 'ratings' => [4.5, 4.8, 5.0, 4.2], 'tags' => ['electronics', 'gadget', 'new']], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"ratings":{"items":{"type":"number"},"type":"array"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["name","tags","ratings"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/items/?limit=10', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"}', true), null, json_decode('{"query":{"limit":{"type":"integer"}}}', true));
        return $app;
    }

    public static function create_json_bodies_boolean_field_success_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['in_stock' => true, 'name' => 'Item', 'price' => 42.0], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"in_stock":{"type":"boolean"},"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price","in_stock"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_date_field_success_27(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['event_date' => '2024-03-15', 'name' => 'Conference'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/events/', $handler, json_decode('{"additionalProperties":false,"properties":{"event_date":{"type":"string"},"name":{"type":"string"}},"required":["name","event_date"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_datetime_field_success_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['created_at' => '2024-03-15T10:30:00Z', 'name' => 'Meeting'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/events/', $handler, json_decode('{"additionalProperties":false,"properties":{"created_at":{"format":"date-time","type":"string"},"name":{"type":"string"}},"required":["name","created_at"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_deeply_nested_objects_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['name' => 'Product', 'price' => 100.0, 'seller' => ['address' => ['city' => 'Springfield', 'country' => ['code' => 'US', 'name' => 'USA'], 'street' => '123 Main St'], 'name' => 'John Doe']], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/nested', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"seller":{"additionalProperties":false,"properties":{"address":{"additionalProperties":false,"properties":{"city":{"type":"string"},"country":{"additionalProperties":false,"properties":{"code":{"type":"string"},"name":{"type":"string"}},"required":["name","code"],"type":"object"},"street":{"type":"string"}},"required":["street","city","country"],"type":"object"},"name":{"type":"string"}},"required":["name","address"],"type":"object"}},"required":["name","price","seller"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_empty_json_object_30(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['description' => null, 'name' => null, 'price' => null, 'tax' => null], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/optional-all', $handler, json_decode('{"additionalProperties":false,"properties":{},"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_empty_array_validation_fail_31(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 1], 'input' => [], 'loc' => ['body', 'tags'], 'msg' => 'List should have at least 1 item after validation', 'type' => 'too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/list-validated', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"tags":{"items":{},"minItems":1,"type":"array"}},"required":["name","tags"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_enum_field_invalid_value_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'electronics\', \'clothing\' or \'books\''], 'input' => 'furniture', 'loc' => ['body', 'category'], 'msg' => 'Input should be \'electronics\', \'clothing\' or \'books\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"category":{"enum":["electronics","clothing","books"],"type":"string"},"name":{"type":"string"}},"required":["name","category"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_enum_field_success_33(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['category' => 'electronics', 'name' => 'Item'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"category":{"type":"string"},"name":{"type":"string"}},"required":["name","category"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"another_extra":{"type":"integer"},"extra_field":{"type":"string"},"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price","extra_field","another_extra"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_field_type_validation_invalid_type_35(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not a number', 'loc' => ['body', 'price'], 'msg' => 'Input should be a valid number', 'type' => 'float_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"description":{"type":"string"},"name":{"type":"string"},"price":{"type":"number"},"tax":{"type":"number"}},"required":["name","description","price","tax"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_nested_object_success_36(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['image' => ['name' => 'Product Image', 'url' => 'https://example.com/image.jpg'], 'name' => 'Foo', 'price' => 42.0], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/nested', $handler, json_decode('{"additionalProperties":false,"properties":{"image":{"additionalProperties":false,"properties":{"name":{"type":"string"},"url":{"type":"string"}},"required":["url","name"],"type":"object"},"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price","image"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_null_value_for_optional_field_37(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['description' => null, 'name' => 'Item', 'price' => 42.0, 'tax' => null], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"description":{"type":"null"},"name":{"type":"string"},"price":{"type":"number"},"tax":{"type":"null"}},"required":["name","price","description","tax"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_numeric_ge_validation_fail_38(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['ge' => 1], 'input' => 0.5, 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than or equal to 1', 'type' => 'greater_than_equal']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"minimum":1,"type":"number"}},"required":["name","price"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_optional_fields_omitted_40(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['description' => null, 'name' => 'Foo', 'price' => 35.4, 'tax' => null], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_patch_partial_update_41(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['description' => 'Original description', 'name' => 'Original Item', 'price' => 45.0], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('PATCH', '/items/1', $handler, json_decode('{"properties":{"price":{"type":"number"}},"required":["price"],"type":"object"}', true), null, json_decode('{"path":{"id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_json_bodies_required_field_missing_validation_error_42(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['body', 'name'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"description":{"type":"string"},"name":{"type":"string"},"price":{"type":"number"}},"required":["description","price","name"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_simple_json_object_success_43(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['description' => 'A very nice Item', 'name' => 'Foo', 'price' => 35.4, 'tax' => 3.2], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"description":{"type":"string"},"name":{"type":"string"},"price":{"type":"number"},"tax":{"type":"number"}},"required":["name","description","price","tax"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_string_max_length_validation_fail_44(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 50], 'input' => 'This is a very long name that exceeds the maximum length', 'loc' => ['body', 'name'], 'msg' => 'String should have at most 50 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"maxLength":50,"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_string_min_length_validation_fail_45(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"minLength":3,"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_string_pattern_validation_fail_46(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[A-Z]{3}[0-9]{4}$'], 'input' => 'ABC-123', 'loc' => ['body', 'sku'], 'msg' => 'String should match pattern \'^[A-Z]{3}[0-9]{4}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"sku":{"pattern":"^[A-Z]{3}[0-9]{4}$","type":"string"}},"required":["name","sku"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"sku":{"type":"string"}},"required":["name","sku"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_uuid_field_invalid_format_48(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-valid-uuid', 'loc' => ['body', 'item_id'], 'msg' => 'Input should be a valid UUID', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"item_id":{"format":"uuid","type":"string"},"name":{"type":"string"}},"required":["name","item_id"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_json_bodies_uuid_field_success_49(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['item_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716', 'name' => 'Item'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"item_id":{"format":"uuid","type":"string"},"name":{"type":"string"}},"required":["name","item_id"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_hook_execution_order_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['execution_order' => ['first_hook', 'second_hook', 'third_hook'], 'message' => 'Hooks executed in order'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-hook-order', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_multiple_hooks_all_phases_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['action' => 'update_profile', 'message' => 'Action completed successfully', 'request_id' => '.*', 'user_id' => 'user-123'], 200, ['X-Content-Type-Options' => 'nosniff', 'X-Response-Time' => '.*ms', 'X-Request-ID' => '.*', 'X-Frame-Options' => 'DENY']);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/full-lifecycle', $handler, json_decode('{"properties":{"action":{"type":"string"},"user_id":{"type":"string"}},"required":["user_id","action"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_onerror_error_logging_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['error' => 'Internal Server Error', 'error_id' => '.*', 'message' => 'An unexpected error occurred'], 500, ['Content-Type' => 'application/json']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-error', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_onrequest_request_logging_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['has_request_id' => true, 'message' => 'onRequest hooks executed', 'request_logged' => true], 200, ['X-Request-ID' => '.*']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-on-request', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-timing', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_onresponse_security_headers_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Response with security headers'], 200, ['Strict-Transport-Security' => 'max-age=31536000; includeSubDomains', 'X-XSS-Protection' => '1; mode=block', 'X-Content-Type-Options' => 'nosniff', 'X-Frame-Options' => 'DENY']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-security-headers', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected-resource-fail', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_prehandler_authentication_success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['authenticated' => true, 'message' => 'Access granted', 'user_id' => 'user-123'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected-resource', $handler, null, null, null);
        return $app;
    }

    public static function create_lifecycle_hooks_prehandler_authorization_check_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['message' => 'Admin access granted', 'role' => 'admin', 'user_id' => 'admin-456'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/admin-only', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/api/admin-only-forbidden', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/api/test-rate-limit-exceeded', $handler, json_decode('{"properties":{"data":{"type":"string"}},"required":["data"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/api/test-rate-limit', $handler, json_decode('{"properties":{"data":{"type":"string"}},"required":["data"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, null, null, json_decode('{"files":{"image":{"content_type":["image/png"],"required":true,"validate_magic_numbers":true}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, null, null, json_decode('{"files":{"image":{"content_type":["image/jpeg"],"required":true,"validate_magic_numbers":true}}}', true));
        return $app;
    }

    public static function create_multipart_19_file_mime_spoofing_png_as_jpeg_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['declared_mime' => 'image/jpeg', 'detected_type' => 'image/png', 'magic_bytes' => '89504e470d0a1a0a'], 'loc' => ['files', 'image'], 'msg' => 'File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, null, null, json_decode('{"files":{"image":{"content_type":["image/jpeg"],"required":true,"validate_magic_numbers":true}}}', true));
        return $app;
    }

    public static function create_multipart_20_file_mime_spoofing_jpeg_as_png_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['declared_mime' => 'image/png', 'detected_type' => 'image/jpeg', 'magic_bytes' => 'ffd8ffe0'], 'loc' => ['files', 'image'], 'msg' => 'File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, null, null, json_decode('{"files":{"image":{"content_type":["image/png"],"required":true,"validate_magic_numbers":true}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, null, null, json_decode('{"files":{"document":{"content_type":["application/pdf"],"required":true,"validate_magic_numbers":true}}}', true));
        return $app;
    }

    public static function create_multipart_22_file_empty_buffer_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['buffer_size' => 0], 'loc' => ['files', 'file'], 'msg' => 'File buffer is empty', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, null, null, json_decode('{"files":{"file":{"required":true,"validate_magic_numbers":true}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/files/images-only', $handler, json_decode('{"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"type":"object"}', true), null, json_decode('{"files":{"file":{"content_type":["image/jpeg","image/png","image/gif"],"required":true}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/files/upload', $handler, json_decode('{"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"required":["file"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/files/list', $handler, json_decode('{"additionalProperties":false,"properties":{"files":{"items":{"format":"binary","type":"string"},"type":"array"}},"required":["files"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/files/validated', $handler, json_decode('{"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_multipart_file_upload_with_custom_headers_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['test2' => ['content' => '<file2 content>', 'content_type' => 'text/plain', 'filename' => 'test2.txt', 'headers' => [['content-disposition', 'form-data; name="test2"; filename="test2.txt"'], ['content-type', 'text/plain'], ['x-custom', 'f2']], 'size' => 15]], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode('{"additionalProperties":false,"properties":{"test2":{"format":"binary","type":"string"}},"required":["test2"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode('{"additionalProperties":false,"properties":{"test1":{"format":"binary","type":"string"}},"required":["test1"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode('{"additionalProperties":false,"properties":{"some":{"type":"string"}},"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_multipart_image_file_upload_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['content_type' => 'image/jpeg', 'filename' => 'photo.jpg', 'size' => 22], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/image', $handler, json_decode('{"additionalProperties":false,"properties":{"image":{"format":"binary","type":"string"}},"required":["image"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_multipart_mixed_files_and_form_data_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['active' => 'true', 'age' => '25', 'file' => ['content' => 'file data here', 'content_type' => 'text/plain', 'filename' => 'upload.txt', 'size' => 14], 'username' => 'testuser'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode('{"additionalProperties":false,"properties":{"active":{"type":"string"},"age":{"type":"string"},"file":{"format":"binary","type":"string"},"username":{"type":"string"}},"required":["file"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_multipart_multiple_file_uploads_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['test1' => ['content' => '<file1 content>', 'content_type' => 'text/plain', 'filename' => 'test1.txt', 'size' => 15], 'test2' => ['content' => '<file2 content>', 'content_type' => 'text/plain', 'filename' => 'test2.txt', 'size' => 15]], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode('{"additionalProperties":false,"properties":{"test1":{"format":"binary","type":"string"},"test2":{"format":"binary","type":"string"}},"required":["test1","test2"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_multipart_multiple_values_for_same_field_name_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['files' => [['content' => 'first file', 'content_type' => 'text/plain', 'filename' => 'file1.txt', 'size' => 10], ['content' => 'second file', 'content_type' => 'text/plain', 'filename' => 'file2.txt', 'size' => 11]], 'tags' => ['python', 'rust', 'web']], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode('{"additionalProperties":false,"properties":{"files":{"items":{"format":"binary","type":"string"},"type":"array"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["files"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/files/optional', $handler, json_decode('{"additionalProperties":false,"properties":{},"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_multipart_optional_file_upload_provided_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['content_type' => 'text/plain', 'filename' => 'optional.txt', 'size' => 21], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/optional', $handler, json_decode('{"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"required":["file"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_multipart_pdf_file_upload_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['content_type' => 'application/pdf', 'filename' => 'report.pdf', 'size' => 16], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/document', $handler, json_decode('{"additionalProperties":false,"properties":{"document":{"format":"binary","type":"string"}},"required":["document"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_multipart_required_file_upload_missing_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'required', 'loc' => ['body', 'file'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/required', $handler, json_decode('{"additionalProperties":false,"properties":{"file":{"format":"binary","type":"string"}},"required":["file"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_multipart_simple_file_upload_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['test' => ['content' => '<file content>', 'content_type' => 'text/plain', 'filename' => 'test.txt', 'size' => 14]], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode('{"additionalProperties":false,"properties":{"test":{"format":"binary","type":"string"}},"required":["test"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/e8b5a51d-11c8-3310-a6ab-367563f20686', $handler, null, null, json_decode('{"path":{"id":{"format":"uuid","type":"string","uuidVersion":"3"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/630eb68f-e0fa-5ecc-887a-7c7a62614681', $handler, null, null, json_decode('{"path":{"id":{"format":"uuid","type":"string","uuidVersion":"5"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/events/2025-10-30', $handler, null, null, json_decode('{"path":{"date":{"format":"date","type":"string"}}}', true));
        return $app;
    }

    public static function create_path_params_25_date_format_invalid_failure_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'date', 'value' => '2025-13-45'], 'loc' => ['path', 'date'], 'msg' => 'Invalid date format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/events/2025-13-45', $handler, null, null, json_decode('{"path":{"date":{"format":"date","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/bookings/2025-10-30T14:30:00Z', $handler, null, null, json_decode('{"path":{"timestamp":{"format":"date-time","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/delays/P1DT2H30M', $handler, null, null, json_decode('{"path":{"duration":{"format":"duration","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/prices/19.99', $handler, null, null, json_decode('{"path":{"amount":{"format":"decimal","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/users/alice', $handler, null, null, json_decode('{"path":{"username":{"minLength":3,"type":"string"}}}', true));
        return $app;
    }

    public static function create_path_params_31_string_minlength_path_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 2, 'min_length' => 3], 'loc' => ['path', 'username'], 'msg' => 'String length must be at least 3', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/ab', $handler, null, null, json_decode('{"path":{"username":{"minLength":3,"required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_path_params_32_string_maxlength_path_failure_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 42, 'max_length' => 20], 'loc' => ['path', 'username'], 'msg' => 'String length must not exceed 20', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/this_username_is_way_too_long_to_be_valid', $handler, null, null, json_decode('{"path":{"username":{"maxLength":20,"required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/repos/spikard-labs/spikard-http', $handler, null, null, json_decode('{"path":{"owner":{"pattern":"^[a-zA-Z0-9-]+$","type":"string"},"repo":{"pattern":"^[a-zA-Z0-9-_]+$","type":"string"}}}', true));
        return $app;
    }

    public static function create_path_params_34_string_pattern_path_failure_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9-]+$', 'value' => 'invalid@owner'], 'loc' => ['path', 'owner'], 'msg' => 'String does not match pattern', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/repos/invalid@owner', $handler, null, null, json_decode('{"path":{"owner":{"pattern":"^[a-zA-Z0-9-]+$","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/offset/-100', $handler, null, null, json_decode('{"path":{"value":{"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/bool/True', $handler, null, null, json_decode('{"path":{"item_id":{"type":"boolean"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/bool/1', $handler, null, null, json_decode('{"path":{"item_id":{"type":"boolean"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/date/2023-07-15', $handler, null, null, json_decode('{"path":{"date_param":{"format":"date","type":"string"}}}', true));
        return $app;
    }

    public static function create_path_params_enum_path_parameter_invalid_value_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\''], 'input' => 'foo', 'loc' => ['path', 'model_name'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/models/foo', $handler, null, null, json_decode('{"path":{"model_name":{"enum":["alexnet","resnet","lenet"],"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/models/alexnet', $handler, null, null, json_decode('{"path":{"model_name":{"enum":["alexnet","lenet","resnet"],"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/float/42.5', $handler, null, null, json_decode('{"path":{"item_id":{"type":"number"}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_invalid_string_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'foobar', 'loc' => ['path', 'item_id'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/int/foobar', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/int/42', $handler, null, null, json_decode('{"path":{"item_id":{"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-lt-gt/2', $handler, null, null, json_decode('{"path":{"item_id":{"exclusiveMaximum":3,"exclusiveMinimum":1,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-ge/3', $handler, null, null, json_decode('{"path":{"item_id":{"minimum":3,"type":"integer"}}}', true));
        return $app;
    }

    public static function create_path_params_integer_path_parameter_with_gt_constraint_failure_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['gt' => 3], 'input' => 2, 'loc' => ['path', 'item_id'], 'msg' => 'Input should be greater than 3', 'type' => 'greater_than']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-gt/2', $handler, null, null, json_decode('{"path":{"item_id":{"exclusiveMinimum":3,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-gt/42', $handler, null, null, json_decode('{"path":{"item_id":{"exclusiveMinimum":3,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-le/3', $handler, null, null, json_decode('{"path":{"item_id":{"maximum":3,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-lt/2', $handler, null, null, json_decode('{"path":{"item_id":{"exclusiveMaximum":3,"type":"integer"}}}', true));
        return $app;
    }

    public static function create_path_params_multiple_path_parameters_success_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['order_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716', 'service_id' => 1, 'user_id' => 'abc', 'version' => 1.0], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716', $handler, null, null, json_decode('{"path":{"order_id":{"format":"uuid","type":"string"},"service_id":{"type":"integer"},"user_id":{"type":"string"},"version":{"type":"number"}}}', true));
        return $app;
    }

    public static function create_path_params_path_parameter_type_syntax_invalid_uuid_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-uuid', 'loc' => ['path', 'id'], 'msg' => 'Input should be a valid UUID', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/type-syntax/items/not-a-uuid', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/type-syntax/items-count/50', $handler, null, null, json_decode('{"path":{"count":{"maximum":100,"minimum":1,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/type-syntax/items/550e8400-e29b-41d4-a716-446655440000', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/type-syntax/users/42', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/files/home/johndoe/myfile.txt', $handler, null, null, json_decode('{"path":{"file_path":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/path/str/foobar', $handler, null, null, json_decode('{"path":{"item_id":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_path_params_string_path_parameter_with_max_length_failure_35(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 3], 'input' => 'foobar', 'loc' => ['path', 'item_id'], 'msg' => 'String should have at most 3 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-maxlength/foobar', $handler, null, null, json_decode('{"path":{"item_id":{"maxLength":3,"type":"string"}}}', true));
        return $app;
    }

    public static function create_path_params_string_path_parameter_with_min_length_failure_36(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'fo', 'loc' => ['path', 'item_id'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-minlength/fo', $handler, null, null, json_decode('{"path":{"item_id":{"minLength":3,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a', $handler, null, null, json_decode('{"path":{"item_id":{"format":"uuid","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/negative', $handler, null, null, json_decode('{"query":{"offset":{"annotation":"int","type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/stats', $handler, null, null, json_decode('{"query":{"threshold":{"annotation":"float","type":"number"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/search', $handler, null, null, json_decode('{"query":{"term":{"minLength":3,"type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_45_string_minlength_validation_failure_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 2, 'min_length' => 3], 'loc' => ['query', 'term'], 'msg' => 'String length must be at least 3', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/search', $handler, null, null, json_decode('{"query":{"term":{"minLength":3,"required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_46_string_maxlength_validation_failure_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 21, 'max_length' => 10], 'loc' => ['query', 'term'], 'msg' => 'String length must not exceed 10', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/search', $handler, null, null, json_decode('{"query":{"term":{"maxLength":10,"required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/subscribe', $handler, null, null, json_decode('{"query":{"email":{"pattern":"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$","type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_48_pattern_validation_email_failure_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$', 'value' => 'invalid-email'], 'loc' => ['query', 'email'], 'msg' => 'String does not match pattern', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/subscribe', $handler, null, null, json_decode('{"query":{"email":{"pattern":"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"limit":{"exclusiveMinimum":0,"type":"integer"}}}', true));
        return $app;
    }

    public static function create_query_params_50_integer_gt_constraint_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['exclusive_minimum' => 0, 'value' => 0], 'loc' => ['query', 'limit'], 'msg' => 'Value must be greater than 0', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"limit":{"exclusiveMinimum":0,"required":true,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"offset":{"minimum":0,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"limit":{"maximum":100,"type":"integer"}}}', true));
        return $app;
    }

    public static function create_query_params_53_integer_le_constraint_failure_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['maximum' => 100, 'value' => 101], 'loc' => ['query', 'limit'], 'msg' => 'Value must not exceed 100', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"limit":{"maximum":100,"required":true,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"ids":{"items":{"type":"integer"},"minItems":2,"type":"array"}}}', true));
        return $app;
    }

    public static function create_query_params_55_array_minitems_constraint_failure_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_items' => 1, 'min_items' => 2], 'loc' => ['query', 'ids'], 'msg' => 'Array must contain at least 2 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"ids":{"items":{"type":"integer"},"minItems":2,"required":true,"type":"array"}}}', true));
        return $app;
    }

    public static function create_query_params_56_array_maxitems_constraint_failure_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_items' => 6, 'max_items' => 5], 'loc' => ['query', 'tags'], 'msg' => 'Array must not contain more than 5 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"tags":{"items":{"type":"string"},"maxItems":5,"required":true,"type":"array"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"active":{"type":"boolean"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/subscribe', $handler, null, null, json_decode('{"query":{"email":{"format":"email","type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_59_format_email_failure_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'email', 'value' => 'not-an-email'], 'loc' => ['query', 'email'], 'msg' => 'Invalid email format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/subscribe', $handler, null, null, json_decode('{"query":{"email":{"format":"email","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/network', $handler, null, null, json_decode('{"query":{"ip":{"format":"ipv4","type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_61_format_ipv4_failure_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'ipv4', 'value' => '999.999.999.999'], 'loc' => ['query', 'ip'], 'msg' => 'Invalid IPv4 address format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/network', $handler, null, null, json_decode('{"query":{"ip":{"format":"ipv4","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/network/ipv6', $handler, null, null, json_decode('{"query":{"ip":{"format":"ipv6","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/redirect', $handler, null, null, json_decode('{"query":{"url":{"format":"uri","type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_64_format_uri_failure_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'uri', 'value' => 'not a uri'], 'loc' => ['query', 'url'], 'msg' => 'Invalid URI format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/redirect', $handler, null, null, json_decode('{"query":{"url":{"format":"uri","required":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/dns', $handler, null, null, json_decode('{"query":{"host":{"format":"hostname","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"quantity":{"multipleOf":5,"type":"integer"}}}', true));
        return $app;
    }

    public static function create_query_params_67_multipleof_constraint_failure_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['multiple_of' => 5, 'value' => 17], 'loc' => ['query', 'quantity'], 'msg' => 'Value must be a multiple of 5', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"quantity":{"multipleOf":5,"required":true,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"ids":{"items":{"type":"integer"},"type":"array","uniqueItems":true}}}', true));
        return $app;
    }

    public static function create_query_params_69_array_uniqueitems_failure_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['duplicate_index' => 2, 'duplicate_value' => 2, 'unique_items' => true], 'loc' => ['query', 'ids'], 'msg' => 'Array items must be unique', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, null, null, json_decode('{"query":{"ids":{"items":{"type":"integer"},"required":true,"type":"array","uniqueItems":true}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items?tags=python|rust|typescript', $handler, null, null, json_decode('{"query":{"tags":{"items":{"type":"string"},"separator":"|","type":"array"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items?colors=red;green;blue', $handler, null, null, json_decode('{"query":{"colors":{"items":{"type":"string"},"separator":";","type":"array"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/search?keywords=rust%20web%20framework', $handler, null, null, json_decode('{"query":{"keywords":{"items":{"type":"string"},"separator":" ","type":"array"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list-default', $handler, null, null, json_decode('{"query":{"tags":{"annotation":"list[str]","default":[],"items":{"type":"string"},"optional":true,"type":"array"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list-default', $handler, null, null, json_decode('{"query":{"tags":{"annotation":"list[str]","default":[],"items":{"type":"string"},"optional":true,"type":"array"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/bool', $handler, null, null, json_decode('{"query":{"flag":{"annotation":"bool","type":"boolean"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/bool', $handler, null, null, json_decode('{"query":{"flag":{"annotation":"bool","type":"boolean"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/date', $handler, null, null, json_decode('{"query":{"event_date":{"annotation":"str","format":"date","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/datetime', $handler, null, null, json_decode('{"query":{"timestamp":{"annotation":"str","format":"date-time","type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_enum_query_parameter_invalid_value_38(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\''], 'input' => 'vgg16', 'loc' => ['query', 'model'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/enum', $handler, null, null, json_decode('{"query":{"model":{"annotation":"str","enum":["alexnet","resnet","lenet"],"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/enum', $handler, null, null, json_decode('{"query":{"model":{"annotation":"str","enum":["alexnet","resnet","lenet"],"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/float-ge', $handler, null, null, json_decode('{"query":{"price":{"annotation":"float","minimum":0.01,"type":"number"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int-ge', $handler, null, null, json_decode('{"query":{"value":{"annotation":"int","minimum":10,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int-gt', $handler, null, null, json_decode('{"query":{"value":{"annotation":"int","exclusiveMinimum":0,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int-le', $handler, null, null, json_decode('{"query":{"value":{"annotation":"int","maximum":100,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int-lt', $handler, null, null, json_decode('{"query":{"value":{"annotation":"int","exclusiveMaximum":50,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int/default', $handler, null, null, json_decode('{"query":{"query":{"annotation":"int","default":10,"optional":true,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int/default', $handler, null, null, json_decode('{"query":{"query":{"annotation":"int","default":10,"optional":true,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list', $handler, null, null, json_decode('{"query":{"device_ids":{"annotation":"list[int]","items":{"type":"integer"},"type":"array"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"query":{"q":{"annotation":"list[str]","items":{"type":"string"},"optional":true,"type":"array"}}}', true));
        return $app;
    }

    public static function create_query_params_list_query_parameter_required_but_missing_49(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'device_ids'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list', $handler, null, null, json_decode('{"query":{"device_ids":{"annotation":"list[int]","items":{"type":"integer"},"type":"array"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list-default', $handler, null, null, json_decode('{"query":{"tags":{"annotation":"list[str]","default":[],"items":{"type":"string"},"optional":true,"type":"array"}}}', true));
        return $app;
    }

    public static function create_query_params_multiple_query_parameters_with_different_types_51(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['active' => true, 'age' => 30, 'name' => 'john', 'score' => 95.5], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/multi-type', $handler, null, null, json_decode('{"query":{"active":{"annotation":"bool","type":"boolean"},"age":{"annotation":"int","type":"integer"},"name":{"annotation":"str","type":"string"},"score":{"annotation":"float","type":"number"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int/optional', $handler, null, null, json_decode('{"query":{"query":{"annotation":"int","optional":true,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/optional-default', $handler, null, null, json_decode('{"query":{"limit":{"annotation":"int","default":10,"optional":true,"type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/optional', $handler, null, null, json_decode('{"query":{"query":{"annotation":"str","optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/optional', $handler, null, null, json_decode('{"query":{"query":{"annotation":"str","optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/basic', $handler, null, null, json_decode('{"query":{"name":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/basic', $handler, null, null, json_decode('{"query":{"name":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/test', $handler, null, null, json_decode('{"query":{"email":{"annotation":"str","type":"string"},"special":{"annotation":"str","type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_required_integer_query_parameter_float_value_59(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 42.5, 'loc' => ['query', 'query'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int', $handler, null, null, json_decode('{"query":{"query":{"annotation":"int","type":"integer"}}}', true));
        return $app;
    }

    public static function create_query_params_required_integer_query_parameter_invalid_type_60(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'baz', 'loc' => ['query', 'query'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int', $handler, null, null, json_decode('{"query":{"query":{"annotation":"int","type":"integer"}}}', true));
        return $app;
    }

    public static function create_query_params_required_integer_query_parameter_missing_61(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'query'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int', $handler, null, null, json_decode('{"query":{"query":{"annotation":"int","type":"integer"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int', $handler, null, null, json_decode('{"query":{"query":{"annotation":"int","type":"integer"}}}', true));
        return $app;
    }

    public static function create_query_params_required_string_query_parameter_missing_63(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'query'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query', $handler, null, null, json_decode('{"query":{"query":{"annotation":"str","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query', $handler, null, null, json_decode('{"query":{"query":{"annotation":"str","type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_string_query_param_with_max_length_constraint_fail_65(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 10], 'input' => 'this_is_way_too_long', 'loc' => ['query', 'name'], 'msg' => 'String should have at most 10 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/str-max-length', $handler, null, null, json_decode('{"query":{"name":{"annotation":"str","maxLength":10,"type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_string_query_param_with_min_length_constraint_fail_66(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['query', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/str-min-length', $handler, null, null, json_decode('{"query":{"name":{"annotation":"str","minLength":3,"type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_string_query_param_with_regex_pattern_fail_67(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[0-9]{3,}$'], 'input' => 'abc123', 'loc' => ['query', 'code'], 'msg' => 'String should match pattern \'^[0-9]{3,}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/pattern', $handler, null, null, json_decode('{"query":{"code":{"annotation":"str","pattern":"^[0-9]{3,}$","type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_string_validation_with_regex_failure_68(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^fixedquery$'], 'input' => 'nonregexquery', 'loc' => ['query', 'item_query'], 'msg' => 'String should match pattern \'^fixedquery$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"query":{"item_query":{"annotation":"str","pattern":"^fixedquery$","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"query":{"item_query":{"annotation":"str","pattern":"^fixedquery$","type":"string"}}}', true));
        return $app;
    }

    public static function create_query_params_uuid_query_parameter_invalid_format_70(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-uuid', 'loc' => ['query', 'item_id'], 'msg' => 'Input should be a valid UUID', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/uuid', $handler, null, null, json_decode('{"query":{"item_id":{"annotation":"str","format":"uuid","type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('GET', '/query/uuid', $handler, null, null, json_decode('{"query":{"item_id":{"annotation":"str","format":"uuid","type":"string"}}}', true));
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
                return new Response(['request' => 'under-limit', 'status' => 'ok'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/rate-limit/basic', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/rate-limit/exceeded', $handler, null, null, null);
        return $app;
    }

    public static function create_request_id_request_id_header_is_preserved_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['echo' => 'trace-123', 'status' => 'preserved'], 200, ['x-request-id' => 'trace-123']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/request-id/preserved', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/request-id/generated', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/request-id/disabled', $handler, null, null, null);
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
                return new Response(['duration' => 'fast', 'status' => 'ok'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/timeouts/fast', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/timeouts/slow', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/public/hello.txt', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/app/', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, json_decode('{"properties":{"data":{"type":"string"}},"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/status-test/200', $handler, null, null, json_decode('{"path":{"code":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/tasks/', $handler, json_decode('{"additionalProperties":false,"properties":{"task":{"type":"string"}},"required":["task"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('DELETE', '/status-test/204', $handler, null, null, json_decode('{"path":{"code":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_status_codes_206_partial_content_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response('binary_data_1024_bytes', 206, ['Content-Range' => 'bytes 0-1023/5000', 'Content-Type' => 'application/pdf', 'Accept-Ranges' => 'bytes', 'Content-Length' => '1024']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/files/document.pdf', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_20_414_uri_too_long_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response([], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/data?skip_template_expansion=true', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/data', $handler, null, null, json_decode('{"headers":{"X-Large-Header":{"optional":true,"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('TRACE', '/data', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/data', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/old-path', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/temp-redirect', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/status-test/304', $handler, null, null, json_decode('{"headers":{"If-None-Match":{"optional":true,"type":"string"}},"path":{"code":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_status_codes_307_temporary_redirect_method_preserved_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response([], 307, ['location' => '/target-post']);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/redirect-post', $handler, json_decode('{"additionalProperties":false,"properties":{},"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"type":"string"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/admin/users', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/status-test/404', $handler, null, null, json_decode('{"path":{"code":{"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/slow-endpoint', $handler, json_decode('{"additionalProperties":false,"properties":{"data":{"type":"string"}},"required":["data"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_status_codes_422_unprocessable_entity_validation_error_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['body', 'name'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"string"}},"required":["price","name"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_status_codes_429_too_many_requests_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Rate limit exceeded. Try again in 60 seconds.'], 429, ['Retry-After' => '60', 'X-RateLimit-Limit' => '100', 'X-RateLimit-Reset' => '1609459200', 'X-RateLimit-Remaining' => '0']);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/resource', $handler, null, null, null);
        return $app;
    }

    public static function create_status_codes_500_internal_server_error_server_error_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => 'Internal server error', 'status' => 500, 'title' => 'Internal Server Error', 'type' => 'https://spikard.dev/errors/internal-server-error'], 500, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/error', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/health', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/stream/logfile', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/stream/csv-report', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('GET', '/stream/json-lines', $handler, null, null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/register', $handler, json_decode('{"properties":{"tags":{"items":{"type":"string"},"minItems":1,"type":"array"}},"required":["tags"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_14_nested_object_bracket_notation_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['user' => ['age' => 30, 'email' => 'john@example.com', 'name' => 'John Doe']], 201, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/profile', $handler, json_decode('{"properties":{"user":{"properties":{"age":{"minimum":0,"type":"integer"},"email":{"format":"email","type":"string"},"name":{"minLength":1,"type":"string"}},"required":["name","email"],"type":"object"}},"required":["user"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_15_special_characters_field_names_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['contact.email' => 'john@example.com', 'user-name' => 'JohnDoe'], 201, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode('{"properties":{"contact.email":{"format":"email","type":"string"},"user-name":{"type":"string"}},"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_16_minlength_validation_failure_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 2, 'min_length' => 3, 'value' => 'ab'], 'loc' => ['body', 'username'], 'msg' => 'String length must be at least 3', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode('{"properties":{"username":{"minLength":3,"type":"string"}},"required":["username"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_17_pattern_validation_failure_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^ACC-[0-9]{6}$', 'value' => 'INVALID123'], 'loc' => ['body', 'account_id'], 'msg' => 'String does not match pattern \'^ACC-[0-9]{6}$\'', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/accounts', $handler, json_decode('{"properties":{"account_id":{"pattern":"^ACC-[0-9]{6}$","type":"string"}},"required":["account_id"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_18_integer_minimum_validation_failure_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_value' => 0, 'minimum' => 1], 'loc' => ['body', 'quantity'], 'msg' => 'Value must be at least 1', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/products', $handler, json_decode('{"properties":{"quantity":{"minimum":1,"type":"integer"}},"required":["quantity"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_19_array_minitems_validation_failure_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_items' => 1, 'min_items' => 2], 'loc' => ['body', 'tags'], 'msg' => 'Array must contain at least 2 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/tags', $handler, json_decode('{"properties":{"tags":{"items":{"type":"string"},"minItems":2,"type":"array"}},"required":["tags"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_20_format_email_validation_failure_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'email', 'value' => 'not-an-email'], 'loc' => ['body', 'email'], 'msg' => 'Invalid email format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/subscribe', $handler, json_decode('{"properties":{"email":{"format":"email","type":"string"}},"required":["email"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_21_integer_type_coercion_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['value' => 'not-a-number'], 'loc' => ['body', 'price'], 'msg' => 'Value is not a valid integer', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/products', $handler, json_decode('{"properties":{"price":{"type":"integer"}},"required":["price"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_22_additional_properties_strict_failure_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['property' => 'unknown_field'], 'loc' => ['body', 'unknown_field'], 'msg' => 'Additional properties are not allowed', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/settings', $handler, json_decode('{"additionalProperties":false,"properties":{"theme":{"enum":["light","dark"],"type":"string"}},"required":["theme"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_boolean_field_conversion_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['subscribe' => true, 'username' => 'johndoe'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/', $handler, json_decode('{"properties":{"subscribe":{"type":"boolean"},"username":{"type":"string"}},"required":["username"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_empty_string_value_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['description' => '', 'username' => 'johndoe'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/', $handler, json_decode('{"properties":{"description":{"type":"string"},"username":{"type":"string"}},"required":["username"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/form/tags', $handler, json_decode('{"properties":{"tags":{"items":{"type":"string"},"type":"array"}},"required":["tags"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_numeric_field_type_conversion_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['age' => 30, 'username' => 'johndoe'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/', $handler, json_decode('{"properties":{"age":{"type":"integer"},"username":{"type":"string"}},"required":["username"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/token', $handler, json_decode('{"properties":{"grant_type":{"type":"string"},"password":{"type":"string"},"scope":{"type":"string"},"username":{"type":"string"}},"required":["username","password","grant_type"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_optional_field_missing_success_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['email' => null, 'username' => 'johndoe'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/register/', $handler, json_decode('{"properties":{"email":{"format":"email","type":["string","null"]},"password":{"type":"string"},"username":{"type":"string"}},"required":["username","password"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_pattern_validation_fail_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-z0-9_]+$'], 'input' => 'john doe', 'loc' => ['body', 'username'], 'msg' => 'String should match pattern \'^[a-z0-9_]+$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/validated', $handler, json_decode('{"properties":{"username":{"pattern":"^[a-z0-9_]+$","type":"string"}},"required":["username"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_required_field_missing_validation_error_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['body', 'username'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/login/', $handler, json_decode('{"properties":{"password":{"type":"string"},"username":{"type":"string"}},"required":["username","password"],"type":"object"}', true), null, null);
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
        };
        $app = $app->addRouteWithSchemas('POST', '/login/', $handler, json_decode('{"properties":{"password":{"type":"string"},"username":{"type":"string"}},"required":["username","password"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_special_characters_encoding_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['description' => 'Test & Development', 'name' => 'John Doe'], 200, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/', $handler, json_decode('{"properties":{"description":{"type":"string"},"name":{"type":"string"}},"required":["name"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_string_max_length_validation_fail_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 20], 'input' => 'this_is_a_very_long_username_that_exceeds_limit', 'loc' => ['body', 'username'], 'msg' => 'String should have at most 20 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/validated', $handler, json_decode('{"properties":{"username":{"maxLength":20,"type":"string"}},"required":["username"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_url_encoded_string_min_length_validation_fail_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['body', 'username'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/validated', $handler, json_decode('{"properties":{"username":{"minLength":3,"type":"string"}},"required":["username"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_09_multiple_validation_errors_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '3 validation errors in request', 'errors' => [['ctx' => ['ge' => 18], 'input' => 15, 'loc' => ['body', 'age'], 'msg' => 'Input should be greater than or equal to 18', 'type' => 'greater_than_equal'], ['ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'], 'input' => 'invalid-email', 'loc' => ['body', 'email'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'', 'type' => 'string_pattern_mismatch'], ['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode('{"properties":{"age":{"minimum":18,"type":"integer"},"email":{"format":"email","type":"string"},"name":{"minLength":3,"type":"string"}},"required":["name","email","age"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_10_nested_error_path_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'], 'input' => 'invalid', 'loc' => ['body', 'profile', 'contact', 'email'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/profiles', $handler, json_decode('{"properties":{"profile":{"properties":{"contact":{"properties":{"email":{"format":"email","type":"string"}},"required":["email"],"type":"object"}},"required":["contact"],"type":"object"}},"required":["profile"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_array_item_validation_error_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 123, 'loc' => ['body', 'tags', '2'], 'msg' => 'Input should be a valid unknown', 'type' => 'type_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"items":{"type":"string"},"type":"array"}},"required":["name","price","tags"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_array_max_items_constraint_violation_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => ['tag1', 'tag2', 'tag3', 'tag4', 'tag5', 'tag6', 'tag7', 'tag8', 'tag9', 'tag10', 'tag11'], 'loc' => ['body', 'tags'], 'msg' => '["tag1","tag2","tag3","tag4","tag5","tag6","tag7","tag8","tag9","tag10","tag11"] has more than 10 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"items":{"type":"string"},"maxItems":10,"type":"array"}},"required":["name","price","tags"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_array_min_items_constraint_violation_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => [], 'loc' => ['body', 'tags'], 'msg' => '[] has less than 1 item', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"tags":{"items":{},"minItems":1,"type":"array"}},"required":["name","price","tags"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_body_field_type_error_string_for_float_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not_a_float', 'loc' => ['body', 'price'], 'msg' => 'Input should be a valid number, unable to parse string as a number', 'type' => 'float_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_header_validation_error_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'x-token'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test', $handler, null, null, json_decode('{"headers":{"x-token":{"required":true,"type":"string"}},"query":{"q":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_validation_errors_invalid_uuid_format_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-uuid', 'loc' => ['path', 'item_id'], 'msg' => 'Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/not-a-uuid', $handler, null, null, json_decode('{"path":{"item_id":{"format":"uuid","type":"string"}}}', true));
        return $app;
    }

    public static function create_validation_errors_invalid_boolean_value_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'maybe', 'loc' => ['query', 'is_active'], 'msg' => 'Input should be a valid boolean, unable to interpret input', 'type' => 'bool_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test&is_active=maybe', $handler, null, null, json_decode('{"query":{"is_active":{"type":"boolean"},"q":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_validation_errors_invalid_datetime_format_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-datetime', 'loc' => ['body', 'created_at'], 'msg' => 'Input should be a valid datetime', 'type' => 'datetime_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"created_at":{"format":"date-time","type":"string"},"name":{"type":"string"},"price":{"type":"number"}},"required":["name","price","created_at"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_invalid_enum_value_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\''], 'input' => 'invalid_model', 'loc' => ['path', 'model_name'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/models/invalid_model', $handler, null, null, json_decode('{"path":{"model_name":{"enum":["alexnet","resnet","lenet"],"type":"string"}}}', true));
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
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"type":"string"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_missing_required_body_field_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => ['name' => 'Item'], 'loc' => ['body', 'price'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"string"}},"required":["name","price"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_missing_required_query_parameter_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'q'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, null, null, json_decode('{"query":{"q":{"required":true,"type":"string"}}}', true));
        return $app;
    }

    public static function create_validation_errors_multiple_validation_errors_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '3 validation errors in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'X', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short'], ['ctx' => ['gt' => 0], 'input' => -10, 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than 0', 'type' => 'greater_than'], ['input' => 'not_a_number', 'loc' => ['body', 'quantity'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"minLength":3,"type":"string"},"price":{"exclusiveMinimum":0,"type":"integer"},"quantity":{"type":"integer"}},"required":["name","price","quantity"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_nested_object_validation_error_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '3 validation errors in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'SF', 'loc' => ['body', 'seller', 'address', 'city'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short'], ['ctx' => ['min_length' => 5], 'input' => '123', 'loc' => ['body', 'seller', 'address', 'zip_code'], 'msg' => 'String should have at least 5 characters', 'type' => 'string_too_short'], ['ctx' => ['min_length' => 3], 'input' => 'Jo', 'loc' => ['body', 'seller', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode('{"additionalProperties":false,"properties":{"name":{"type":"string"},"price":{"type":"number"},"seller":{"additionalProperties":false,"properties":{"address":{"additionalProperties":false,"properties":{"city":{"minLength":3,"type":"string"},"zip_code":{"minLength":5,"type":"string"}},"required":["city","zip_code"],"type":"object"},"name":{"minLength":3,"type":"string"}},"required":["name","address"],"type":"object"}},"required":["name","price","seller"],"type":"object"}', true), null, null);
        return $app;
    }

    public static function create_validation_errors_numeric_constraint_violation_gt_greater_than_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['gt' => 0], 'input' => '0', 'loc' => ['query', 'price'], 'msg' => 'Input should be greater than 0', 'type' => 'greater_than']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test&price=0', $handler, null, null, json_decode('{"query":{"price":{"exclusiveMinimum":0,"type":"number"},"q":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_validation_errors_numeric_constraint_violation_le_less_than_or_equal_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['le' => 100], 'input' => '101', 'loc' => ['query', 'limit'], 'msg' => 'Input should be less than or equal to 100', 'type' => 'less_than_equal']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test&limit=101', $handler, null, null, json_decode('{"query":{"limit":{"maximum":100,"type":"integer"},"q":{"type":"string"}}}', true));
        return $app;
    }

    public static function create_validation_errors_query_param_type_error_string_provided_for_int_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['input' => 'not_a_number', 'loc' => ['query', 'skip'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test&skip=not_a_number', $handler, null, null, json_decode('{"query":{"q":{"type":"string"},"skip":{"type":"integer"}}}', true));
        return $app;
    }

    public static function create_validation_errors_string_max_length_constraint_violation_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 50], 'input' => 'this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', 'loc' => ['query', 'q'], 'msg' => 'String should have at most 50 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', $handler, null, null, json_decode('{"query":{"q":{"maxLength":50,"type":"string"}}}', true));
        return $app;
    }

    public static function create_validation_errors_string_min_length_constraint_violation_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['query', 'q'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=ab', $handler, null, null, json_decode('{"query":{"q":{"minLength":3,"type":"string"}}}', true));
        return $app;
    }

    public static function create_validation_errors_string_regex_pattern_mismatch_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response {
                return new Response(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9_-]+$'], 'input' => 'invalid!', 'loc' => ['query', 'q'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_-]+$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], 422, []);
            }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=invalid!', $handler, null, null, json_decode('{"query":{"q":{"pattern":"^[a-zA-Z0-9_-]+$","type":"string"}}}', true));
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
            yield 'data: ' . json_encode($event) . "\n\n";
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
            yield 'data: ' . json_encode($event) . "\n\n";
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
            yield 'data: ' . json_encode($event) . "\n\n";
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
            yield 'data: ' . json_encode($event) . "\n\n";
        }
    }
}

final class WebSocketHandler_1 implements WebSocketHandlerInterface
{
    private array $messages = [['level' => 'example_level', 'message' => 'example_message', 'source' => 'example_source', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'system_alert']];
    private int $messageIndex = 0;

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
    private array $messages = [['text' => 'Hello, everyone!', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'message', 'user' => 'alice']];
    private int $messageIndex = 0;

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
    private array $messages = [['messageId' => 'ack-123', 'status' => 'delivered', 'timestamp' => '2024-01-15T10:31:00Z', 'type' => 'chatAck']];
    private int $messageIndex = 0;

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
    private array $messages = [['timestamp' => '2024-01-15T10:35:00Z', 'type' => 'userLeft', 'user' => 'charlie']];
    private int $messageIndex = 0;

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
    private array $messages = [['timestamp' => '2024-01-15T10:29:55Z', 'type' => 'userJoined', 'user' => 'bob']];
    private int $messageIndex = 0;

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
    private array $messages = [['body' => 'example_body', 'priority' => 'example_priority', 'timestamp' => '2024-01-15T10:30:00Z', 'title' => 'example_title', 'type' => 'user_notification', 'userId' => 'example_userId']];
    private int $messageIndex = 0;

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
    private array $messages = [['message' => 'example_message', 'metadata' => [], 'service' => 'example_service', 'status' => 'example_status', 'timestamp' => '2024-01-15T10:30:00Z', 'type' => 'status_update']];
    private int $messageIndex = 0;

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
