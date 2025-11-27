<?php

declare(strict_types=1);

namespace E2E\Php;

use Spikard\App;
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
    public static function create_sse_systemAlert_1(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_1());
        return $app;
    }

    public static function create_sse_notificationBatch_2(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_2());
        return $app;
    }

    public static function create_sse_userNotification_3(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_3());
        return $app;
    }

    public static function create_sse_statusUpdate_4(): App
    {
        $app = new App();
        $app = $app->addSse('/notifications', new SseProducer_4());
        return $app;
    }

    public static function create_websocket_systemAlert_1(): App
    {
        $app = new App();
        $app = $app->addWebSocket('systemAlert', new WebSocketHandler_1());
        return $app;
    }

    public static function create_websocket_chatMessage_2(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chat', new WebSocketHandler_2());
        return $app;
    }

    public static function create_websocket_chatAck_3(): App
    {
        $app = new App();
        $app = $app->addWebSocket('chatAck', new WebSocketHandler_3());
        return $app;
    }

    public static function create_websocket_userLeft_4(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chat', new WebSocketHandler_4());
        return $app;
    }

    public static function create_websocket_userJoined_5(): App
    {
        $app = new App();
        $app = $app->addWebSocket('/chat', new WebSocketHandler_5());
        return $app;
    }

    public static function create_websocket_userNotification_6(): App
    {
        $app = new App();
        $app = $app->addWebSocket('userNotification', new WebSocketHandler_6());
        return $app;
    }

    public static function create_websocket_statusUpdate_7(): App
    {
        $app = new App();
        $app = $app->addWebSocket('statusUpdate', new WebSocketHandler_7());
        return $app;
    }

    public static function create_auth_API_key_authentication___invalid_key_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_API_key_authentication___missing_header_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_API_key_authentication___valid_key_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_API_key_in_query_parameter_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data?api_key=sk_test_123456', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_API_key_rotation___old_key_still_valid_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_API_key_with_custom_header_name_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_Bearer_token_without_prefix_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_authentication___expired_token_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_authentication___invalid_audience_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_authentication___invalid_signature_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_authentication___missing_Authorization_header_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_authentication___valid_token_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected/user', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_invalid_issuer_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_malformed_token_format_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_missing_required_custom_claims_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/admin', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_not_before_claim_in_future_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_JWT_with_multiple_audiences_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_auth_Multiple_authentication_schemes___JWT_precedence_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_background_Background_event_logging_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/background/events', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_background_Background_event_logging___second_payload_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/background/events', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_body_limits_Body_over_limit_returns_413_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/body-limit/over', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_body_limits_Body_under_limit_succeeds_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/body-limit/under', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_compression_Compression___gzip_applied_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/compression/gzip', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_compression_Compression___payload_below_min_size_is_not_compressed_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/compression/skip', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_13_json_with_charset_utf16_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_14_content_type_case_insensitive_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_15_multipart_boundary_required_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_16_text_plain_not_accepted_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_17_vendor_json_accepted_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/v1/resource', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_18_content_type_with_multiple_params_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_19_missing_content_type_default_json_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_20_content_length_mismatch_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_415_Unsupported_Media_Type_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_Binary_response___application_octet_stream_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/download/file.bin', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_CSV_response___text_csv_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/export/data.csv', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_Content_negotiation___Accept_header_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/accept-test/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_HTML_response___text_html_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/html', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_JPEG_image_response___image_jpeg_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/images/photo.jpg', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_JSON_response___application_json_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/json', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_JSON_with_UTF_8_charset_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/unicode', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_PDF_response___application_pdf_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/download/document.pdf', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_PNG_image_response___image_png_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/images/logo.png', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_Plain_text_response___text_plain_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/text', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_content_types_XML_response___application_xml_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/xml', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_24_cookie_samesite_strict_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/secure', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_25_cookie_samesite_lax_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_26_cookie_secure_flag_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/secure', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_27_cookie_httponly_flag_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/secure', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_APIKey_cookie_authentication___missing_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me/auth', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_APIKey_cookie_authentication___success_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Cookie_regex_pattern_validation___fail_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/cookies/pattern', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Cookie_regex_pattern_validation___success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/cookies/pattern', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Cookie_validation___max_length_constraint_fail_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/cookies/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Cookie_validation___min_length_constraint_success_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/cookies/min-length', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Cookie_validation___min_length_failure_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Multiple_cookies___success_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Optional_APIKey_cookie___missing_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Optional_cookie_parameter___missing_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Optional_cookie_parameter___success_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Required_cookie___missing_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/cookies', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response___delete_cookie_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/delete', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response___multiple_cookies_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/multiple', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response___session_cookie__no_max_age__19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/session', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response_cookie_with_SameSite_Lax_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/samesite-lax', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response_cookie_with_SameSite_None_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/samesite-none', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response_cookie_with_SameSite_Strict_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/samesite-strict', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response_cookie_with_attributes_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/cookie/set', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response_cookie_with_domain_attribute_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/set-with-domain', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response_cookie_with_path_attribute_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/cookies/set-with-path', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cookies_Response_set_cookie___basic_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/cookie/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_06_cors_preflight_method_not_allowed_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_07_cors_preflight_header_not_allowed_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_08_cors_max_age_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_09_cors_expose_headers_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_10_cors_origin_null_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_Private_Network_Access_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/local-resource', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_Vary_header_for_proper_caching_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/cached-resource', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_multiple_allowed_origins_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_origin_case_sensitivity_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_preflight_for_DELETE_method_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/resource/456', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_preflight_for_PUT_method_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/api/resource/123', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_preflight_request_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_regex_pattern_matching_for_origins_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_request_blocked_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_safelisted_headers_without_preflight_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/form', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_wildcard_origin_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/public/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_CORS_with_credentials_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/user/profile', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_cors_Simple_CORS_request_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Async_factory_dependency___success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/db-status', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Circular_dependency_detection___error_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/circular', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Dependency_injection_in_lifecycle_hooks___success_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/hook-di-test', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Factory_dependency___success_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/timestamp', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Missing_dependency___error_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/missing-dep', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Mixed_singleton_and_per_request_caching___success_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/mixed-caching', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Multiple_dependencies_with_cleanup___success_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/multi-cleanup-test', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Nested_dependencies__3_levels____success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/auth-status', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Node_js_object_destructuring_injection___success_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/node-destructure', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Per_request_dependency_caching___success_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/request-id', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Python_parameter_name_based_injection___success_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/python-name-inject', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Python_type_annotation_based_injection___success_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/python-type-inject', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Resource_cleanup_after_request___success_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/cleanup-test', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Route_level_dependency_override___success_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/override-test', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Ruby_keyword_argument_injection___success_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/ruby-kwargs', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Singleton_dependency_caching___success_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/app-counter', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Type_mismatch_in_dependency_resolution___error_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/type-mismatch', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_di_Value_dependency_injection___success_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/config', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_11_utf8_query_parameter_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/search', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_12_percent_encoded_special_chars_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/search?term=hi%20there', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_13_empty_string_query_param_preserved_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items?filter=', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_14_large_integer_boundary_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_15_float_precision_preservation_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/calculate', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_16_negative_zero_handling_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_17_extremely_long_string_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/text', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_18_unicode_normalization_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_19_emoji_in_strings_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/messages', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_20_null_byte_in_string_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_21_scientific_notation_number_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/calculate', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_22_leading_zeros_integer_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_23_deeply_nested_json_limit_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_24_array_with_holes_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_Deeply_nested_structure__10__levels__15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/nested/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_Empty_and_null_value_handling_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/nulls/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_Float_precision_and_rounding_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/calculations/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_Large_integer_boundary_values_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/numbers/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_Special_string_values_and_escaping_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/strings/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_edge_cases_Unicode_and_emoji_handling_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_30_bearer_token_format_valid_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_31_bearer_token_format_invalid_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_32_bearer_token_missing_prefix_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/protected', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_33_api_key_header_valid_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_34_api_key_header_invalid_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Accept_header___JSON_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/accept', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Accept_Encoding_header_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/accept-encoding', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Accept_Language_header_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/accept-language', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Authorization_header___missing_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Authorization_header___success_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Authorization_header___wrong_scheme_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Basic_authentication___success_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/basic-auth', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Bearer_token_authentication___missing_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/bearer-auth', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Bearer_token_authentication___success_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/bearer-auth', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Content_Type_header___application_json_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/content-type', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Header_case_insensitivity___access_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/echo', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Header_regex_validation___fail_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/pattern', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Header_regex_validation___success_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/pattern', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Header_validation___max_length_constraint_fail_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/max-length', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Header_validation___min_length_constraint_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Header_with_underscore_conversion___explicit_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/underscore', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Host_header_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/host', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Multiple_custom_headers_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/multiple', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Multiple_header_values___X_Token_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Optional_header_with_None_default___missing_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Origin_header_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/origin', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_Referer_header_27(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/headers/referer', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_User_Agent_header___custom_value_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_User_Agent_header___default_value_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_X_API_Key_optional_header___missing_30(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_X_API_Key_optional_header___success_31(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_X_API_Key_required_header___missing_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_headers_X_API_Key_required_header___success_33(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_DELETE___Remove_resource_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('DELETE', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_DELETE___Resource_not_found_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('DELETE', '/items/999', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_DELETE___With_response_body_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('DELETE', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_HEAD___Get_metadata_without_body_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('HEAD', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_OPTIONS___CORS_preflight_request_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('OPTIONS', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_PATCH___Partial_update_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('PATCH', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_PATCH___Update_multiple_fields_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('PATCH', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_PUT___Complete_resource_replacement_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_PUT___Create_resource_if_doesn_t_exist_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/999', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_PUT___Idempotent_operation_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_PUT___Missing_required_field_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_http_methods_PUT___Validation_error_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('PUT', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_29_nested_object_validation_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_30_nested_object_missing_field_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_31_nullable_property_null_value_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_32_schema_ref_definitions_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/products', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_33_allof_schema_composition_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_34_additional_properties_false_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_35_oneof_schema_success_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/payment', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_36_oneof_schema_multiple_match_failure_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/payment', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_37_oneof_schema_no_match_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/payment', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_38_anyof_schema_success_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/contact', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_39_anyof_schema_multiple_match_success_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/contact', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_40_anyof_schema_failure_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/contact', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_41_not_schema_success_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_42_not_schema_failure_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_43_const_validation_success_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/v1/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_44_const_validation_failure_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/v1/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_45_minproperties_validation_success_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/config', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_46_minproperties_validation_failure_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/config', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_47_maxproperties_validation_failure_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/config', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_48_dependencies_validation_success_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/billing', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_49_dependencies_validation_failure_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/billing', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_50_deep_nesting_4_levels_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Array_of_objects___success_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/list', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Array_of_primitive_values_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Body_with_query_parameters_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/?limit=10', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Boolean_field___success_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Date_field___success_27(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/events/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Datetime_field___success_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/events/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Deeply_nested_objects_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/nested', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Empty_JSON_object_30(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/optional-all', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Empty_array_validation___fail_31(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/list-validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Enum_field___invalid_value_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Enum_field___success_33(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Extra_fields_ignored__no_additionalProperties__34(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Field_type_validation___invalid_type_35(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Nested_object___success_36(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/nested', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Null_value_for_optional_field_37(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Numeric_ge_validation___fail_38(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Numeric_le_validation___success_39(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Optional_fields___omitted_40(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_PATCH_partial_update_41(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('PATCH', '/items/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Required_field_missing___validation_error_42(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_Simple_JSON_object___success_43(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_String_max_length_validation___fail_44(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_String_min_length_validation___fail_45(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_String_pattern_validation___fail_46(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_String_pattern_validation___success_47(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_UUID_field___invalid_format_48(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_json_bodies_UUID_field___success_49(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_Hook_Execution_Order_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-hook-order', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_Multiple_Hooks___All_Phases_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/full-lifecycle', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_onError___Error_Logging_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-error', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_onRequest___Request_Logging_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-on-request', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_onResponse___Response_Timing_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-timing', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_onResponse___Security_Headers_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/test-security-headers', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_preHandler___Authentication_Failed__Short_Circuit__7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected-resource-fail', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_preHandler___Authentication_Success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/protected-resource', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_preHandler___Authorization_Check_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/admin-only', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_preHandler___Authorization_Forbidden__Short_Circuit__10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/admin-only-forbidden', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_preValidation___Rate_Limit_Exceeded__Short_Circuit__11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/test-rate-limit-exceeded', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_lifecycle_hooks_preValidation___Rate_Limiting_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/api/test-rate-limit', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_17_file_magic_number_png_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_18_file_magic_number_jpeg_success_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_19_file_mime_spoofing_png_as_jpeg_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_20_file_mime_spoofing_jpeg_as_png_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_21_file_pdf_magic_number_success_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_22_file_empty_buffer_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Content_Type_validation___invalid_type_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/images-only', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Empty_file_upload_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/upload', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_File_list_upload__array_of_files__9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/list', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_File_size_validation___too_large_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_File_upload_with_custom_headers_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_File_upload_without_filename_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Form_data_without_files_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Image_file_upload_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/image', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Mixed_files_and_form_data_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Multiple_file_uploads_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Multiple_values_for_same_field_name_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Optional_file_upload___missing_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/optional', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Optional_file_upload___provided_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/optional', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_PDF_file_upload_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/document', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Required_file_upload___missing_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/files/required', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_multipart_Simple_file_upload_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_20_uuid_v3_path_param_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/e8b5a51d-11c8-3310-a6ab-367563f20686', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_21_uuid_v5_path_param_success_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/630eb68f-e0fa-5ecc-887a-7c7a62614681', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_24_date_format_path_param_success_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/events/2025-10-30', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_25_date_format_invalid_failure_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/events/2025-13-45', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_27_datetime_format_path_param_success_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/bookings/2025-10-30T14:30:00Z', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_28_duration_format_path_param_success_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/delays/P1DT2H30M', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_29_decimal_path_param_success_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/prices/19.99', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_30_string_minlength_path_success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/alice', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_31_string_minlength_path_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/ab', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_32_string_maxlength_path_failure_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/this_username_is_way_too_long_to_be_valid', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_33_string_pattern_path_success_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/repos/spikard-labs/spikard-http', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_34_string_pattern_path_failure_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/repos/invalid@owner', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_35_negative_integer_path_param_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/offset/-100', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Boolean_path_parameter___True_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/bool/True', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Boolean_path_parameter___numeric_1_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/bool/1', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Date_path_parameter___success_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/date/2023-07-15', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Enum_path_parameter___invalid_value_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/models/foo', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Enum_path_parameter___success_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/models/alexnet', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Float_path_parameter___success_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/float/42.5', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Integer_path_parameter___invalid_string_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/int/foobar', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Integer_path_parameter___success_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/int/42', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Integer_path_parameter_with_combined_lt_and_gt_constraints___success_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-lt-gt/2', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Integer_path_parameter_with_ge_constraint___success_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-ge/3', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Integer_path_parameter_with_gt_constraint___failure_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-gt/2', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Integer_path_parameter_with_gt_constraint___success_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-gt/42', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Integer_path_parameter_with_le_constraint___success_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-le/3', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Integer_path_parameter_with_lt_constraint___success_27(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-lt/2', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Multiple_path_parameters___success_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Path_parameter_type_syntax___invalid_UUID_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/type-syntax/items/not-a-uuid', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Path_parameter_type_syntax_with_override_30(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/type-syntax/items-count/50', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Path_parameter_with_type_syntax___UUID_31(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/type-syntax/items/550e8400-e29b-41d4-a716-446655440000', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Path_parameter_with_type_syntax___integer_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/type-syntax/users/42', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_Path_type_parameter___file_path_33(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/files/home/johndoe/myfile.txt', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_String_path_parameter___success_34(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/str/foobar', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_String_path_parameter_with_max_length___failure_35(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-maxlength/foobar', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_String_path_parameter_with_min_length___failure_36(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/path/param-minlength/fo', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_path_params_UUID_path_parameter___success_37(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_42_negative_integer_query_param_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/negative', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_43_scientific_notation_float_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/stats', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_44_string_minlength_validation_success_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/search', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_45_string_minlength_validation_failure_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/search', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_46_string_maxlength_validation_failure_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/search', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_47_pattern_validation_email_success_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/subscribe', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_48_pattern_validation_email_failure_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/subscribe', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_49_integer_gt_constraint_success_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_50_integer_gt_constraint_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_51_integer_ge_constraint_boundary_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_52_integer_le_constraint_boundary_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_53_integer_le_constraint_failure_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_54_array_minitems_constraint_success_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_55_array_minitems_constraint_failure_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_56_array_maxitems_constraint_failure_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_57_boolean_empty_string_coercion_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_58_format_email_success_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/subscribe', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_59_format_email_failure_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/subscribe', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_60_format_ipv4_success_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/network', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_61_format_ipv4_failure_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/network', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_62_format_ipv6_success_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/network/ipv6', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_63_format_uri_success_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/redirect', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_64_format_uri_failure_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/redirect', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_65_format_hostname_success_24(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/dns', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_66_multipleof_constraint_success_25(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_67_multipleof_constraint_failure_26(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_68_array_uniqueitems_success_27(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_69_array_uniqueitems_failure_28(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_70_array_separator_pipe_29(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items?tags=python|rust|typescript', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_71_array_separator_semicolon_30(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items?colors=red;green;blue', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_72_array_separator_space_31(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/search?keywords=rust%20web%20framework', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Array_query_parameter___empty_array_32(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list-default', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Array_query_parameter___single_value_33(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list-default', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Boolean_query_parameter___numeric_1_34(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/bool', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Boolean_query_parameter___true_35(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/bool', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Date_query_parameter___success_36(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/date', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Datetime_query_parameter___success_37(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/datetime', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Enum_query_parameter___invalid_value_38(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/enum', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Enum_query_parameter___success_39(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/enum', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Float_query_param_with_ge_constraint___success_40(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/float-ge', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Integer_query_param_with_ge_constraint___boundary_41(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int-ge', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Integer_query_param_with_gt_constraint___valid_42(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int-gt', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Integer_query_param_with_le_constraint___boundary_43(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int-le', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Integer_query_param_with_lt_constraint___valid_44(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int-lt', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Integer_with_default_value___not_provided_45(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int/default', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Integer_with_default_value___override_46(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int/default', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_List_of_integers___multiple_values_47(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_List_of_strings___multiple_values_48(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_List_query_parameter___required_but_missing_49(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_List_with_default_empty_array___no_values_provided_50(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/list-default', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Multiple_query_parameters_with_different_types_51(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/multi-type', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Optional_integer_query_parameter___missing_52(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int/optional', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Optional_query_parameter_with_default_value_53(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/optional-default', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Optional_string_query_parameter___missing_54(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/optional', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Optional_string_query_parameter___provided_55(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/optional', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Query_parameter_with_URL_encoded_space_56(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/basic', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Query_parameter_with_URL_encoded_special_characters_57(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/basic', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Query_parameter_with_special_characters___URL_encoding_58(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/test', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Required_integer_query_parameter___float_value_59(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Required_integer_query_parameter___invalid_type_60(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Required_integer_query_parameter___missing_61(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Required_integer_query_parameter___success_62(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/int', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Required_string_query_parameter___missing_63(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_Required_string_query_parameter___success_64(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_String_query_param_with_max_length_constraint___fail_65(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/str-max-length', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_String_query_param_with_min_length_constraint___fail_66(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/str-min-length', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_String_query_param_with_regex_pattern___fail_67(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/pattern', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_String_validation_with_regex___failure_68(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_String_validation_with_regex___success_69(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_UUID_query_parameter___invalid_format_70(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/uuid', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_query_params_UUID_query_parameter___success_71(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/query/uuid', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_rate_limit_Rate_limit_below_threshold_succeeds_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/rate-limit/basic', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_rate_limit_Rate_limit_exceeded_returns_429_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/rate-limit/exceeded', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_request_id_Request_ID_header_is_preserved_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/request-id/preserved', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_request_id_Request_ID_is_generated_when_not_provided_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/request-id/generated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_request_id_Request_ID_middleware_can_be_disabled_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/request-id/disabled', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_request_timeout_Request_completes_before_timeout_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/timeouts/fast', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_request_timeout_Request_exceeds_timeout_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/timeouts/slow', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_static_files_Static_file_server_returns_text_file_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/public/hello.txt', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_static_files_Static_server_returns_index_html_for_directory_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/app/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_19_413_payload_too_large_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/upload', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_200_OK___Success_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/status-test/200', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_201_Created___Resource_created_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_202_Accepted___Request_accepted_for_processing_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/tasks/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_204_No_Content___Success_with_no_body_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('DELETE', '/status-test/204', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_206_Partial_Content_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/files/document.pdf', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_20_414_uri_too_long_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/data?skip_template_expansion=true', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_21_431_request_header_fields_too_large_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_22_501_not_implemented_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('TRACE', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_23_503_service_unavailable_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_301_Moved_Permanently___Permanent_redirect_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/old-path', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_302_Found___Temporary_redirect_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/temp-redirect', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_304_Not_Modified___Cached_content_valid_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/status-test/304', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_307_Temporary_Redirect___Method_preserved_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/redirect-post', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_400_Bad_Request___Invalid_request_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_401_Unauthorized___Missing_authentication_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/users/me', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_403_Forbidden___Insufficient_permissions_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/admin/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_404_Not_Found___Resource_not_found_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/status-test/404', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_408_Request_Timeout_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/slow-endpoint', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_422_Unprocessable_Entity___Validation_error_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_429_Too_Many_Requests_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/api/resource', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_500_Internal_Server_Error___Server_error_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/error', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_status_codes_503_Service_Unavailable___Server_overload_23(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/health', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_streaming_Binary_log_download_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/stream/logfile', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_streaming_Chunked_CSV_export_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/stream/csv-report', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_streaming_Stream_JSON_lines_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/stream/json-lines', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_13_array_field_success_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/register', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_14_nested_object_bracket_notation_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/profile', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_15_special_characters_field_names_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/data', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_16_minlength_validation_failure_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_17_pattern_validation_failure_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/accounts', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_18_integer_minimum_validation_failure_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/products', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_19_array_minitems_validation_failure_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/tags', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_20_format_email_validation_failure_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/subscribe', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_21_integer_type_coercion_failure_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/products', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_22_additional_properties_strict_failure_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/settings', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_Boolean_field_conversion_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_Empty_string_value_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_Multiple_values_for_same_field_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/tags', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_Numeric_field_type_conversion_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_OAuth2_password_grant_flow_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/token', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_Optional_field_missing___success_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/register/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_Pattern_validation___fail_17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_Required_field_missing___validation_error_18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/login/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_Simple_form_submission___success_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/login/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_Special_characters_encoding_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_String_max_length_validation___fail_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_url_encoded_String_min_length_validation___fail_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/form/validated', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_09_multiple_validation_errors_1(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/users', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_10_nested_error_path_2(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/profiles', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Array_item_validation_error_3(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Array_max_items_constraint_violation_4(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Array_min_items_constraint_violation_5(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Body_field_type_error___string_for_float_6(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Header_validation_error_7(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Invalid_UUID_format_8(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/not-a-uuid', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Invalid_boolean_value_9(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test&is_active=maybe', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Invalid_datetime_format_10(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Invalid_enum_value_11(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/models/invalid_model', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Malformed_JSON_body_12(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Missing_required_body_field_13(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Missing_required_query_parameter_14(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Multiple_validation_errors_15(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Nested_object_validation_error_16(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('POST', '/items/', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Numeric_constraint_violation___gt__greater_than__17(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test&price=0', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Numeric_constraint_violation___le__less_than_or_equal__18(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test&limit=101', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_Query_param_type_error___string_provided_for_int_19(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=test&skip=not_a_number', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_String_max_length_constraint_violation_20(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_String_min_length_constraint_violation_21(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=ab', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
        return $app;
    }

    public static function create_validation_errors_String_regex_pattern_mismatch_22(): App
    {
        $app = new App();
        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool { return true; }
            public function handle(Request $request): Response { return new Response([], 200); }
        };
        $app = $app->addRouteWithSchemas('GET', '/items/?q=invalid!', $handler, json_decode(null, true), json_decode(null, true), json_decode(null, true));
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
