<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;
use E2E\Php\AppFactory;
use function Spikard\Tests\normalize_validation_errors;

/**
 * Generated from testing_data fixtures.
 * @phpstan-type ResponseBody array<string, mixed>|string|int|float|bool|null
 */
final class GeneratedTest extends TestCase
{
    public function test_auth_api_key_authentication_invalid_key(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["X-API-Key" => "invalid_key_12345"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "Invalid API key", "status" => 401, "detail" => "The provided API key is not valid"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_api_key_authentication_missing_header(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_missing_header_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "Missing API key", "status" => 401, "detail" => "Expected 'X-API-Key' header or 'api_key' query parameter with valid API key"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_api_key_authentication_valid_key(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["X-API-Key" => "sk_test_123456"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Access granted", "data" => "sensitive information"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_api_key_in_query_parameter(): void
    {
        $app = AppFactory::create_auth_api_key_in_query_parameter_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data?api_key=sk_test_123456', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Access granted", "data" => "sensitive information"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_api_key_rotation_old_key_still_valid(): void
    {
        $app = AppFactory::create_auth_api_key_rotation_old_key_still_valid_5();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["X-API-Key" => "sk_test_old_123456"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Access granted", "data" => "sensitive information"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_api_key_with_custom_header_name(): void
    {
        $app = AppFactory::create_auth_api_key_with_custom_header_name_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["X-API-Token" => "sk_test_123456"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Access granted", "data" => "sensitive information"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_bearer_token_without_prefix(): void
    {
        $app = AppFactory::create_auth_bearer_token_without_prefix_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ["Authorization" => "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDZ9.8yXqZ9jKCR0BwqJc7pN_QvD3mYLxHfWzUeIaGkTnOsA"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "Invalid Authorization header format", "status" => 401, "detail" => "Authorization header must use Bearer scheme: 'Bearer <token>'"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_authentication_expired_token(): void
    {
        $app = AppFactory::create_auth_jwt_authentication_expired_token_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', ['headers' => ["Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNjAwMDAwMDAwLCJpYXQiOjE1OTAwMDAwMDB9.n4oBw9XuO2aAJWi1e4Bz9Y_m2iEyJHGAODcetNuwYFo"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "JWT validation failed", "status" => 401, "detail" => "Token has expired"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_authentication_invalid_audience(): void
    {
        $app = AppFactory::create_auth_jwt_authentication_invalid_audience_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', ['headers' => ["Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTk5LCJpYXQiOjE3MzEyNTIwMDAsImF1ZCI6WyJodHRwczovL3dyb25nLXNlcnZpY2UuY29tIl19.YR2a9fSJjhen7ksYFI2djSBSC7Pc29FDCloBGhkj3kU"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "JWT validation failed", "status" => 401, "detail" => "Token audience is invalid"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_authentication_invalid_signature(): void
    {
        $app = AppFactory::create_auth_jwt_authentication_invalid_signature_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', ['headers' => ["Authorization" => "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMH0.invalid_signature_here"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "JWT validation failed", "status" => 401, "detail" => "Token signature is invalid"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_authentication_missing_authorization_header(): void
    {
        $app = AppFactory::create_auth_jwt_authentication_missing_authorization_header_11();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "Missing or invalid Authorization header", "status" => 401, "detail" => "Expected 'Authorization: Bearer <token>'"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_authentication_valid_token(): void
    {
        $app = AppFactory::create_auth_jwt_authentication_valid_token_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', ['headers' => ["Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Access granted", "user_id" => "user123"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_invalid_issuer(): void
    {
        $app = AppFactory::create_auth_jwt_invalid_issuer_13();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ["Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2V2aWwuY29tIn0.mbL5L04_hpaaiz0SPABap6ZWfBLu18aiexBjzwQ1nnA"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "JWT validation failed", "status" => 401, "detail" => "Token issuer is invalid, expected 'https://auth.example.com'"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_malformed_token_format(): void
    {
        $app = AppFactory::create_auth_jwt_malformed_token_format_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ["Authorization" => "Bearer invalid.token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "Malformed JWT token", "status" => 401, "detail" => "Malformed JWT token: expected 3 parts separated by dots, found 2"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_missing_required_custom_claims(): void
    {
        $app = AppFactory::create_auth_jwt_missing_required_custom_claims_15();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/admin', ['headers' => ["Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(403, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/forbidden", "title" => "Forbidden", "status" => 403, "detail" => "Required claims 'role' and 'permissions' missing from JWT"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_not_before_claim_in_future(): void
    {
        $app = AppFactory::create_auth_jwt_not_before_claim_in_future_16();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ["Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsIm5iZiI6MjYyNjc4Mzk0NiwiYXVkIjpbImh0dHBzOi8vYXBpLmV4YW1wbGUuY29tIl0sImlzcyI6Imh0dHBzOi8vYXV0aC5leGFtcGxlLmNvbSJ9.hG4I76_3kJfsbJ_jmxoP1NSYnkcqdyBFcPpdo-jYU4E"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unauthorized", "title" => "JWT validation failed", "status" => 401, "detail" => "JWT not valid yet, not before claim is in the future"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_jwt_with_multiple_audiences(): void
    {
        $app = AppFactory::create_auth_jwt_with_multiple_audiences_17();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ["Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSIsImh0dHBzOi8vYWRtaW4uZXhhbXBsZS5jb20iXSwiaXNzIjoiaHR0cHM6Ly9hdXRoLmV4YW1wbGUuY29tIn0.9MBL_XccGXfu9cDUnCpQruDMOl2hHYydzeGn-20dQOs"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Access granted", "user_id" => "user123"];
        $this->assertEquals($expected, $body);
    }

    public function test_auth_multiple_authentication_schemes_jwt_precedence(): void
    {
        $app = AppFactory::create_auth_multiple_authentication_schemes_jwt_precedence_18();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["X-API-Key" => "sk_test_123456", "Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Access granted", "user_id" => "user123", "auth_method" => "jwt"];
        $this->assertEquals($expected, $body);
    }

    public function test_background_background_event_logging(): void
    {
        $app = AppFactory::create_background_background_event_logging_1();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/background/events', ['body' => ["event" => "alpha"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(202, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_background_background_event_logging_second_payload(): void
    {
        $app = AppFactory::create_background_background_event_logging_second_payload_2();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/background/events', ['body' => ["event" => "beta"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(202, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_body_limits_body_over_limit_returns_413(): void
    {
        $app = AppFactory::create_body_limits_body_over_limit_returns_413_1();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/body-limit/over', ['body' => ["note" => "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(413, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_body_limits_body_under_limit_succeeds(): void
    {
        $app = AppFactory::create_body_limits_body_under_limit_succeeds_2();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/body-limit/under', ['body' => ["note" => "small"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["accepted" => true, "note" => "small"];
        $this->assertEquals($expected, $body);
    }

    public function test_compression_compression_gzip_applied(): void
    {
        $app = AppFactory::create_compression_compression_gzip_applied_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/compression/gzip', ['headers' => ["Accept-Encoding" => "gzip"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Compressed payload", "payload" => "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"];
        $this->assertEquals($expected, $body);
    }

    public function test_compression_compression_payload_below_min_size_is_not_compressed(): void
    {
        $app = AppFactory::create_compression_compression_payload_below_min_size_is_not_compressed_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/compression/skip', ['headers' => ["Accept-Encoding" => "gzip"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Small payload", "payload" => "tiny"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_13_json_with_charset_utf16(): void
    {
        $app = AppFactory::create_content_types_13_json_with_charset_utf16_1();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ["Content-Type" => "application/json; charset=utf-16"], 'body' => ["value" => "test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(415, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unsupported-charset", "title" => "Unsupported Charset", "status" => 415, "detail" => "Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported."];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_14_content_type_case_insensitive(): void
    {
        $app = AppFactory::create_content_types_14_content_type_case_insensitive_2();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ["Content-Type" => "APPLICATION/JSON"], 'body' => ["name" => "test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "test"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_15_multipart_boundary_required(): void
    {
        $app = AppFactory::create_content_types_15_multipart_boundary_required_3();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['headers' => ["Content-Type" => "multipart/form-data"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(400, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "multipart/form-data requires 'boundary' parameter"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_16_text_plain_not_accepted(): void
    {
        $app = AppFactory::create_content_types_16_text_plain_not_accepted_4();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ["Content-Type" => "text/plain"], 'body' => "{\"data\": \"value\"}"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(415, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unsupported-media-type", "title" => "Unsupported Media Type", "status" => 415, "detail" => "Unsupported media type"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_17_vendor_json_accepted(): void
    {
        $app = AppFactory::create_content_types_17_vendor_json_accepted_5();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/v1/resource', ['headers' => ["Content-Type" => "application/vnd.api+json"], 'body' => ["data" => "value"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["data" => "value"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_18_content_type_with_multiple_params(): void
    {
        $app = AppFactory::create_content_types_18_content_type_with_multiple_params_6();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ["Content-Type" => "application/json; charset=utf-8; boundary=something"], 'body' => ["value" => "test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["value" => "test"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_19_missing_content_type_default_json(): void
    {
        $app = AppFactory::create_content_types_19_missing_content_type_default_json_7();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['body' => ["name" => "test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "test"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_20_content_length_mismatch(): void
    {
        $app = AppFactory::create_content_types_20_content_length_mismatch_8();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ["Content-Type" => "application/json", "Content-Length" => "100"], 'body' => ["value" => "short"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(400, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Content-Length header does not match actual body size"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_415_unsupported_media_type(): void
    {
        $app = AppFactory::create_content_types_415_unsupported_media_type_9();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/xml"], 'body' => "<?xml version=\"1.0\"?><item><name>Item</name></item>"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(415, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/unsupported-media-type", "title" => "Unsupported Media Type", "status" => 415, "detail" => "Unsupported media type"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_binary_response_application_octet_stream(): void
    {
        $app = AppFactory::create_content_types_binary_response_application_octet_stream_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/download/file.bin', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("binary_data_placeholder", $body);
    }

    public function test_content_types_csv_response_text_csv(): void
    {
        $app = AppFactory::create_content_types_csv_response_text_csv_11();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/export/data.csv', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("id,name,price\n1,Item A,10.0\n2,Item B,20.0", $body);
    }

    public function test_content_types_content_negotiation_accept_header(): void
    {
        $app = AppFactory::create_content_types_content_negotiation_accept_header_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/accept-test/1', ['headers' => ["Accept" => "application/json"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 1, "name" => "Item"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_html_response_text_html(): void
    {
        $app = AppFactory::create_content_types_html_response_text_html_13();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/html', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("<html><body><h1>Hello</h1></body></html>", $body);
    }

    public function test_content_types_jpeg_image_response_image_jpeg(): void
    {
        $app = AppFactory::create_content_types_jpeg_image_response_image_jpeg_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/images/photo.jpg', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("jpeg_binary_data", $body);
    }

    public function test_content_types_json_response_application_json(): void
    {
        $app = AppFactory::create_content_types_json_response_application_json_15();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/json', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Item", "price" => 42.0];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_json_with_utf_8_charset(): void
    {
        $app = AppFactory::create_content_types_json_with_utf_8_charset_16();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/unicode', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Café", "emoji" => "☕"];
        $this->assertEquals($expected, $body);
    }

    public function test_content_types_pdf_response_application_pdf(): void
    {
        $app = AppFactory::create_content_types_pdf_response_application_pdf_17();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/download/document.pdf', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("pdf_binary_data", $body);
    }

    public function test_content_types_png_image_response_image_png(): void
    {
        $app = AppFactory::create_content_types_png_image_response_image_png_18();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/images/logo.png', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("png_binary_data", $body);
    }

    public function test_content_types_plain_text_response_text_plain(): void
    {
        $app = AppFactory::create_content_types_plain_text_response_text_plain_19();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/text', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("Hello, World!", $body);
    }

    public function test_content_types_xml_response_application_xml(): void
    {
        $app = AppFactory::create_content_types_xml_response_application_xml_20();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/xml', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>", $body);
    }

    public function test_cookies_24_cookie_samesite_strict(): void
    {
        $app = AppFactory::create_cookies_24_cookie_samesite_strict_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/secure', ['cookies' => ["session_id" => "abc123xyz789"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cookies_25_cookie_samesite_lax(): void
    {
        $app = AppFactory::create_cookies_25_cookie_samesite_lax_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data', ['cookies' => ["tracking" => "track123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cookies_26_cookie_secure_flag(): void
    {
        $app = AppFactory::create_cookies_26_cookie_secure_flag_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/secure', ['cookies' => ["auth_token" => "secure_token_xyz"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cookies_27_cookie_httponly_flag(): void
    {
        $app = AppFactory::create_cookies_27_cookie_httponly_flag_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/secure', ['cookies' => ["session" => "session_abc123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cookies_apikey_cookie_authentication_missing(): void
    {
        $app = AppFactory::create_cookies_apikey_cookie_authentication_missing_5();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me/auth', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["cookie", "key"], "msg" => "Field required", "input" => null]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_apikey_cookie_authentication_success(): void
    {
        $app = AppFactory::create_cookies_apikey_cookie_authentication_success_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['cookies' => ["key" => "secret"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "secret"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_cookie_regex_pattern_validation_fail(): void
    {
        $app = AppFactory::create_cookies_cookie_regex_pattern_validation_fail_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookies/pattern', ['cookies' => ["tracking_id" => "invalid-format"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["cookie", "tracking_id"], "msg" => "String should match pattern '^[A-Z0-9]{8}\$'", "input" => "invalid-format", "ctx" => ["pattern" => "^[A-Z0-9]{8}\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_cookie_regex_pattern_validation_success(): void
    {
        $app = AppFactory::create_cookies_cookie_regex_pattern_validation_success_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookies/pattern', ['cookies' => ["tracking_id" => "ABC12345"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["tracking_id" => "ABC12345"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_cookie_validation_max_length_constraint_fail(): void
    {
        $app = AppFactory::create_cookies_cookie_validation_max_length_constraint_fail_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookies/validated', ['cookies' => ["session_id" => "this_cookie_value_is_way_too_long"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_long", "loc" => ["cookie", "session_id"], "msg" => "String should have at most 20 characters", "input" => "this_cookie_value_is_way_too_long", "ctx" => ["max_length" => 20]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_cookie_validation_min_length_constraint_success(): void
    {
        $app = AppFactory::create_cookies_cookie_validation_min_length_constraint_success_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookies/min-length', ['cookies' => ["token" => "abc"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["token" => "abc"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_cookie_validation_min_length_failure(): void
    {
        $app = AppFactory::create_cookies_cookie_validation_min_length_failure_11();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['cookies' => ["tracking_id" => "ab"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_short", "loc" => ["cookie", "tracking_id"], "msg" => "String should have at least 3 characters", "input" => ""]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_multiple_cookies_success(): void
    {
        $app = AppFactory::create_cookies_multiple_cookies_success_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['cookies' => ["fatebook_tracker" => "tracker456", "session_id" => "session123", "googall_tracker" => "ga789"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["session_id" => "session123", "fatebook_tracker" => "tracker456", "googall_tracker" => "ga789"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_optional_apikey_cookie_missing(): void
    {
        $app = AppFactory::create_cookies_optional_apikey_cookie_missing_13();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["msg" => "Create an account first"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_optional_cookie_parameter_missing(): void
    {
        $app = AppFactory::create_cookies_optional_cookie_parameter_missing_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["ads_id" => null];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_optional_cookie_parameter_success(): void
    {
        $app = AppFactory::create_cookies_optional_cookie_parameter_success_15();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['cookies' => ["ads_id" => "abc123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["ads_id" => "abc123"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_required_cookie_missing(): void
    {
        $app = AppFactory::create_cookies_required_cookie_missing_16();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/cookies', ['cookies' => ["fatebook_tracker" => "tracker456"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["cookie", "session_id"], "msg" => "Field required", "input" => ""]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_delete_cookie(): void
    {
        $app = AppFactory::create_cookies_response_delete_cookie_17();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/delete', ['cookies' => ["session" => "old_session_123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Cookie deleted"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_multiple_cookies(): void
    {
        $app = AppFactory::create_cookies_response_multiple_cookies_18();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/multiple', ['body' => ["user" => "john", "session" => "session123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Multiple cookies set"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_session_cookie_no_max_age(): void
    {
        $app = AppFactory::create_cookies_response_session_cookie_no_max_age_19();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/session', ['body' => ["value" => "session_abc123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Session cookie set"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_cookie_with_samesite_lax(): void
    {
        $app = AppFactory::create_cookies_response_cookie_with_samesite_lax_20();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/samesite-lax', ['body' => ["value" => "lax_cookie"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Cookie set with SameSite=Lax"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_cookie_with_samesite_none(): void
    {
        $app = AppFactory::create_cookies_response_cookie_with_samesite_none_21();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/samesite-none', ['body' => ["value" => "none_cookie"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Cookie set with SameSite=None"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_cookie_with_samesite_strict(): void
    {
        $app = AppFactory::create_cookies_response_cookie_with_samesite_strict_22();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/samesite-strict', ['body' => ["value" => "strict_cookie"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Cookie set with SameSite=Strict"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_cookie_with_attributes(): void
    {
        $app = AppFactory::create_cookies_response_cookie_with_attributes_23();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookie/set', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Cookie set"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_cookie_with_domain_attribute(): void
    {
        $app = AppFactory::create_cookies_response_cookie_with_domain_attribute_24();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/set-with-domain', ['body' => ["value" => "domain_test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Cookie set with domain"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_cookie_with_path_attribute(): void
    {
        $app = AppFactory::create_cookies_response_cookie_with_path_attribute_25();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/set-with-path', ['body' => ["value" => "path_test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Cookie set with path"];
        $this->assertEquals($expected, $body);
    }

    public function test_cookies_response_set_cookie_basic(): void
    {
        $app = AppFactory::create_cookies_response_set_cookie_basic_26();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookie/', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Come to the dark side, we have cookies"];
        $this->assertEquals($expected, $body);
    }

    public function test_cors_06_cors_preflight_method_not_allowed(): void
    {
        $app = AppFactory::create_cors_06_cors_preflight_method_not_allowed_1();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/data', ['headers' => ["Access-Control-Request-Method" => "DELETE", "Origin" => "https://example.com", "Access-Control-Request-Headers" => "Content-Type"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(403, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cors_07_cors_preflight_header_not_allowed(): void
    {
        $app = AppFactory::create_cors_07_cors_preflight_header_not_allowed_2();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/data', ['headers' => ["Origin" => "https://example.com", "Access-Control-Request-Headers" => "X-Custom-Header", "Access-Control-Request-Method" => "POST"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(403, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cors_08_cors_max_age(): void
    {
        $app = AppFactory::create_cors_08_cors_max_age_3();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/data', ['headers' => ["Origin" => "https://example.com", "Access-Control-Request-Method" => "POST", "Access-Control-Request-Headers" => "Content-Type"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(204, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cors_09_cors_expose_headers(): void
    {
        $app = AppFactory::create_cors_09_cors_expose_headers_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["Origin" => "https://example.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cors_10_cors_origin_null(): void
    {
        $app = AppFactory::create_cors_10_cors_origin_null_5();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["Origin" => "null"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(403, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Origin 'null' is not allowed"];
        $this->assertEquals($expected, $body);
    }

    public function test_cors_cors_private_network_access(): void
    {
        $app = AppFactory::create_cors_cors_private_network_access_6();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/local-resource', ['headers' => ["Access-Control-Request-Method" => "GET", "Origin" => "https://public.example.com", "Access-Control-Request-Private-Network" => "true"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(204, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cors_cors_vary_header_for_proper_caching(): void
    {
        $app = AppFactory::create_cors_cors_vary_header_for_proper_caching_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/cached-resource', ['headers' => ["Origin" => "https://app.example.com", "Cache-Control" => "max-age=3600"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["data" => "cacheable resource"];
        $this->assertEquals($expected, $body);
    }

    public function test_cors_cors_multiple_allowed_origins(): void
    {
        $app = AppFactory::create_cors_cors_multiple_allowed_origins_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["Origin" => "https://admin.example.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["data" => "resource data"];
        $this->assertEquals($expected, $body);
    }

    public function test_cors_cors_origin_case_sensitivity(): void
    {
        $app = AppFactory::create_cors_cors_origin_case_sensitivity_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["Origin" => "https://EXAMPLE.COM"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cors_cors_preflight_for_delete_method(): void
    {
        $app = AppFactory::create_cors_cors_preflight_for_delete_method_10();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/resource/456', ['headers' => ["Access-Control-Request-Method" => "DELETE", "Origin" => "https://app.example.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(204, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cors_cors_preflight_for_put_method(): void
    {
        $app = AppFactory::create_cors_cors_preflight_for_put_method_11();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/resource/123', ['headers' => ["Access-Control-Request-Method" => "PUT", "Origin" => "https://app.example.com", "Access-Control-Request-Headers" => "Content-Type, X-Custom-Header"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(204, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cors_cors_preflight_request(): void
    {
        $app = AppFactory::create_cors_cors_preflight_request_12();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/items/', ['headers' => ["Origin" => "https://example.com", "Access-Control-Request-Method" => "POST", "Access-Control-Request-Headers" => "Content-Type, X-Custom-Header"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_cors_cors_regex_pattern_matching_for_origins(): void
    {
        $app = AppFactory::create_cors_cors_regex_pattern_matching_for_origins_13();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["Origin" => "https://subdomain.example.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["data" => "resource data"];
        $this->assertEquals($expected, $body);
    }

    public function test_cors_cors_request_blocked(): void
    {
        $app = AppFactory::create_cors_cors_request_blocked_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ["Origin" => "https://malicious-site.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(403, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "CORS request from origin 'https://malicious-site.com' not allowed"];
        $this->assertEquals($expected, $body);
    }

    public function test_cors_cors_safelisted_headers_without_preflight(): void
    {
        $app = AppFactory::create_cors_cors_safelisted_headers_without_preflight_15();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/form', ['headers' => ["Origin" => "https://app.example.com", "Accept" => "application/json", "Accept-Language" => "en-US", "Content-Type" => "text/plain"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Success"];
        $this->assertEquals($expected, $body);
    }

    public function test_cors_cors_wildcard_origin(): void
    {
        $app = AppFactory::create_cors_cors_wildcard_origin_16();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/public/data', ['headers' => ["Origin" => "https://random-site.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["data" => "public"];
        $this->assertEquals($expected, $body);
    }

    public function test_cors_cors_with_credentials(): void
    {
        $app = AppFactory::create_cors_cors_with_credentials_17();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/user/profile', ['headers' => ["Cookie" => "session=abc123", "Origin" => "https://app.example.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "john"];
        $this->assertEquals($expected, $body);
    }

    public function test_cors_simple_cors_request(): void
    {
        $app = AppFactory::create_cors_simple_cors_request_18();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ["Origin" => "https://example.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["items" => []];
        $this->assertEquals($expected, $body);
    }

    public function test_di_async_factory_dependency_success(): void
    {
        $app = AppFactory::create_di_async_factory_dependency_success_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/db-status', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["pool_status" => "connected", "max_size" => 10];
        $this->assertEquals($expected, $body);
    }

    public function test_di_circular_dependency_detection_error(): void
    {
        $app = AppFactory::create_di_circular_dependency_detection_error_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/circular', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(500, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/dependency-error", "title" => "Dependency Resolution Failed", "status" => 500, "detail" => "Circular dependency detected", "errors" => [["type" => "circular_dependency", "msg" => "Circular dependency detected in dependency graph", "cycle" => ["service_a", "service_b", "service_a"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_di_dependency_injection_in_lifecycle_hooks_success(): void
    {
        $app = AppFactory::create_di_dependency_injection_in_lifecycle_hooks_success_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/hook-di-test', ['headers' => ["authorization" => "Bearer valid_token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["authenticated" => true, "logged" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_di_factory_dependency_success(): void
    {
        $app = AppFactory::create_di_factory_dependency_success_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/timestamp', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["timestamp" => "<<present>>"];
        $this->assertEquals($expected, $body);
    }

    public function test_di_missing_dependency_error(): void
    {
        $app = AppFactory::create_di_missing_dependency_error_5();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/missing-dep', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(500, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/dependency-error", "title" => "Dependency Resolution Failed", "status" => 500, "detail" => "Required dependency not found", "errors" => [["type" => "missing_dependency", "msg" => "Dependency 'non_existent_service' is not registered", "dependency_key" => "non_existent_service"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_di_mixed_singleton_and_per_request_caching_success(): void
    {
        $app = AppFactory::create_di_mixed_singleton_and_per_request_caching_success_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/mixed-caching', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["app_name" => "MyApp", "pool_id" => "<<uuid>>", "context_id" => "<<uuid>>"];
        $this->assertEquals($expected, $body);
    }

    public function test_di_multiple_dependencies_with_cleanup_success(): void
    {
        $app = AppFactory::create_di_multiple_dependencies_with_cleanup_success_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/multi-cleanup-test', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["session_active" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_di_nested_dependencies_3_levels_success(): void
    {
        $app = AppFactory::create_di_nested_dependencies_3_levels_success_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/auth-status', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["auth_enabled" => true, "has_db" => true, "has_cache" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_di_node_js_object_destructuring_injection_success(): void
    {
        $app = AppFactory::create_di_node_js_object_destructuring_injection_success_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/node-destructure', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["db_name" => "PostgreSQL", "log_level" => "info"];
        $this->assertEquals($expected, $body);
    }

    public function test_di_per_request_dependency_caching_success(): void
    {
        $app = AppFactory::create_di_per_request_dependency_caching_success_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/request-id', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["first_id" => "<<uuid>>", "second_id" => "<<same_as:first_id>>"];
        $this->assertEquals($expected, $body);
    }

    public function test_di_python_parameter_name_based_injection_success(): void
    {
        $app = AppFactory::create_di_python_parameter_name_based_injection_success_11();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/python-name-inject', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["db_status" => "connected", "cache_status" => "ready"];
        $this->assertEquals($expected, $body);
    }

    public function test_di_python_type_annotation_based_injection_success(): void
    {
        $app = AppFactory::create_di_python_type_annotation_based_injection_success_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/python-type-inject', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["pool_type" => "PostgreSQL", "cache_type" => "Redis"];
        $this->assertEquals($expected, $body);
    }

    public function test_di_resource_cleanup_after_request_success(): void
    {
        $app = AppFactory::create_di_resource_cleanup_after_request_success_13();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/cleanup-test', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["session_id" => "<<uuid>>", "status" => "completed"];
        $this->assertEquals($expected, $body);
    }

    public function test_di_route_level_dependency_override_success(): void
    {
        $app = AppFactory::create_di_route_level_dependency_override_success_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/override-test', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["mode" => "test", "strict" => false];
        $this->assertEquals($expected, $body);
    }

    public function test_di_ruby_keyword_argument_injection_success(): void
    {
        $app = AppFactory::create_di_ruby_keyword_argument_injection_success_15();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/ruby-kwargs', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["adapter" => "postgresql", "user_id" => 42];
        $this->assertEquals($expected, $body);
    }

    public function test_di_singleton_dependency_caching_success(): void
    {
        $app = AppFactory::create_di_singleton_dependency_caching_success_16();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/app-counter', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["counter_id" => "<<uuid>>", "count" => 1];
        $this->assertEquals($expected, $body);
    }

    public function test_di_type_mismatch_in_dependency_resolution_error(): void
    {
        $app = AppFactory::create_di_type_mismatch_in_dependency_resolution_error_17();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/type-mismatch', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(500, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/dependency-error", "title" => "Dependency Resolution Failed", "status" => 500, "detail" => "Dependency type mismatch", "errors" => [["type" => "type_mismatch", "msg" => "Dependency 'config' type mismatch: expected object, got string", "dependency_key" => "config", "expected_type" => "object", "actual_type" => "string"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_di_value_dependency_injection_success(): void
    {
        $app = AppFactory::create_di_value_dependency_injection_success_18();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/config', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["app_name" => "SpikardApp", "version" => "1.0.0", "max_connections" => 100];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_11_utf8_query_parameter(): void
    {
        $app = AppFactory::create_edge_cases_11_utf8_query_parameter_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=caf%C3%A9', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["term" => "café"];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_12_percent_encoded_special_chars(): void
    {
        $app = AppFactory::create_edge_cases_12_percent_encoded_special_chars_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=hi%20there&term=hi%20there', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["term" => "hi there"];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_13_empty_string_query_param_preserved(): void
    {
        $app = AppFactory::create_edge_cases_13_empty_string_query_param_preserved_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?filter=&filter=', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["filter" => ""];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_14_large_integer_boundary(): void
    {
        $app = AppFactory::create_edge_cases_14_large_integer_boundary_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?id=9007199254740991', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 9007199254740991];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_15_float_precision_preservation(): void
    {
        $app = AppFactory::create_edge_cases_15_float_precision_preservation_5();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/calculate', ['body' => ["value" => 3.141592653589793]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["value" => 3.141592653589793];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_16_negative_zero_handling(): void
    {
        $app = AppFactory::create_edge_cases_16_negative_zero_handling_6();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['body' => ["offset" => -0.0]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["offset" => 0];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_17_extremely_long_string(): void
    {
        $app = AppFactory::create_edge_cases_17_extremely_long_string_7();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/text', ['body' => ["content" => "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_long", "loc" => ["body", "content"], "msg" => "String should have at most 10000 characters", "input" => "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "ctx" => ["max_length" => 10000]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_18_unicode_normalization(): void
    {
        $app = AppFactory::create_edge_cases_18_unicode_normalization_8();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ["name" => "café"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "café"];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_19_emoji_in_strings(): void
    {
        $app = AppFactory::create_edge_cases_19_emoji_in_strings_9();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/messages', ['body' => ["text" => "Hello 👋 World 🌍"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["text" => "Hello 👋 World 🌍"];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_20_null_byte_in_string(): void
    {
        $app = AppFactory::create_edge_cases_20_null_byte_in_string_10();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files', ['body' => ["filename" => "file\x00.txt"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["body", "filename"], "msg" => "String should match pattern '^[^\\x00]+\$'", "input" => "file\x00.txt", "ctx" => ["pattern" => "^[^\\x00]+\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_21_scientific_notation_number(): void
    {
        $app = AppFactory::create_edge_cases_21_scientific_notation_number_11();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/calculate', ['body' => ["value" => 123000.0]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["value" => 123000];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_22_leading_zeros_integer(): void
    {
        $app = AppFactory::create_edge_cases_22_leading_zeros_integer_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data?value=0123', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["value" => 123];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_23_deeply_nested_json_limit(): void
    {
        $app = AppFactory::create_edge_cases_23_deeply_nested_json_limit_13();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['body' => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["nested" => ["value" => "deep"]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(400, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Request body exceeds maximum nesting depth of 32"];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_24_array_with_holes(): void
    {
        $app = AppFactory::create_edge_cases_24_array_with_holes_14();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "items[0]=first&items[2]=third&items[5]=sixth"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["items" => ["first", "third", "sixth"]];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_deeply_nested_structure_10_levels(): void
    {
        $app = AppFactory::create_edge_cases_deeply_nested_structure_10_levels_15();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/nested/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["level1" => ["level2" => ["level3" => ["level4" => ["level5" => ["level6" => ["level7" => ["level8" => ["level9" => ["level10" => ["value" => "deep", "depth" => 10]]]]]]]]]]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Processed deeply nested structure", "max_depth" => 10, "value_found" => "deep"];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_empty_and_null_value_handling(): void
    {
        $app = AppFactory::create_edge_cases_empty_and_null_value_handling_16();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/nulls/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["explicit_null" => null, "empty_string" => "", "empty_array" => [], "empty_object" => (object)[], "zero_number" => 0, "false_boolean" => false]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["explicit_null_is_null" => true, "empty_string_length" => 0, "empty_array_length" => 0, "empty_object_keys" => 0, "zero_is_falsy" => true, "false_is_false" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_float_precision_and_rounding(): void
    {
        $app = AppFactory::create_edge_cases_float_precision_and_rounding_17();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/calculations/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["value1" => 0.1, "value2" => 0.2, "expected_sum" => 0.3, "precise_value" => 3.141592653589793, "very_small" => 1e-10, "very_large" => 1.7976931348623157e+308]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["sum" => 0.30000000000000004, "precise_value" => 3.141592653589793, "very_small" => 1e-10, "very_large" => 1.7976931348623157e+308];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_large_integer_boundary_values(): void
    {
        $app = AppFactory::create_edge_cases_large_integer_boundary_values_18();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/numbers/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["max_safe_int" => 9007199254740991, "large_int" => 9223372036854775807, "negative_large" => -9223372036854775808]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["max_safe_int" => 9007199254740991, "large_int" => 9223372036854775807, "negative_large" => -9223372036854775808];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_special_string_values_and_escaping(): void
    {
        $app = AppFactory::create_edge_cases_special_string_values_and_escaping_19();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/strings/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["empty_string" => "", "whitespace" => "   ", "tabs_newlines" => "line1\n\tline2\r\nline3", "quotes" => "He said \"hello\" and 'goodbye'", "backslashes" => "C:\\\\Users\\\\Path", "unicode_escapes" => "\\u0048\\u0065\\u006c\\u006c\\u006f", "special_chars" => "!@#\$%^&*()_+-=[]{}|;':\",./<>?"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["empty_string" => "", "whitespace" => "   ", "tabs_newlines" => "line1\n\tline2\r\nline3", "quotes" => "He said \"hello\" and 'goodbye'", "backslashes" => "C:\\\\Users\\\\Path", "unicode_escapes" => "Hello", "special_chars" => "!@#\$%^&*()_+-=[]{}|;':\",./<>?"];
        $this->assertEquals($expected, $body);
    }

    public function test_edge_cases_unicode_and_emoji_handling(): void
    {
        $app = AppFactory::create_edge_cases_unicode_and_emoji_handling_20();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json; charset=utf-8"], 'body' => ["name" => "Coffee Shop ☕", "description" => "Best café in München 🇩🇪", "tags" => ["食べ物", "音楽", "💰"], "emoji_reactions" => "👍❤️😂🎉"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 1, "name" => "Coffee Shop ☕", "description" => "Best café in München 🇩🇪", "tags" => ["食べ物", "音楽", "💰"], "emoji_reactions" => "👍❤️😂🎉"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_30_bearer_token_format_valid(): void
    {
        $app = AppFactory::create_headers_30_bearer_token_format_valid_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected', ['headers' => ["Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_headers_31_bearer_token_format_invalid(): void
    {
        $app = AppFactory::create_headers_31_bearer_token_format_invalid_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected', ['headers' => ["Authorization" => "Bearer invalid token with spaces"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["headers", "authorization"], "msg" => "Invalid Bearer token format", "ctx" => ["pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*\$", "value" => "Bearer invalid token with spaces"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_32_bearer_token_missing_prefix(): void
    {
        $app = AppFactory::create_headers_32_bearer_token_missing_prefix_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected', ['headers' => ["Authorization" => "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["headers", "authorization"], "msg" => "Invalid Bearer token format", "ctx" => ["pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*\$", "value" => "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_33_api_key_header_valid(): void
    {
        $app = AppFactory::create_headers_33_api_key_header_valid_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["X-API-Key" => "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_headers_34_api_key_header_invalid(): void
    {
        $app = AppFactory::create_headers_34_api_key_header_invalid_5();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ["X-API-Key" => "invalid-key"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["headers", "x-api-key"], "msg" => "Invalid API key format", "ctx" => ["pattern" => "^[a-f0-9]{32}\$", "value" => "invalid-key"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_accept_header_json(): void
    {
        $app = AppFactory::create_headers_accept_header_json_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/accept', ['headers' => ["Accept" => "application/json"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["accept" => "application/json"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_accept_encoding_header(): void
    {
        $app = AppFactory::create_headers_accept_encoding_header_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/accept-encoding', ['headers' => ["Accept-Encoding" => "gzip, deflate, br"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["accept_encoding" => "gzip, deflate, br"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_accept_language_header(): void
    {
        $app = AppFactory::create_headers_accept_language_header_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/accept-language', ['headers' => ["Accept-Language" => "en-US,en;q=0.9"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["accept_language" => "en-US,en;q=0.9"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_authorization_header_missing(): void
    {
        $app = AppFactory::create_headers_authorization_header_missing_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["headers", "authorization"], "msg" => "Field required", "input" => null]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_authorization_header_success(): void
    {
        $app = AppFactory::create_headers_authorization_header_success_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['headers' => ["Authorization" => "Digest foobar"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["scheme" => "Digest", "credentials" => "foobar"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_authorization_header_wrong_scheme(): void
    {
        $app = AppFactory::create_headers_authorization_header_wrong_scheme_11();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['headers' => ["Authorization" => "Other invalidauthorization"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["headers", "authorization"], "msg" => "String should match pattern '^Digest .+'", "input" => "Other invalidauthorization"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_basic_authentication_success(): void
    {
        $app = AppFactory::create_headers_basic_authentication_success_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/basic-auth', ['headers' => ["Authorization" => "Basic dXNlcm5hbWU6cGFzc3dvcmQ="]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "username", "password" => "password"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_bearer_token_authentication_missing(): void
    {
        $app = AppFactory::create_headers_bearer_token_authentication_missing_13();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/bearer-auth', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["headers", "authorization"], "msg" => "Field required", "input" => null]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_bearer_token_authentication_success(): void
    {
        $app = AppFactory::create_headers_bearer_token_authentication_success_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/bearer-auth', ['headers' => ["Authorization" => "Bearer valid_token_123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["token" => "valid_token_123"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_content_type_header_application_json(): void
    {
        $app = AppFactory::create_headers_content_type_header_application_json_15();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/content-type', ['headers' => ["Content-Type" => "application/json"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["content_type" => "application/json"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_header_case_insensitivity_access(): void
    {
        $app = AppFactory::create_headers_header_case_insensitivity_access_16();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/echo', ['headers' => ["content-type" => "application/json"], 'body' => ["test" => "data"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["content_type_lower" => "application/json", "content_type_upper" => "application/json", "content_type_mixed" => "application/json"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_header_regex_validation_fail(): void
    {
        $app = AppFactory::create_headers_header_regex_validation_fail_17();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/pattern', ['headers' => ["X-Request-Id" => "invalid-format"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["headers", "x-request-id"], "msg" => "String should match pattern '^[0-9]{3,}\$'", "input" => "invalid-format", "ctx" => ["pattern" => "^[0-9]{3,}\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_header_regex_validation_success(): void
    {
        $app = AppFactory::create_headers_header_regex_validation_success_18();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/pattern', ['headers' => ["X-Request-Id" => "12345"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["x_request_id" => "12345"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_header_validation_max_length_constraint_fail(): void
    {
        $app = AppFactory::create_headers_header_validation_max_length_constraint_fail_19();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/max-length', ['headers' => ["X-Session-Id" => "this_is_way_too_long_for_validation"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_long", "loc" => ["headers", "x-session-id"], "msg" => "String should have at most 20 characters", "input" => "this_is_way_too_long_for_validation", "ctx" => ["max_length" => 20]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_header_validation_min_length_constraint(): void
    {
        $app = AppFactory::create_headers_header_validation_min_length_constraint_20();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/validated', ['headers' => ["X-Token" => "ab"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_short", "loc" => ["headers", "x-token"], "msg" => "String should have at least 3 characters", "input" => "ab", "ctx" => ["min_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_header_with_underscore_conversion_explicit(): void
    {
        $app = AppFactory::create_headers_header_with_underscore_conversion_explicit_21();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/underscore', ['headers' => ["X-Token" => "secret123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["x_token" => "secret123"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_host_header(): void
    {
        $app = AppFactory::create_headers_host_header_22();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/host', ['headers' => ["Host" => "example.com:8080"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["host" => "example.com:8080"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_multiple_custom_headers(): void
    {
        $app = AppFactory::create_headers_multiple_custom_headers_23();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/multiple', ['headers' => ["X-Request-Id" => "req-12345", "X-Trace-Id" => "trace-abc", "X-Client-Version" => "1.2.3"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["x_request_id" => "req-12345", "x_client_version" => "1.2.3", "x_trace_id" => "trace-abc"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_multiple_header_values_x_token(): void
    {
        $app = AppFactory::create_headers_multiple_header_values_x_token_24();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ["x-token" => "foo, bar"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["X-Token values" => ["foo", "bar"]];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_optional_header_with_none_default_missing(): void
    {
        $app = AppFactory::create_headers_optional_header_with_none_default_missing_25();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["strange_header" => null];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_origin_header(): void
    {
        $app = AppFactory::create_headers_origin_header_26();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/origin', ['headers' => ["Origin" => "https://example.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["origin" => "https://example.com"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_referer_header(): void
    {
        $app = AppFactory::create_headers_referer_header_27();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/referer', ['headers' => ["Referer" => "https://example.com/page"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["referer" => "https://example.com/page"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_user_agent_header_custom_value(): void
    {
        $app = AppFactory::create_headers_user_agent_header_custom_value_28();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ["User-Agent" => "Mozilla/5.0 Custom Browser"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["User-Agent" => "Mozilla/5.0 Custom Browser"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_user_agent_header_default_value(): void
    {
        $app = AppFactory::create_headers_user_agent_header_default_value_29();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["User-Agent" => "testclient"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_x_api_key_optional_header_missing(): void
    {
        $app = AppFactory::create_headers_x_api_key_optional_header_missing_30();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["msg" => "Hello World"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_x_api_key_optional_header_success(): void
    {
        $app = AppFactory::create_headers_x_api_key_optional_header_success_31();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['headers' => ["key" => "secret"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["msg" => "Hello secret"];
        $this->assertEquals($expected, $body);
    }

    public function test_headers_x_api_key_required_header_missing(): void
    {
        $app = AppFactory::create_headers_x_api_key_required_header_missing_32();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["headers", "x-api-key"], "msg" => "Field required", "input" => null]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_headers_x_api_key_required_header_success(): void
    {
        $app = AppFactory::create_headers_x_api_key_required_header_success_33();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['headers' => ["key" => "secret"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "secret"];
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_delete_remove_resource(): void
    {
        $app = AppFactory::create_http_methods_delete_remove_resource_1();
        $client = TestClient::create($app);
        $response = $client->request('DELETE', '/items/1', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = [];
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_delete_resource_not_found(): void
    {
        $app = AppFactory::create_http_methods_delete_resource_not_found_2();
        $client = TestClient::create($app);
        $response = $client->request('DELETE', '/items/999', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = [];
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_delete_with_response_body(): void
    {
        $app = AppFactory::create_http_methods_delete_with_response_body_3();
        $client = TestClient::create($app);
        $response = $client->request('DELETE', '/items/1', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 1, "name" => "Deleted Item", "message" => "Item deleted successfully"];
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_head_get_metadata_without_body(): void
    {
        $app = AppFactory::create_http_methods_head_get_metadata_without_body_4();
        $client = TestClient::create($app);
        $response = $client->request('HEAD', '/items/1', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_http_methods_options_cors_preflight_request(): void
    {
        $app = AppFactory::create_http_methods_options_cors_preflight_request_5();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/items/', ['headers' => ["Origin" => "https://example.com", "Access-Control-Request-Method" => "POST", "Access-Control-Request-Headers" => "Content-Type"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_http_methods_patch_partial_update(): void
    {
        $app = AppFactory::create_http_methods_patch_partial_update_6();
        $client = TestClient::create($app);
        $response = $client->request('PATCH', '/items/1', ['headers' => ["Content-Type" => "application/json"], 'body' => ["price" => 79.99]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 1, "name" => "Existing Item", "price" => 79.99, "in_stock" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_patch_update_multiple_fields(): void
    {
        $app = AppFactory::create_http_methods_patch_update_multiple_fields_7();
        $client = TestClient::create($app);
        $response = $client->request('PATCH', '/items/1', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Updated Name", "price" => 89.99, "in_stock" => false]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 1, "name" => "Updated Name", "price" => 89.99, "in_stock" => false];
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_put_complete_resource_replacement(): void
    {
        $app = AppFactory::create_http_methods_put_complete_resource_replacement_8();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/1', ['headers' => ["Content-Type" => "application/json"], 'body' => ["id" => 1, "name" => "Updated Item", "description" => "Completely replaced", "price" => 99.99, "in_stock" => true]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 1, "name" => "Updated Item", "description" => "Completely replaced", "price" => 99.99, "in_stock" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_put_create_resource_if_doesn_t_exist(): void
    {
        $app = AppFactory::create_http_methods_put_create_resource_if_doesn_t_exist_9();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/999', ['headers' => ["Content-Type" => "application/json"], 'body' => ["id" => 999, "name" => "New Item", "price" => 49.99]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 999, "name" => "New Item", "price" => 49.99];
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_put_idempotent_operation(): void
    {
        $app = AppFactory::create_http_methods_put_idempotent_operation_10();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/1', ['headers' => ["Content-Type" => "application/json"], 'body' => ["id" => 1, "name" => "Fixed Name", "price" => 50.0]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 1, "name" => "Fixed Name", "price" => 50.0];
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_put_missing_required_field(): void
    {
        $app = AppFactory::create_http_methods_put_missing_required_field_11();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/1', ['headers' => ["Content-Type" => "application/json"], 'body' => ["id" => 1, "name" => "Item Name"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["body", "price"], "msg" => "Field required", "input" => ["id" => 1, "name" => "Item Name"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_http_methods_put_validation_error(): void
    {
        $app = AppFactory::create_http_methods_put_validation_error_12();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/1', ['headers' => ["Content-Type" => "application/json"], 'body' => ["id" => 1, "name" => "X", "price" => -10]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "2 validation errors in request", "errors" => [["type" => "string_too_short", "loc" => ["body", "name"], "msg" => "String should have at least 3 characters", "input" => "X", "ctx" => ["min_length" => 3]], ["type" => "greater_than", "loc" => ["body", "price"], "msg" => "Input should be greater than 0", "input" => -10, "ctx" => ["gt" => 0]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_29_nested_object_validation_success(): void
    {
        $app = AppFactory::create_json_bodies_29_nested_object_validation_success_1();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ["profile" => ["name" => "John Doe", "email" => "john@example.com"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_30_nested_object_missing_field(): void
    {
        $app = AppFactory::create_json_bodies_30_nested_object_missing_field_2();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ["profile" => ["name" => "John Doe"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["body", "profile", "email"], "msg" => "Field required", "input" => ["name" => "John Doe"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_31_nullable_property_null_value(): void
    {
        $app = AppFactory::create_json_bodies_31_nullable_property_null_value_3();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ["name" => "Test User", "description" => null]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_32_schema_ref_definitions(): void
    {
        $app = AppFactory::create_json_bodies_32_schema_ref_definitions_4();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/products', ['body' => ["product" => ["name" => "Widget", "price" => 9.99]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_33_allof_schema_composition(): void
    {
        $app = AppFactory::create_json_bodies_33_allof_schema_composition_5();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items', ['body' => ["name" => "Product", "price" => 29.99]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_34_additional_properties_false(): void
    {
        $app = AppFactory::create_json_bodies_34_additional_properties_false_6();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ["name" => "John", "email" => "john@example.com", "extra_field" => "should fail"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body", "extra_field"], "msg" => "Additional properties are not allowed", "ctx" => ["additional_properties" => false, "unexpected_field" => "extra_field"], "input" => ["name" => "John", "email" => "john@example.com", "extra_field" => "should fail"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_35_oneof_schema_success(): void
    {
        $app = AppFactory::create_json_bodies_35_oneof_schema_success_7();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/payment', ['body' => ["credit_card" => "1234567812345678"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_36_oneof_schema_multiple_match_failure(): void
    {
        $app = AppFactory::create_json_bodies_36_oneof_schema_multiple_match_failure_8();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/payment', ['body' => ["credit_card" => "1234567812345678", "paypal_email" => "user@example.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body"], "msg" => "{\"credit_card\":\"1234567812345678\",\"paypal_email\":\"user@example.com\"} is valid under more than one of the schemas listed in the 'oneOf' keyword", "input" => ["credit_card" => "1234567812345678", "paypal_email" => "user@example.com"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_37_oneof_schema_no_match_failure(): void
    {
        $app = AppFactory::create_json_bodies_37_oneof_schema_no_match_failure_9();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/payment', ['body' => ["bitcoin_address" => "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body"], "msg" => "{\"bitcoin_address\":\"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa\"} is not valid under any of the schemas listed in the 'oneOf' keyword", "input" => ["bitcoin_address" => "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_38_anyof_schema_success(): void
    {
        $app = AppFactory::create_json_bodies_38_anyof_schema_success_10();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/contact', ['body' => ["name" => "John Doe", "email" => "john@example.com"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_39_anyof_schema_multiple_match_success(): void
    {
        $app = AppFactory::create_json_bodies_39_anyof_schema_multiple_match_success_11();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/contact', ['body' => ["name" => "John Doe", "email" => "john@example.com", "phone" => "+1-555-0100"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_40_anyof_schema_failure(): void
    {
        $app = AppFactory::create_json_bodies_40_anyof_schema_failure_12();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/contact', ['body' => ["name" => "John Doe"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body"], "msg" => "{\"name\":\"John Doe\"} is not valid under any of the schemas listed in the 'anyOf' keyword", "input" => ["name" => "John Doe"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_41_not_schema_success(): void
    {
        $app = AppFactory::create_json_bodies_41_not_schema_success_13();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ["username" => "john_doe"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_42_not_schema_failure(): void
    {
        $app = AppFactory::create_json_bodies_42_not_schema_failure_14();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ["username" => "admin"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body", "username"], "msg" => "{\"enum\":[\"admin\",\"root\",\"system\"]} is not allowed for \"admin\"", "input" => "admin"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_43_const_validation_success(): void
    {
        $app = AppFactory::create_json_bodies_43_const_validation_success_15();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/v1/data', ['body' => ["version" => "1.0", "data" => "test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_44_const_validation_failure(): void
    {
        $app = AppFactory::create_json_bodies_44_const_validation_failure_16();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/v1/data', ['body' => ["version" => "2.0", "data" => "test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body", "version"], "msg" => "\"1.0\" was expected", "input" => "2.0"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_45_minproperties_validation_success(): void
    {
        $app = AppFactory::create_json_bodies_45_minproperties_validation_success_17();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/config', ['body' => ["host" => "localhost", "port" => 8080]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_46_minproperties_validation_failure(): void
    {
        $app = AppFactory::create_json_bodies_46_minproperties_validation_failure_18();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/config', ['body' => ["host" => "localhost"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body"], "msg" => "{\"host\":\"localhost\"} has less than 2 properties", "input" => ["host" => "localhost"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_47_maxproperties_validation_failure(): void
    {
        $app = AppFactory::create_json_bodies_47_maxproperties_validation_failure_19();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/config', ['body' => ["host" => "localhost", "port" => 8080, "ssl" => true, "debug" => false]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body"], "msg" => "{\"debug\":false,\"host\":\"localhost\",\"port\":8080,\"ssl\":true} has more than 3 properties", "input" => ["host" => "localhost", "port" => 8080, "ssl" => true, "debug" => false]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_48_dependencies_validation_success(): void
    {
        $app = AppFactory::create_json_bodies_48_dependencies_validation_success_20();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/billing', ['body' => ["name" => "John Doe", "credit_card" => "1234567812345678", "billing_address" => "123 Main St"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_49_dependencies_validation_failure(): void
    {
        $app = AppFactory::create_json_bodies_49_dependencies_validation_failure_21();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/billing', ['body' => ["name" => "John Doe", "credit_card" => "1234567812345678"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body"], "msg" => "\"billing_address\" is a required property", "input" => ["name" => "John Doe", "credit_card" => "1234567812345678"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_50_deep_nesting_4_levels(): void
    {
        $app = AppFactory::create_json_bodies_50_deep_nesting_4_levels_22();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['body' => ["user" => ["profile" => ["contact" => ["address" => ["street" => "123 Main St"]]]]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_json_bodies_array_of_objects_success(): void
    {
        $app = AppFactory::create_json_bodies_array_of_objects_success_23();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/list', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Product Bundle", "tags" => ["electronics", "gadget"], "images" => [["url" => "https://example.com/img1.jpg", "name" => "Front"], ["url" => "https://example.com/img2.jpg", "name" => "Back"]]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Product Bundle", "tags" => ["electronics", "gadget"], "images" => [["url" => "https://example.com/img1.jpg", "name" => "Front"], ["url" => "https://example.com/img2.jpg", "name" => "Back"]]];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_array_of_primitive_values(): void
    {
        $app = AppFactory::create_json_bodies_array_of_primitive_values_24();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Product", "tags" => ["electronics", "gadget", "new"], "ratings" => [4.5, 4.8, 5.0, 4.2]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Product", "tags" => ["electronics", "gadget", "new"], "ratings" => [4.5, 4.8, 5.0, 4.2]];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_body_with_query_parameters(): void
    {
        $app = AppFactory::create_json_bodies_body_with_query_parameters_25();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/?limit=10&limit=10', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 42.0]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item" => ["name" => "Item", "price" => 42.0], "limit" => 10];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_boolean_field_success(): void
    {
        $app = AppFactory::create_json_bodies_boolean_field_success_26();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 42.0, "in_stock" => true]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Item", "price" => 42.0, "in_stock" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_date_field_success(): void
    {
        $app = AppFactory::create_json_bodies_date_field_success_27();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/events/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Conference", "event_date" => "2024-03-15"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Conference", "event_date" => "2024-03-15"];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_datetime_field_success(): void
    {
        $app = AppFactory::create_json_bodies_datetime_field_success_28();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/events/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Meeting", "created_at" => "2024-03-15T10:30:00Z"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Meeting", "created_at" => "2024-03-15T10:30:00Z"];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_deeply_nested_objects(): void
    {
        $app = AppFactory::create_json_bodies_deeply_nested_objects_29();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/nested', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Product", "price" => 100.0, "seller" => ["name" => "John Doe", "address" => ["street" => "123 Main St", "city" => "Springfield", "country" => ["name" => "USA", "code" => "US"]]]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Product", "price" => 100.0, "seller" => ["name" => "John Doe", "address" => ["street" => "123 Main St", "city" => "Springfield", "country" => ["name" => "USA", "code" => "US"]]]];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_empty_json_object(): void
    {
        $app = AppFactory::create_json_bodies_empty_json_object_30();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/optional-all', ['headers' => ["Content-Type" => "application/json"], 'body' => (object)[]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => null, "description" => null, "price" => null, "tax" => null];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_empty_array_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies_empty_array_validation_fail_31();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/list-validated', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Product", "tags" => []]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "too_short", "loc" => ["body", "tags"], "msg" => "List should have at least 1 item after validation", "input" => [], "ctx" => ["min_length" => 1]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_enum_field_invalid_value(): void
    {
        $app = AppFactory::create_json_bodies_enum_field_invalid_value_32();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "category" => "furniture"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "enum", "loc" => ["body", "category"], "msg" => "Input should be 'electronics', 'clothing' or 'books'", "input" => "furniture", "ctx" => ["expected" => "'electronics', 'clothing' or 'books'"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_enum_field_success(): void
    {
        $app = AppFactory::create_json_bodies_enum_field_success_33();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "category" => "electronics"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Item", "category" => "electronics"];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_extra_fields_ignored_no_additionalproperties(): void
    {
        $app = AppFactory::create_json_bodies_extra_fields_ignored_no_additionalproperties_34();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 42.0, "extra_field" => "this should be ignored", "another_extra" => 123]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Item", "price" => 42.0];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_field_type_validation_invalid_type(): void
    {
        $app = AppFactory::create_json_bodies_field_type_validation_invalid_type_35();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Foo", "description" => "A very nice Item", "price" => "not a number", "tax" => 3.2]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "float_parsing", "loc" => ["body", "price"], "msg" => "Input should be a valid number, unable to parse string as a number", "input" => "not a number"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_nested_object_success(): void
    {
        $app = AppFactory::create_json_bodies_nested_object_success_36();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/nested', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Foo", "price" => 42.0, "image" => ["url" => "https://example.com/image.jpg", "name" => "Product Image"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Foo", "price" => 42.0, "image" => ["url" => "https://example.com/image.jpg", "name" => "Product Image"]];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_null_value_for_optional_field(): void
    {
        $app = AppFactory::create_json_bodies_null_value_for_optional_field_37();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 42.0, "description" => null, "tax" => null]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Item", "price" => 42.0, "description" => null, "tax" => null];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_numeric_ge_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies_numeric_ge_validation_fail_38();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 0.5]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "greater_than_equal", "loc" => ["body", "price"], "msg" => "Input should be greater than or equal to 1", "input" => 0.5, "ctx" => ["ge" => 1]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_numeric_le_validation_success(): void
    {
        $app = AppFactory::create_json_bodies_numeric_le_validation_success_39();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 100.0]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Item", "price" => 100.0];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_optional_fields_omitted(): void
    {
        $app = AppFactory::create_json_bodies_optional_fields_omitted_40();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Foo", "price" => 35.4]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Foo", "price" => 35.4, "description" => null, "tax" => null];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_patch_partial_update(): void
    {
        $app = AppFactory::create_json_bodies_patch_partial_update_41();
        $client = TestClient::create($app);
        $response = $client->request('PATCH', '/items/1', ['headers' => ["Content-Type" => "application/json"], 'body' => ["price" => 45.0]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Original Item", "price" => 45.0, "description" => "Original description"];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_required_field_missing_validation_error(): void
    {
        $app = AppFactory::create_json_bodies_required_field_missing_validation_error_42();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["description" => "A very nice Item", "price" => 35.4]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["body", "name"], "msg" => "Field required", "input" => ["description" => "A very nice Item", "price" => 35.4]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_simple_json_object_success(): void
    {
        $app = AppFactory::create_json_bodies_simple_json_object_success_43();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Foo", "description" => "A very nice Item", "price" => 35.4, "tax" => 3.2]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Foo", "description" => "A very nice Item", "price" => 35.4, "tax" => 3.2];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_string_max_length_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies_string_max_length_validation_fail_44();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "This is a very long name that exceeds the maximum length", "price" => 35.4]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_long", "loc" => ["body", "name"], "msg" => "String should have at most 50 characters", "input" => "This is a very long name that exceeds the maximum length", "ctx" => ["max_length" => 50]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_string_min_length_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies_string_min_length_validation_fail_45();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "ab", "price" => 35.4]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_short", "loc" => ["body", "name"], "msg" => "String should have at least 3 characters", "input" => "ab", "ctx" => ["min_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_string_pattern_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies_string_pattern_validation_fail_46();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "sku" => "ABC-123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["body", "sku"], "msg" => "String should match pattern '^[A-Z]{3}[0-9]{4}\$'", "input" => "ABC-123", "ctx" => ["pattern" => "^[A-Z]{3}[0-9]{4}\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_string_pattern_validation_success(): void
    {
        $app = AppFactory::create_json_bodies_string_pattern_validation_success_47();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "sku" => "ABC1234"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Item", "sku" => "ABC1234"];
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_uuid_field_invalid_format(): void
    {
        $app = AppFactory::create_json_bodies_uuid_field_invalid_format_48();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "item_id" => "not-a-valid-uuid"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "uuid_parsing", "loc" => ["body", "item_id"], "msg" => "Input should be a valid UUID", "input" => "not-a-valid-uuid"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_json_bodies_uuid_field_success(): void
    {
        $app = AppFactory::create_json_bodies_uuid_field_success_49();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "Item", "item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_hook_execution_order(): void
    {
        $app = AppFactory::create_lifecycle_hooks_hook_execution_order_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-hook-order', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Hooks executed in order", "execution_order" => ["first_hook", "second_hook", "third_hook"]];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_multiple_hooks_all_phases(): void
    {
        $app = AppFactory::create_lifecycle_hooks_multiple_hooks_all_phases_2();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/full-lifecycle', ['headers' => ["Authorization" => "Bearer valid-token-12345", "Content-Type" => "application/json"], 'body' => ["user_id" => "user-123", "action" => "update_profile"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Action completed successfully", "user_id" => "user-123", "action" => "update_profile", "request_id" => ".*"];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_onerror_error_logging(): void
    {
        $app = AppFactory::create_lifecycle_hooks_onerror_error_logging_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-error', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(500, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Internal Server Error", "message" => "An unexpected error occurred", "error_id" => ".*"];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_onrequest_request_logging(): void
    {
        $app = AppFactory::create_lifecycle_hooks_onrequest_request_logging_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-on-request', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "onRequest hooks executed", "request_logged" => true, "has_request_id" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_onresponse_response_timing(): void
    {
        $app = AppFactory::create_lifecycle_hooks_onresponse_response_timing_5();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-timing', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Response with timing info"];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_onresponse_security_headers(): void
    {
        $app = AppFactory::create_lifecycle_hooks_onresponse_security_headers_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-security-headers', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Response with security headers"];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_prehandler_authentication_failed_short_circuit(): void
    {
        $app = AppFactory::create_lifecycle_hooks_prehandler_authentication_failed_short_circuit_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected-resource-fail', ['headers' => ["Authorization" => "Bearer invalid-token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Unauthorized", "message" => "Invalid or expired authentication token"];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_prehandler_authentication_success(): void
    {
        $app = AppFactory::create_lifecycle_hooks_prehandler_authentication_success_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected-resource', ['headers' => ["Authorization" => "Bearer valid-token-12345"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Access granted", "user_id" => "user-123", "authenticated" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_prehandler_authorization_check(): void
    {
        $app = AppFactory::create_lifecycle_hooks_prehandler_authorization_check_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/admin-only', ['headers' => ["Authorization" => "Bearer admin-token-67890"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Admin access granted", "user_id" => "admin-456", "role" => "admin"];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_prehandler_authorization_forbidden_short_circuit(): void
    {
        $app = AppFactory::create_lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/admin-only-forbidden', ['headers' => ["Authorization" => "Bearer user-token-11111"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(403, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Forbidden", "message" => "Admin role required for this endpoint"];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit(): void
    {
        $app = AppFactory::create_lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit_11();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/test-rate-limit-exceeded', ['headers' => ["Content-Type" => "application/json"], 'body' => ["data" => "test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(429, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Rate limit exceeded", "message" => "Too many requests, please try again later"];
        $this->assertEquals($expected, $body);
    }

    public function test_lifecycle_hooks_prevalidation_rate_limiting(): void
    {
        $app = AppFactory::create_lifecycle_hooks_prevalidation_rate_limiting_12();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/test-rate-limit', ['headers' => ["Content-Type" => "application/json"], 'body' => ["data" => "test"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Request accepted", "rate_limit_checked" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_17_file_magic_number_png_success(): void
    {
        $app = AppFactory::create_multipart_17_file_magic_number_png_success_1();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [["field_name" => "image", "filename" => "test.png", "content_type" => "image/png", "magic_bytes" => "89504e470d0a1a0a"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_multipart_18_file_magic_number_jpeg_success(): void
    {
        $app = AppFactory::create_multipart_18_file_magic_number_jpeg_success_2();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [["field_name" => "image", "filename" => "test.jpg", "content_type" => "image/jpeg", "magic_bytes" => "ffd8ffe0"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_multipart_19_file_mime_spoofing_png_as_jpeg(): void
    {
        $app = AppFactory::create_multipart_19_file_mime_spoofing_png_as_jpeg_3();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [["field_name" => "image", "filename" => "fake.jpg", "content_type" => "image/jpeg", "magic_bytes" => "89504e470d0a1a0a"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["files", "image"], "msg" => "File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png", "ctx" => ["declared_mime" => "image/jpeg", "detected_type" => "image/png", "magic_bytes" => "89504e470d0a1a0a"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_20_file_mime_spoofing_jpeg_as_png(): void
    {
        $app = AppFactory::create_multipart_20_file_mime_spoofing_jpeg_as_png_4();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [["field_name" => "image", "filename" => "fake.png", "content_type" => "image/png", "magic_bytes" => "ffd8ffe0"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["files", "image"], "msg" => "File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg", "ctx" => ["declared_mime" => "image/png", "detected_type" => "image/jpeg", "magic_bytes" => "ffd8ffe0"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_21_file_pdf_magic_number_success(): void
    {
        $app = AppFactory::create_multipart_21_file_pdf_magic_number_success_5();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [["field_name" => "document", "filename" => "test.pdf", "content_type" => "application/pdf", "magic_bytes" => "25504446"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_multipart_22_file_empty_buffer(): void
    {
        $app = AppFactory::create_multipart_22_file_empty_buffer_6();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [["field_name" => "file", "filename" => "empty.txt", "content_type" => "text/plain", "magic_bytes" => ""]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["files", "file"], "msg" => "File buffer is empty", "ctx" => ["buffer_size" => 0]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_content_type_validation_invalid_type(): void
    {
        $app = AppFactory::create_multipart_content_type_validation_invalid_type_7();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/images-only', ['files' => [["field_name" => "file", "filename" => "script.sh", "content" => "#!/bin/bash\necho hello", "content_type" => "application/x-sh"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_multipart_empty_file_upload(): void
    {
        $app = AppFactory::create_multipart_empty_file_upload_8();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/upload', ['files' => [["field_name" => "file", "filename" => "empty.txt", "content" => "", "content_type" => "text/plain"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["filename" => "empty.txt", "size" => 0];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_file_list_upload_array_of_files(): void
    {
        $app = AppFactory::create_multipart_file_list_upload_array_of_files_9();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/list', ['files' => [["field_name" => "files", "filename" => "file1.txt", "content" => "content of file 1", "content_type" => "text/plain"], ["field_name" => "files", "filename" => "file2.txt", "content" => "content of file 2", "content_type" => "text/plain"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["filenames" => ["file1.txt", "file2.txt"], "total_size" => 35];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_file_size_validation_too_large(): void
    {
        $app = AppFactory::create_multipart_file_size_validation_too_large_10();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/validated', ['files' => [["field_name" => "file", "filename" => "large.txt", "content" => "x", "content_type" => "text/plain"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(413, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "File too large. Maximum size is 1MB"];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_file_upload_with_custom_headers(): void
    {
        $app = AppFactory::create_multipart_file_upload_with_custom_headers_11();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['headers' => ["Content-Type" => "multipart/form-data"], 'files' => [["field_name" => "test2", "filename" => "test2.txt", "content" => "<file2 content>", "content_type" => "text/plain", "content_encoding" => "text"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["test2" => ["filename" => "test2.txt", "size" => 15, "content" => "<file2 content>", "content_type" => "text/plain", "headers" => [["content-disposition", "form-data; name=\"test2\"; filename=\"test2.txt\""], ["content-type", "text/plain"], ["x-custom", "f2"]]]];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_file_upload_without_filename(): void
    {
        $app = AppFactory::create_multipart_file_upload_without_filename_12();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['headers' => ["Content-Type" => "multipart/form-data"], 'files' => [["field_name" => "test1", "content" => "<file1 content>", "content_encoding" => "text"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["test1" => "<file1 content>"];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_form_data_without_files(): void
    {
        $app = AppFactory::create_multipart_form_data_without_files_13();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['headers' => ["Content-Type" => "multipart/form-data"], 'data' => ["some" => "data"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["some" => "data"];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_image_file_upload(): void
    {
        $app = AppFactory::create_multipart_image_file_upload_14();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/image', ['files' => [["field_name" => "image", "filename" => "photo.jpg", "content" => "fake_jpeg_content_here", "content_type" => "image/jpeg"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["filename" => "photo.jpg", "content_type" => "image/jpeg", "size" => 22];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_mixed_files_and_form_data(): void
    {
        $app = AppFactory::create_multipart_mixed_files_and_form_data_15();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['headers' => ["Content-Type" => "multipart/form-data"], 'files' => [["field_name" => "file", "filename" => "upload.txt", "content" => "file data here", "content_type" => "text/plain", "content_encoding" => "text"]], 'data' => ["age" => "25", "username" => "testuser", "active" => "true"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["file" => ["filename" => "upload.txt", "size" => 14, "content" => "file data here", "content_type" => "text/plain"], "username" => "testuser", "age" => "25", "active" => "true"];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_multiple_file_uploads(): void
    {
        $app = AppFactory::create_multipart_multiple_file_uploads_16();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['headers' => ["Content-Type" => "multipart/form-data"], 'files' => [["field_name" => "test1", "filename" => "test1.txt", "content" => "<file1 content>", "content_type" => "text/plain", "content_encoding" => "text"], ["field_name" => "test2", "filename" => "test2.txt", "content" => "<file2 content>", "content_type" => "text/plain", "content_encoding" => "text"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["test1" => ["filename" => "test1.txt", "size" => 15, "content" => "<file1 content>", "content_type" => "text/plain"], "test2" => ["filename" => "test2.txt", "size" => 15, "content" => "<file2 content>", "content_type" => "text/plain"]];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_multiple_values_for_same_field_name(): void
    {
        $app = AppFactory::create_multipart_multiple_values_for_same_field_name_17();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['headers' => ["Content-Type" => "multipart/form-data"], 'files' => [["field_name" => "files", "filename" => "file1.txt", "content" => "first file", "content_type" => "text/plain", "content_encoding" => "text"], ["field_name" => "files", "filename" => "file2.txt", "content" => "second file", "content_type" => "text/plain", "content_encoding" => "text"]], 'data' => ["tags" => ["python", "rust", "web"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["files" => [["filename" => "file1.txt", "size" => 10, "content" => "first file", "content_type" => "text/plain"], ["filename" => "file2.txt", "size" => 11, "content" => "second file", "content_type" => "text/plain"]], "tags" => ["python", "rust", "web"]];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_optional_file_upload_missing(): void
    {
        $app = AppFactory::create_multipart_optional_file_upload_missing_18();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/optional', ['files' => [], 'form_data' => (object)[]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["file" => null];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_optional_file_upload_provided(): void
    {
        $app = AppFactory::create_multipart_optional_file_upload_provided_19();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/optional', ['files' => [["field_name" => "file", "filename" => "optional.txt", "content" => "optional file content", "content_type" => "text/plain"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["filename" => "optional.txt", "content_type" => "text/plain", "size" => 21];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_pdf_file_upload(): void
    {
        $app = AppFactory::create_multipart_pdf_file_upload_20();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/document', ['files' => [["field_name" => "document", "filename" => "report.pdf", "content" => "fake_pdf_content", "content_type" => "application/pdf"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["filename" => "report.pdf", "content_type" => "application/pdf", "size" => 16];
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_required_file_upload_missing(): void
    {
        $app = AppFactory::create_multipart_required_file_upload_missing_21();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/required', ['files' => [], 'form_data' => (object)[]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["body", "file"], "msg" => "Field required", "input" => []]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_multipart_simple_file_upload(): void
    {
        $app = AppFactory::create_multipart_simple_file_upload_22();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['headers' => ["Content-Type" => "multipart/form-data"], 'files' => [["field_name" => "test", "filename" => "test.txt", "content" => "<file content>", "content_type" => "text/plain", "content_encoding" => "text"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["test" => ["filename" => "test.txt", "size" => 14, "content" => "<file content>", "content_type" => "text/plain"]];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_20_uuid_v3_path_param_success(): void
    {
        $app = AppFactory::create_path_params_20_uuid_v3_path_param_success_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/e8b5a51d-11c8-3310-a6ab-367563f20686', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => "e8b5a51d-11c8-3310-a6ab-367563f20686"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_21_uuid_v5_path_param_success(): void
    {
        $app = AppFactory::create_path_params_21_uuid_v5_path_param_success_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/630eb68f-e0fa-5ecc-887a-7c7a62614681', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => "630eb68f-e0fa-5ecc-887a-7c7a62614681"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_24_date_format_path_param_success(): void
    {
        $app = AppFactory::create_path_params_24_date_format_path_param_success_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/events/2025-10-30', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["date" => "2025-10-30"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_25_date_format_invalid_failure(): void
    {
        $app = AppFactory::create_path_params_25_date_format_invalid_failure_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/events/2025-13-45', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["path", "date"], "msg" => "Invalid date format", "ctx" => ["format" => "date", "value" => "2025-13-45"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_27_datetime_format_path_param_success(): void
    {
        $app = AppFactory::create_path_params_27_datetime_format_path_param_success_5();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/bookings/2025-10-30T14:30:00Z', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["timestamp" => "2025-10-30T14:30:00Z"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_28_duration_format_path_param_success(): void
    {
        $app = AppFactory::create_path_params_28_duration_format_path_param_success_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/delays/P1DT2H30M', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["duration" => "P1DT2H30M"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_29_decimal_path_param_success(): void
    {
        $app = AppFactory::create_path_params_29_decimal_path_param_success_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/prices/19.99', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["amount" => "19.99"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_30_string_minlength_path_success(): void
    {
        $app = AppFactory::create_path_params_30_string_minlength_path_success_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/alice', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "alice"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_31_string_minlength_path_failure(): void
    {
        $app = AppFactory::create_path_params_31_string_minlength_path_failure_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/ab', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["path", "username"], "msg" => "String length must be at least 3", "ctx" => ["min_length" => 3, "actual_length" => 2]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_32_string_maxlength_path_failure(): void
    {
        $app = AppFactory::create_path_params_32_string_maxlength_path_failure_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/this_username_is_way_too_long_to_be_valid', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["path", "username"], "msg" => "String length must not exceed 20", "ctx" => ["max_length" => 20, "actual_length" => 42]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_33_string_pattern_path_success(): void
    {
        $app = AppFactory::create_path_params_33_string_pattern_path_success_11();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/repos/spikard-labs/spikard-http', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["owner" => "spikard-labs", "repo" => "spikard-http"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_34_string_pattern_path_failure(): void
    {
        $app = AppFactory::create_path_params_34_string_pattern_path_failure_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/repos/invalid@owner', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["path", "owner"], "msg" => "String does not match pattern", "ctx" => ["pattern" => "^[a-zA-Z0-9-]+\$", "value" => "invalid@owner"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_35_negative_integer_path_param(): void
    {
        $app = AppFactory::create_path_params_35_negative_integer_path_param_13();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/offset/-100', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["value" => -100];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_boolean_path_parameter_true(): void
    {
        $app = AppFactory::create_path_params_boolean_path_parameter_true_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/bool/True', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_boolean_path_parameter_numeric_1(): void
    {
        $app = AppFactory::create_path_params_boolean_path_parameter_numeric_1_15();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/bool/1', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_date_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params_date_path_parameter_success_16();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/date/2023-07-15', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["date_param" => "2023-07-15"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_enum_path_parameter_invalid_value(): void
    {
        $app = AppFactory::create_path_params_enum_path_parameter_invalid_value_17();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/models/foo', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "enum", "loc" => ["path", "model_name"], "msg" => "Input should be 'alexnet', 'resnet' or 'lenet'", "input" => "foo", "ctx" => ["expected" => "'alexnet', 'resnet' or 'lenet'"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_enum_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params_enum_path_parameter_success_18();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/models/alexnet', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["model_name" => "alexnet"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_float_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params_float_path_parameter_success_19();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/float/42.5', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => 42.5];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_integer_path_parameter_invalid_string(): void
    {
        $app = AppFactory::create_path_params_integer_path_parameter_invalid_string_20();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/int/foobar', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "int_parsing", "loc" => ["path", "item_id"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "input" => "foobar"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_integer_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params_integer_path_parameter_success_21();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/int/42', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => 42];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success(): void
    {
        $app = AppFactory::create_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success_22();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-lt-gt/2', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => 2];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_integer_path_parameter_with_ge_constraint_success(): void
    {
        $app = AppFactory::create_path_params_integer_path_parameter_with_ge_constraint_success_23();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-ge/3', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => 3];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_integer_path_parameter_with_gt_constraint_failure(): void
    {
        $app = AppFactory::create_path_params_integer_path_parameter_with_gt_constraint_failure_24();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-gt/2', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "greater_than", "loc" => ["path", "item_id"], "msg" => "Input should be greater than 3", "input" => 2, "ctx" => ["gt" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_integer_path_parameter_with_gt_constraint_success(): void
    {
        $app = AppFactory::create_path_params_integer_path_parameter_with_gt_constraint_success_25();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-gt/42', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => 42];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_integer_path_parameter_with_le_constraint_success(): void
    {
        $app = AppFactory::create_path_params_integer_path_parameter_with_le_constraint_success_26();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-le/3', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => 3];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_integer_path_parameter_with_lt_constraint_success(): void
    {
        $app = AppFactory::create_path_params_integer_path_parameter_with_lt_constraint_success_27();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-lt/2', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => 2];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_multiple_path_parameters_success(): void
    {
        $app = AppFactory::create_path_params_multiple_path_parameters_success_28();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["version" => 1.0, "service_id" => 1, "user_id" => "abc", "order_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_path_parameter_type_syntax_invalid_uuid(): void
    {
        $app = AppFactory::create_path_params_path_parameter_type_syntax_invalid_uuid_29();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/type-syntax/items/not-a-uuid', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "uuid_parsing", "loc" => ["path", "id"], "msg" => "Input should be a valid UUID", "input" => "not-a-uuid"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_path_parameter_type_syntax_with_override(): void
    {
        $app = AppFactory::create_path_params_path_parameter_type_syntax_with_override_30();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/type-syntax/items-count/50', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["count" => "50"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_path_parameter_with_type_syntax_uuid(): void
    {
        $app = AppFactory::create_path_params_path_parameter_with_type_syntax_uuid_31();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/type-syntax/items/550e8400-e29b-41d4-a716-446655440000', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => "550e8400-e29b-41d4-a716-446655440000"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_path_parameter_with_type_syntax_integer(): void
    {
        $app = AppFactory::create_path_params_path_parameter_with_type_syntax_integer_32();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/type-syntax/users/42', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["user_id" => "42"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_path_type_parameter_file_path(): void
    {
        $app = AppFactory::create_path_params_path_type_parameter_file_path_33();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/files/home/johndoe/myfile.txt', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["file_path" => "home/johndoe/myfile.txt"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_string_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params_string_path_parameter_success_34();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/str/foobar', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => "foobar"];
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_string_path_parameter_with_max_length_failure(): void
    {
        $app = AppFactory::create_path_params_string_path_parameter_with_max_length_failure_35();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-maxlength/foobar', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_long", "loc" => ["path", "item_id"], "msg" => "String should have at most 3 characters", "input" => "foobar", "ctx" => ["max_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_string_path_parameter_with_min_length_failure(): void
    {
        $app = AppFactory::create_path_params_string_path_parameter_with_min_length_failure_36();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-minlength/fo', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_short", "loc" => ["path", "item_id"], "msg" => "String should have at least 3 characters", "input" => "fo", "ctx" => ["min_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_path_params_uuid_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params_uuid_path_parameter_success_37();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => "ec38df32-ceda-4cfa-9b4a-1aeb94ad551a"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_42_negative_integer_query_param(): void
    {
        $app = AppFactory::create_query_params_42_negative_integer_query_param_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/negative?offset=-10', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["offset" => -10];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_43_scientific_notation_float(): void
    {
        $app = AppFactory::create_query_params_43_scientific_notation_float_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/stats?threshold=1.5e-3', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["threshold" => 0.0015];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_44_string_minlength_validation_success(): void
    {
        $app = AppFactory::create_query_params_44_string_minlength_validation_success_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=foo', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["term" => "foo"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_45_string_minlength_validation_failure(): void
    {
        $app = AppFactory::create_query_params_45_string_minlength_validation_failure_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=ab', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "term"], "msg" => "String length must be at least 3", "ctx" => ["min_length" => 3, "actual_length" => 2]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_46_string_maxlength_validation_failure(): void
    {
        $app = AppFactory::create_query_params_46_string_maxlength_validation_failure_5();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=this_is_way_too_long', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "term"], "msg" => "String length must not exceed 10", "ctx" => ["max_length" => 10, "actual_length" => 21]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_47_pattern_validation_email_success(): void
    {
        $app = AppFactory::create_query_params_47_pattern_validation_email_success_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/subscribe?email=user%40example.com', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["email" => "user@example.com"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_48_pattern_validation_email_failure(): void
    {
        $app = AppFactory::create_query_params_48_pattern_validation_email_failure_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/subscribe?email=invalid-email', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "email"], "msg" => "String does not match pattern", "ctx" => ["pattern" => "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}\$", "value" => "invalid-email"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_49_integer_gt_constraint_success(): void
    {
        $app = AppFactory::create_query_params_49_integer_gt_constraint_success_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?limit=5', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["limit" => 5];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_50_integer_gt_constraint_failure(): void
    {
        $app = AppFactory::create_query_params_50_integer_gt_constraint_failure_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?limit=0', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "limit"], "msg" => "Value must be greater than 0", "ctx" => ["exclusive_minimum" => 0, "value" => 0]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_51_integer_ge_constraint_boundary(): void
    {
        $app = AppFactory::create_query_params_51_integer_ge_constraint_boundary_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?offset=0', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["offset" => 0];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_52_integer_le_constraint_boundary(): void
    {
        $app = AppFactory::create_query_params_52_integer_le_constraint_boundary_11();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?limit=100', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["limit" => 100];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_53_integer_le_constraint_failure(): void
    {
        $app = AppFactory::create_query_params_53_integer_le_constraint_failure_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?limit=101', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "limit"], "msg" => "Value must not exceed 100", "ctx" => ["maximum" => 100, "value" => 101]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_54_array_minitems_constraint_success(): void
    {
        $app = AppFactory::create_query_params_54_array_minitems_constraint_success_13();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?ids=1&ids=2&ids=3', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["ids" => [1, 2, 3]];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_55_array_minitems_constraint_failure(): void
    {
        $app = AppFactory::create_query_params_55_array_minitems_constraint_failure_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?ids=1', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "ids"], "msg" => "Array must contain at least 2 items", "ctx" => ["min_items" => 2, "actual_items" => 1]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_56_array_maxitems_constraint_failure(): void
    {
        $app = AppFactory::create_query_params_56_array_maxitems_constraint_failure_15();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?tags=a&tags=b&tags=c&tags=d&tags=e&tags=f', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "tags"], "msg" => "Array must not contain more than 5 items", "ctx" => ["max_items" => 5, "actual_items" => 6]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_57_boolean_empty_string_coercion(): void
    {
        $app = AppFactory::create_query_params_57_boolean_empty_string_coercion_16();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?active=', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["active" => false];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_58_format_email_success(): void
    {
        $app = AppFactory::create_query_params_58_format_email_success_17();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/subscribe?email=user%40example.com', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["email" => "user@example.com"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_59_format_email_failure(): void
    {
        $app = AppFactory::create_query_params_59_format_email_failure_18();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/subscribe?email=not-an-email', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "email"], "msg" => "Invalid email format", "ctx" => ["format" => "email", "value" => "not-an-email"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_60_format_ipv4_success(): void
    {
        $app = AppFactory::create_query_params_60_format_ipv4_success_19();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/network?ip=192.168.1.1', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["ip" => "192.168.1.1"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_61_format_ipv4_failure(): void
    {
        $app = AppFactory::create_query_params_61_format_ipv4_failure_20();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/network?ip=999.999.999.999', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "ip"], "msg" => "Invalid IPv4 address format", "ctx" => ["format" => "ipv4", "value" => "999.999.999.999"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_62_format_ipv6_success(): void
    {
        $app = AppFactory::create_query_params_62_format_ipv6_success_21();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/network/ipv6?ip=2001%3A0db8%3A85a3%3A0000%3A0000%3A8a2e%3A0370%3A7334', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["ip" => "2001:0db8:85a3:0000:0000:8a2e:0370:7334"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_63_format_uri_success(): void
    {
        $app = AppFactory::create_query_params_63_format_uri_success_22();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/redirect?url=https%3A%2F%2Fexample.com%2Fpath%3Fquery%3Dvalue', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["url" => "https://example.com/path?query=value"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_64_format_uri_failure(): void
    {
        $app = AppFactory::create_query_params_64_format_uri_failure_23();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/redirect?url=not%20a%20uri', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "url"], "msg" => "Invalid URI format", "ctx" => ["format" => "uri", "value" => "not a uri"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_65_format_hostname_success(): void
    {
        $app = AppFactory::create_query_params_65_format_hostname_success_24();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/dns?host=api.example.com', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["host" => "api.example.com"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_66_multipleof_constraint_success(): void
    {
        $app = AppFactory::create_query_params_66_multipleof_constraint_success_25();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?quantity=15', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["quantity" => 15];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_67_multipleof_constraint_failure(): void
    {
        $app = AppFactory::create_query_params_67_multipleof_constraint_failure_26();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?quantity=17', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "quantity"], "msg" => "Value must be a multiple of 5", "ctx" => ["multiple_of" => 5, "value" => 17]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_68_array_uniqueitems_success(): void
    {
        $app = AppFactory::create_query_params_68_array_uniqueitems_success_27();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?ids=1&ids=2&ids=3&ids=4', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["ids" => [1, 2, 3, 4]];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_69_array_uniqueitems_failure(): void
    {
        $app = AppFactory::create_query_params_69_array_uniqueitems_failure_28();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?ids=1&ids=2&ids=2&ids=3', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["query", "ids"], "msg" => "Array items must be unique", "ctx" => ["unique_items" => true, "duplicate_value" => 2, "duplicate_index" => 2]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_70_array_separator_pipe(): void
    {
        $app = AppFactory::create_query_params_70_array_separator_pipe_29();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?tags=python%7Crust%7Ctypescript&tags=python%7Crust%7Ctypescript', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["tags" => ["python", "rust", "typescript"]];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_71_array_separator_semicolon(): void
    {
        $app = AppFactory::create_query_params_71_array_separator_semicolon_30();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?colors=red%3Bgreen%3Bblue&colors=red%3Bgreen%3Bblue', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["colors" => ["red", "green", "blue"]];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_72_array_separator_space(): void
    {
        $app = AppFactory::create_query_params_72_array_separator_space_31();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?keywords=rust%20web%20framework&keywords=rust%20web%20framework', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["keywords" => ["rust", "web", "framework"]];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_array_query_parameter_empty_array(): void
    {
        $app = AppFactory::create_query_params_array_query_parameter_empty_array_32();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list-default', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = [];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_array_query_parameter_single_value(): void
    {
        $app = AppFactory::create_query_params_array_query_parameter_single_value_33();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list-default?tags=apple', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["apple"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_boolean_query_parameter_numeric_1(): void
    {
        $app = AppFactory::create_query_params_boolean_query_parameter_numeric_1_34();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/bool?flag=1', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["flag" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_boolean_query_parameter_true(): void
    {
        $app = AppFactory::create_query_params_boolean_query_parameter_true_35();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/bool?flag=true', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["flag" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_date_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params_date_query_parameter_success_36();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/date?event_date=2024-01-15', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["event_date" => "2024-01-15"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_datetime_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params_datetime_query_parameter_success_37();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/datetime?timestamp=2024-01-15T10%3A30%3A00Z', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["timestamp" => "2024-01-15T10:30:00Z"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_enum_query_parameter_invalid_value(): void
    {
        $app = AppFactory::create_query_params_enum_query_parameter_invalid_value_38();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/enum?model=vgg16', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "enum", "loc" => ["query", "model"], "msg" => "Input should be 'alexnet', 'resnet' or 'lenet'", "input" => "vgg16", "ctx" => ["expected" => "'alexnet', 'resnet' or 'lenet'"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_enum_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params_enum_query_parameter_success_39();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/enum?model=alexnet', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["model" => "alexnet"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_float_query_param_with_ge_constraint_success(): void
    {
        $app = AppFactory::create_query_params_float_query_param_with_ge_constraint_success_40();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/float-ge?price=0.01', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["price" => 0.01];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_integer_query_param_with_ge_constraint_boundary(): void
    {
        $app = AppFactory::create_query_params_integer_query_param_with_ge_constraint_boundary_41();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int-ge?value=10', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["value" => 10];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_integer_query_param_with_gt_constraint_valid(): void
    {
        $app = AppFactory::create_query_params_integer_query_param_with_gt_constraint_valid_42();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int-gt?value=1', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["value" => 1];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_integer_query_param_with_le_constraint_boundary(): void
    {
        $app = AppFactory::create_query_params_integer_query_param_with_le_constraint_boundary_43();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int-le?value=100', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["value" => 100];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_integer_query_param_with_lt_constraint_valid(): void
    {
        $app = AppFactory::create_query_params_integer_query_param_with_lt_constraint_valid_44();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int-lt?value=49', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["value" => 49];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_integer_with_default_value_not_provided(): void
    {
        $app = AppFactory::create_query_params_integer_with_default_value_not_provided_45();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int/default', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("foo bar 10", $body);
    }

    public function test_query_params_integer_with_default_value_override(): void
    {
        $app = AppFactory::create_query_params_integer_with_default_value_override_46();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int/default?query=50', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("foo bar 50", $body);
    }

    public function test_query_params_list_of_integers_multiple_values(): void
    {
        $app = AppFactory::create_query_params_list_of_integers_multiple_values_47();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list?device_ids=1&device_ids=2', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = [1, 2];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_list_of_strings_multiple_values(): void
    {
        $app = AppFactory::create_query_params_list_of_strings_multiple_values_48();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=foo&q=bar', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["q" => ["foo", "bar"]];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_list_query_parameter_required_but_missing(): void
    {
        $app = AppFactory::create_query_params_list_query_parameter_required_but_missing_49();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["query", "device_ids"], "msg" => "Field required", "input" => null]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_list_with_default_empty_array_no_values_provided(): void
    {
        $app = AppFactory::create_query_params_list_with_default_empty_array_no_values_provided_50();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list-default', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = [];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_multiple_query_parameters_with_different_types(): void
    {
        $app = AppFactory::create_query_params_multiple_query_parameters_with_different_types_51();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/multi-type?active=true&age=30&name=john&score=95.5', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "john", "age" => 30, "active" => true, "score" => 95.5];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_optional_integer_query_parameter_missing(): void
    {
        $app = AppFactory::create_query_params_optional_integer_query_parameter_missing_52();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int/optional', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("foo bar None", $body);
    }

    public function test_query_params_optional_query_parameter_with_default_value(): void
    {
        $app = AppFactory::create_query_params_optional_query_parameter_with_default_value_53();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/optional-default', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["limit" => 10];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_optional_string_query_parameter_missing(): void
    {
        $app = AppFactory::create_query_params_optional_string_query_parameter_missing_54();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/optional', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("foo bar None", $body);
    }

    public function test_query_params_optional_string_query_parameter_provided(): void
    {
        $app = AppFactory::create_query_params_optional_string_query_parameter_provided_55();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/optional?query=baz', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("foo bar baz", $body);
    }

    public function test_query_params_query_parameter_with_url_encoded_space(): void
    {
        $app = AppFactory::create_query_params_query_parameter_with_url_encoded_space_56();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/basic?name=hello%20world', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "hello world"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_query_parameter_with_url_encoded_special_characters(): void
    {
        $app = AppFactory::create_query_params_query_parameter_with_url_encoded_special_characters_57();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/basic?name=test%26value%3D123', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "test&value=123"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_query_parameter_with_special_characters_url_encoding(): void
    {
        $app = AppFactory::create_query_params_query_parameter_with_special_characters_url_encoding_58();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/test?email=x%40test.com&special=%26%40A.ac', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["email" => "x@test.com", "special" => "&@A.ac"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_required_integer_query_parameter_float_value(): void
    {
        $app = AppFactory::create_query_params_required_integer_query_parameter_float_value_59();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int?query=42.5', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "int_parsing", "loc" => ["query", "query"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "input" => 42.5]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_required_integer_query_parameter_invalid_type(): void
    {
        $app = AppFactory::create_query_params_required_integer_query_parameter_invalid_type_60();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int?query=baz', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "int_parsing", "loc" => ["query", "query"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "input" => "baz"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_required_integer_query_parameter_missing(): void
    {
        $app = AppFactory::create_query_params_required_integer_query_parameter_missing_61();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["query", "query"], "msg" => "Field required", "input" => null]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_required_integer_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params_required_integer_query_parameter_success_62();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int?query=42', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("foo bar 42", $body);
    }

    public function test_query_params_required_string_query_parameter_missing(): void
    {
        $app = AppFactory::create_query_params_required_string_query_parameter_missing_63();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["query", "query"], "msg" => "Field required", "input" => null]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_required_string_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params_required_string_query_parameter_success_64();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query?query=baz', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("foo bar baz", $body);
    }

    public function test_query_params_string_query_param_with_max_length_constraint_fail(): void
    {
        $app = AppFactory::create_query_params_string_query_param_with_max_length_constraint_fail_65();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/str-max-length?name=this_is_way_too_long', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_long", "loc" => ["query", "name"], "msg" => "String should have at most 10 characters", "input" => "this_is_way_too_long", "ctx" => ["max_length" => 10]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_string_query_param_with_min_length_constraint_fail(): void
    {
        $app = AppFactory::create_query_params_string_query_param_with_min_length_constraint_fail_66();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/str-min-length?name=ab', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_short", "loc" => ["query", "name"], "msg" => "String should have at least 3 characters", "input" => "ab", "ctx" => ["min_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_string_query_param_with_regex_pattern_fail(): void
    {
        $app = AppFactory::create_query_params_string_query_param_with_regex_pattern_fail_67();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/pattern?code=abc123', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["query", "code"], "msg" => "String should match pattern '^[0-9]{3,}\$'", "input" => "abc123", "ctx" => ["pattern" => "^[0-9]{3,}\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_string_validation_with_regex_failure(): void
    {
        $app = AppFactory::create_query_params_string_validation_with_regex_failure_68();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?item_query=nonregexquery', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["query", "item_query"], "msg" => "String should match pattern '^fixedquery\$'", "input" => "nonregexquery", "ctx" => ["pattern" => "^fixedquery\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_string_validation_with_regex_success(): void
    {
        $app = AppFactory::create_query_params_string_validation_with_regex_success_69();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?item_query=fixedquery', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_query" => "fixedquery"];
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_uuid_query_parameter_invalid_format(): void
    {
        $app = AppFactory::create_query_params_uuid_query_parameter_invalid_format_70();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/uuid?item_id=not-a-uuid', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "uuid_parsing", "loc" => ["query", "item_id"], "msg" => "Input should be a valid UUID", "input" => "not-a-uuid"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_query_params_uuid_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params_uuid_query_parameter_success_71();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/uuid?item_id=c892496f-b1fd-4b91-bdb8-b46f92df1716', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"];
        $this->assertEquals($expected, $body);
    }

    public function test_rate_limit_rate_limit_below_threshold_succeeds(): void
    {
        $app = AppFactory::create_rate_limit_rate_limit_below_threshold_succeeds_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/rate-limit/basic', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["status" => "ok", "request" => "under-limit"];
        $this->assertEquals($expected, $body);
    }

    public function test_rate_limit_rate_limit_exceeded_returns_429(): void
    {
        $app = AppFactory::create_rate_limit_rate_limit_exceeded_returns_429_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/rate-limit/exceeded', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(429, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_request_id_request_id_header_is_preserved(): void
    {
        $app = AppFactory::create_request_id_request_id_header_is_preserved_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/request-id/preserved', ['headers' => ["X-Request-ID" => "trace-123"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["status" => "preserved", "echo" => "trace-123"];
        $this->assertEquals($expected, $body);
    }

    public function test_request_id_request_id_is_generated_when_not_provided(): void
    {
        $app = AppFactory::create_request_id_request_id_is_generated_when_not_provided_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/request-id/generated', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["status" => "generated"];
        $this->assertEquals($expected, $body);
    }

    public function test_request_id_request_id_middleware_can_be_disabled(): void
    {
        $app = AppFactory::create_request_id_request_id_middleware_can_be_disabled_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/request-id/disabled', ['headers' => ["X-Request-ID" => "external-id"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["status" => "no-request-id"];
        $this->assertEquals($expected, $body);
    }

    public function test_request_timeout_request_completes_before_timeout(): void
    {
        $app = AppFactory::create_request_timeout_request_completes_before_timeout_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/timeouts/fast', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["status" => "ok", "duration" => "fast"];
        $this->assertEquals($expected, $body);
    }

    public function test_request_timeout_request_exceeds_timeout(): void
    {
        $app = AppFactory::create_request_timeout_request_exceeds_timeout_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/timeouts/slow', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(408, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_static_files_static_file_server_returns_text_file(): void
    {
        $app = AppFactory::create_static_files_static_file_server_returns_text_file_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/public/hello.txt', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("Hello from static storage", $body);
    }

    public function test_static_files_static_server_returns_index_html_for_directory(): void
    {
        $app = AppFactory::create_static_files_static_server_returns_index_html_for_directory_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/app/', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("<!doctype html><h1>Welcome</h1>", $body);
    }

    public function test_status_codes_19_413_payload_too_large(): void
    {
        $app = AppFactory::create_status_codes_19_413_payload_too_large_1();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['body' => ["data" => "{{ repeat 'x' 2000 times }}"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(413, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Payload Too Large", "message" => "Request body size exceeds maximum allowed size of 1024 bytes"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_200_ok_success(): void
    {
        $app = AppFactory::create_status_codes_200_ok_success_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/status-test/200', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 1, "name" => "Item 1"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_201_created_resource_created(): void
    {
        $app = AppFactory::create_status_codes_201_created_resource_created_3();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "New Item"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["id" => 1, "name" => "New Item"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_202_accepted_request_accepted_for_processing(): void
    {
        $app = AppFactory::create_status_codes_202_accepted_request_accepted_for_processing_4();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/tasks/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["task" => "process_data"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(202, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["message" => "Task accepted for processing", "task_id" => "abc123"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_204_no_content_success_with_no_body(): void
    {
        $app = AppFactory::create_status_codes_204_no_content_success_with_no_body_5();
        $client = TestClient::create($app);
        $response = $client->request('DELETE', '/status-test/204', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(204, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_status_codes_206_partial_content(): void
    {
        $app = AppFactory::create_status_codes_206_partial_content_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/files/document.pdf', ['headers' => ["Range" => "bytes=0-1023"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(206, $statusCode);

        $body = $response->body;
        $this->assertEquals("binary_data_1024_bytes", $body);
    }

    public function test_status_codes_20_414_uri_too_long(): void
    {
        $app = AppFactory::create_status_codes_20_414_uri_too_long_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data?skip_template_expansion=true', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = [];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_21_431_request_header_fields_too_large(): void
    {
        $app = AppFactory::create_status_codes_21_431_request_header_fields_too_large_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data', ['headers' => ["X-Large-Header" => "{{ repeat 'x' 10000 times }}"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(431, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Request Header Fields Too Large", "message" => "Request headers exceed maximum allowed size of 8192 bytes"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_22_501_not_implemented(): void
    {
        $app = AppFactory::create_status_codes_22_501_not_implemented_9();
        $client = TestClient::create($app);
        $response = $client->request('TRACE', '/data', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(405, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_status_codes_23_503_service_unavailable(): void
    {
        $app = AppFactory::create_status_codes_23_503_service_unavailable_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(503, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["error" => "Service Unavailable", "message" => "The service is temporarily unavailable. Please try again later."];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_301_moved_permanently_permanent_redirect(): void
    {
        $app = AppFactory::create_status_codes_301_moved_permanently_permanent_redirect_11();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/old-path', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(301, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_status_codes_302_found_temporary_redirect(): void
    {
        $app = AppFactory::create_status_codes_302_found_temporary_redirect_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/temp-redirect', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(302, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_status_codes_304_not_modified_cached_content_valid(): void
    {
        $app = AppFactory::create_status_codes_304_not_modified_cached_content_valid_13();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/status-test/304', ['headers' => ["If-None-Match" => "\"abc123\""]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(304, $statusCode);

        $body = $response->body;
        $this->assertEquals(null, $body);
    }

    public function test_status_codes_307_temporary_redirect_method_preserved(): void
    {
        $app = AppFactory::create_status_codes_307_temporary_redirect_method_preserved_14();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/redirect-post', ['headers' => ["Content-Type" => "application/json"], 'body' => (object)[]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(307, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = [];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_400_bad_request_invalid_request(): void
    {
        $app = AppFactory::create_status_codes_400_bad_request_invalid_request_15();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => "not valid json"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(400, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "Invalid request format"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_401_unauthorized_missing_authentication(): void
    {
        $app = AppFactory::create_status_codes_401_unauthorized_missing_authentication_16();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(401, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "Not authenticated"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_403_forbidden_insufficient_permissions(): void
    {
        $app = AppFactory::create_status_codes_403_forbidden_insufficient_permissions_17();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/admin/users', ['headers' => ["Authorization" => "Bearer valid_token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(403, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "Not enough permissions"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_404_not_found_resource_not_found(): void
    {
        $app = AppFactory::create_status_codes_404_not_found_resource_not_found_18();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/status-test/404', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(404, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "Item not found"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_408_request_timeout(): void
    {
        $app = AppFactory::create_status_codes_408_request_timeout_19();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/slow-endpoint', ['headers' => ["Content-Type" => "application/json"], 'body' => ["data" => "large_data"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(408, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "Request timeout"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_422_unprocessable_entity_validation_error(): void
    {
        $app = AppFactory::create_status_codes_422_unprocessable_entity_validation_error_20();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["price" => "not a number"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["body", "name"], "msg" => "Field required", "input" => ["price" => "not a number"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_429_too_many_requests(): void
    {
        $app = AppFactory::create_status_codes_429_too_many_requests_21();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/resource', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(429, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "Rate limit exceeded. Try again in 60 seconds."];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_500_internal_server_error_server_error(): void
    {
        $app = AppFactory::create_status_codes_500_internal_server_error_server_error_22();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/error', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(500, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["type" => "https://spikard.dev/errors/internal-server-error", "title" => "Internal Server Error", "status" => 500, "detail" => "Internal server error"];
        $this->assertEquals($expected, $body);
    }

    public function test_status_codes_503_service_unavailable_server_overload(): void
    {
        $app = AppFactory::create_status_codes_503_service_unavailable_server_overload_23();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/health', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(503, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "Service temporarily unavailable"];
        $this->assertEquals($expected, $body);
    }

    public function test_streaming_binary_log_download(): void
    {
        $app = AppFactory::create_streaming_binary_log_download_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/stream/logfile', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("LOG:\\u0000\\u0001\\u0002\\u0003|TAIL|\\u0007\\n", $body);
    }

    public function test_streaming_chunked_csv_export(): void
    {
        $app = AppFactory::create_streaming_chunked_csv_export_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/stream/csv-report', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("id,name,value\\n1,Alice,42\\n2,Bob,7\\n", $body);
    }

    public function test_streaming_stream_json_lines(): void
    {
        $app = AppFactory::create_streaming_stream_json_lines_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/stream/json-lines', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        $this->assertEquals("{\"index\":0,\"payload\":\"alpha\"}\\n{\"index\":1,\"payload\":\"beta\"}\\n{\"index\":2,\"payload\":\"gamma\"}\\n", $body);
    }

    public function test_url_encoded_13_array_field_success(): void
    {
        $app = AppFactory::create_url_encoded_13_array_field_success_1();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/register', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "tags[]=python&tags[]=rust&tags[]=typescript"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["tags" => ["python", "rust", "typescript"]];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_14_nested_object_bracket_notation(): void
    {
        $app = AppFactory::create_url_encoded_14_nested_object_bracket_notation_2();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/profile', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "user[name]=John%20Doe&user[email]=john@example.com&user[age]=30"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["user" => ["name" => "John Doe", "email" => "john@example.com", "age" => 30]];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_15_special_characters_field_names(): void
    {
        $app = AppFactory::create_url_encoded_15_special_characters_field_names_3();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "user-name=JohnDoe&contact.email=john%40example.com"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(201, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["user-name" => "JohnDoe", "contact.email" => "john@example.com"];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_16_minlength_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded_16_minlength_validation_failure_4();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "username=ab"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_short", "loc" => ["body", "username"], "msg" => "String should have at least 3 characters", "input" => "ab", "ctx" => ["min_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_17_pattern_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded_17_pattern_validation_failure_5();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/accounts', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "account_id=INVALID123"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["body", "account_id"], "msg" => "String should match pattern '^ACC-[0-9]{6}\$'", "input" => "INVALID123", "ctx" => ["pattern" => "^ACC-[0-9]{6}\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_18_integer_minimum_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded_18_integer_minimum_validation_failure_6();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/products', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "quantity=0"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "greater_than_equal", "loc" => ["body", "quantity"], "msg" => "Input should be greater than or equal to 1", "input" => 0, "ctx" => ["ge" => 1]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_19_array_minitems_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded_19_array_minitems_validation_failure_7();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/tags', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "tags[]=single"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "too_short", "loc" => ["body", "tags"], "msg" => "List should have at least 2 item after validation", "input" => ["single"], "ctx" => ["min_length" => 2]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_20_format_email_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded_20_format_email_validation_failure_8();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/subscribe', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "email=not-an-email"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["body", "email"], "msg" => "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+\$'", "input" => "not-an-email", "ctx" => ["pattern" => "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_21_integer_type_coercion_failure(): void
    {
        $app = AppFactory::create_url_encoded_21_integer_type_coercion_failure_9();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/products', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "price=not-a-number"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "int_parsing", "loc" => ["body", "price"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "input" => "not-a-number"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_22_additional_properties_strict_failure(): void
    {
        $app = AppFactory::create_url_encoded_22_additional_properties_strict_failure_10();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/settings', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'body' => "theme=dark&unknown_field=value"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "validation_error", "loc" => ["body", "unknown_field"], "msg" => "Additional properties are not allowed", "input" => ["theme" => "dark", "unknown_field" => "value"], "ctx" => ["additional_properties" => false, "unexpected_field" => "unknown_field"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_boolean_field_conversion(): void
    {
        $app = AppFactory::create_url_encoded_boolean_field_conversion_11();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["username" => "johndoe", "subscribe" => "true"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "johndoe", "subscribe" => true];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_empty_string_value(): void
    {
        $app = AppFactory::create_url_encoded_empty_string_value_12();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["username" => "johndoe", "description" => ""]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "johndoe", "description" => ""];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_multiple_values_for_same_field(): void
    {
        $app = AppFactory::create_url_encoded_multiple_values_for_same_field_13();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/tags', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["tags" => ["python", "fastapi", "web"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["tags" => ["python", "fastapi", "web"]];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_numeric_field_type_conversion(): void
    {
        $app = AppFactory::create_url_encoded_numeric_field_type_conversion_14();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["age" => "30", "username" => "johndoe"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "johndoe", "age" => 30];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_oauth2_password_grant_flow(): void
    {
        $app = AppFactory::create_url_encoded_oauth2_password_grant_flow_15();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/token', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["scope" => "", "password" => "secret", "username" => "johndoe", "grant_type" => "password"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["access_token" => "johndoe", "token_type" => "bearer"];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_optional_field_missing_success(): void
    {
        $app = AppFactory::create_url_encoded_optional_field_missing_success_16();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/register/', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["username" => "johndoe", "password" => "secret"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "johndoe", "email" => null];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_pattern_validation_fail(): void
    {
        $app = AppFactory::create_url_encoded_pattern_validation_fail_17();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/validated', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["username" => "john doe"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["body", "username"], "msg" => "String should match pattern '^[a-z0-9_]+\$'", "input" => "john doe", "ctx" => ["pattern" => "^[a-z0-9_]+\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_required_field_missing_validation_error(): void
    {
        $app = AppFactory::create_url_encoded_required_field_missing_validation_error_18();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/login/', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["password" => "secret"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["body", "username"], "msg" => "Field required", "input" => ["password" => "secret"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_simple_form_submission_success(): void
    {
        $app = AppFactory::create_url_encoded_simple_form_submission_success_19();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/login/', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["username" => "johndoe", "password" => "secret"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["username" => "johndoe"];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_special_characters_encoding(): void
    {
        $app = AppFactory::create_url_encoded_special_characters_encoding_20();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["description" => "Test & Development", "name" => "John Doe"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(200, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["name" => "John Doe", "description" => "Test & Development"];
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_string_max_length_validation_fail(): void
    {
        $app = AppFactory::create_url_encoded_string_max_length_validation_fail_21();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/validated', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["username" => "this_is_a_very_long_username_that_exceeds_limit"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_long", "loc" => ["body", "username"], "msg" => "String should have at most 20 characters", "input" => "this_is_a_very_long_username_that_exceeds_limit", "ctx" => ["max_length" => 20]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_url_encoded_string_min_length_validation_fail(): void
    {
        $app = AppFactory::create_url_encoded_string_min_length_validation_fail_22();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/validated', ['headers' => ["Content-Type" => "application/x-www-form-urlencoded"], 'form_data' => ["username" => "ab"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_short", "loc" => ["body", "username"], "msg" => "String should have at least 3 characters", "input" => "ab", "ctx" => ["min_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_09_multiple_validation_errors(): void
    {
        $app = AppFactory::create_validation_errors_09_multiple_validation_errors_1();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ["name" => "ab", "email" => "invalid-email", "age" => 15]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "3 validation errors in request", "errors" => [["type" => "greater_than_equal", "loc" => ["body", "age"], "msg" => "Input should be greater than or equal to 18", "input" => 15, "ctx" => ["ge" => 18]], ["type" => "string_pattern_mismatch", "loc" => ["body", "email"], "msg" => "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+\$'", "input" => "invalid-email", "ctx" => ["pattern" => "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+\$"]], ["type" => "string_too_short", "loc" => ["body", "name"], "msg" => "String should have at least 3 characters", "input" => "ab", "ctx" => ["min_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_10_nested_error_path(): void
    {
        $app = AppFactory::create_validation_errors_10_nested_error_path_2();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/profiles', ['body' => ["profile" => ["contact" => ["email" => "invalid"]]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["body", "profile", "contact", "email"], "msg" => "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+\$'", "input" => "invalid", "ctx" => ["pattern" => "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_array_item_validation_error(): void
    {
        $app = AppFactory::create_validation_errors_array_item_validation_error_3();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 10.0, "tags" => ["tag1", "tag2", 123]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "type_error", "loc" => ["body", "tags", "2"], "msg" => "Input should be a valid unknown", "input" => 123]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_array_max_items_constraint_violation(): void
    {
        $app = AppFactory::create_validation_errors_array_max_items_constraint_violation_4();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 10.0, "tags" => ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", "tag8", "tag9", "tag10", "tag11"]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "too_long", "loc" => ["body", "tags"], "msg" => "List should have at most 10 items after validation", "input" => ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", "tag8", "tag9", "tag10", "tag11"], "ctx" => ["max_length" => 10]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_array_min_items_constraint_violation(): void
    {
        $app = AppFactory::create_validation_errors_array_min_items_constraint_violation_5();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 10.0, "tags" => []]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "too_short", "loc" => ["body", "tags"], "msg" => "List should have at least 1 item after validation", "input" => [], "ctx" => ["min_length" => 1]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_body_field_type_error_string_for_float(): void
    {
        $app = AppFactory::create_validation_errors_body_field_type_error_string_for_float_6();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => "not_a_float"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "float_parsing", "loc" => ["body", "price"], "msg" => "Input should be a valid number, unable to parse string as a number", "input" => "not_a_float"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_header_validation_error(): void
    {
        $app = AppFactory::create_validation_errors_header_validation_error_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=test', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["headers", "x-token"], "msg" => "Field required", "input" => null]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_invalid_uuid_format(): void
    {
        $app = AppFactory::create_validation_errors_invalid_uuid_format_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/not-a-uuid', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "uuid_parsing", "loc" => ["path", "item_id"], "msg" => "Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0", "input" => "not-a-uuid"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_invalid_boolean_value(): void
    {
        $app = AppFactory::create_validation_errors_invalid_boolean_value_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?is_active=maybe&q=test', ['headers' => ["x-token" => "test-token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "bool_parsing", "loc" => ["query", "is_active"], "msg" => "Input should be a valid boolean, unable to interpret input", "input" => "maybe"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_invalid_datetime_format(): void
    {
        $app = AppFactory::create_validation_errors_invalid_datetime_format_10();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item", "price" => 10.0, "created_at" => "not-a-datetime"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "datetime_parsing", "loc" => ["body", "created_at"], "msg" => "Input should be a valid datetime", "input" => "not-a-datetime"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_invalid_enum_value(): void
    {
        $app = AppFactory::create_validation_errors_invalid_enum_value_11();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/models/invalid_model', []);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "enum", "loc" => ["path", "model_name"], "msg" => "Input should be 'alexnet', 'resnet' or 'lenet'", "input" => "invalid_model", "ctx" => ["expected" => "'alexnet', 'resnet' or 'lenet'"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_malformed_json_body(): void
    {
        $app = AppFactory::create_validation_errors_malformed_json_body_12();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => "{\"name\": \"Item\", \"price\": }"]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(400, $statusCode);

        $body = $response->body;
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = ["detail" => "Invalid request format"];
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_missing_required_body_field(): void
    {
        $app = AppFactory::create_validation_errors_missing_required_body_field_13();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Item"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["body", "price"], "msg" => "Field required", "input" => ["name" => "Item"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_missing_required_query_parameter(): void
    {
        $app = AppFactory::create_validation_errors_missing_required_query_parameter_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ["x-token" => "test-token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "missing", "loc" => ["query", "q"], "msg" => "Field required", "input" => null]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_multiple_validation_errors(): void
    {
        $app = AppFactory::create_validation_errors_multiple_validation_errors_15();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "X", "price" => -10, "quantity" => "not_a_number"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "3 validation errors in request", "errors" => [["type" => "string_too_short", "loc" => ["body", "name"], "msg" => "String should have at least 3 characters", "input" => "X", "ctx" => ["min_length" => 3]], ["type" => "greater_than", "loc" => ["body", "price"], "msg" => "Input should be greater than 0", "input" => -10, "ctx" => ["gt" => 0]], ["type" => "int_parsing", "loc" => ["body", "quantity"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "input" => "not_a_number"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_nested_object_validation_error(): void
    {
        $app = AppFactory::create_validation_errors_nested_object_validation_error_16();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ["Content-Type" => "application/json"], 'body' => ["name" => "Product", "price" => 10.0, "seller" => ["name" => "Jo", "address" => ["city" => "SF", "zip_code" => "123"]]]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "3 validation errors in request", "errors" => [["type" => "string_too_short", "loc" => ["body", "seller", "address", "city"], "msg" => "String should have at least 3 characters", "input" => "SF", "ctx" => ["min_length" => 3]], ["type" => "string_too_short", "loc" => ["body", "seller", "address", "zip_code"], "msg" => "String should have at least 5 characters", "input" => "123", "ctx" => ["min_length" => 5]], ["type" => "string_too_short", "loc" => ["body", "seller", "name"], "msg" => "String should have at least 3 characters", "input" => "Jo", "ctx" => ["min_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_numeric_constraint_violation_gt_greater_than(): void
    {
        $app = AppFactory::create_validation_errors_numeric_constraint_violation_gt_greater_than_17();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?price=0&q=test', ['headers' => ["x-token" => "test-token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "greater_than", "loc" => ["query", "price"], "msg" => "Input should be greater than 0", "input" => "0", "ctx" => ["gt" => 0]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_numeric_constraint_violation_le_less_than_or_equal(): void
    {
        $app = AppFactory::create_validation_errors_numeric_constraint_violation_le_less_than_or_equal_18();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?limit=101&q=test', ['headers' => ["x-token" => "test-token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "less_than_equal", "loc" => ["query", "limit"], "msg" => "Input should be less than or equal to 100", "input" => "101", "ctx" => ["le" => 100]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_query_param_type_error_string_provided_for_int(): void
    {
        $app = AppFactory::create_validation_errors_query_param_type_error_string_provided_for_int_19();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=test&skip=not_a_number', ['headers' => ["x-token" => "test-token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "int_parsing", "loc" => ["query", "skip"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "input" => "not_a_number"]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_string_max_length_constraint_violation(): void
    {
        $app = AppFactory::create_validation_errors_string_max_length_constraint_violation_20();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', ['headers' => ["x-token" => "test-token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_long", "loc" => ["query", "q"], "msg" => "String should have at most 50 characters", "input" => "this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter", "ctx" => ["max_length" => 50]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_string_min_length_constraint_violation(): void
    {
        $app = AppFactory::create_validation_errors_string_min_length_constraint_violation_21();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=ab', ['headers' => ["x-token" => "test-token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_too_short", "loc" => ["query", "q"], "msg" => "String should have at least 3 characters", "input" => "ab", "ctx" => ["min_length" => 3]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_validation_errors_string_regex_pattern_mismatch(): void
    {
        $app = AppFactory::create_validation_errors_string_regex_pattern_mismatch_22();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=invalid%21', ['headers' => ["x-token" => "test-token"]]);

        /** @var int $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame(422, $statusCode);

        $body = normalize_validation_errors($response->body);
        /** @var array<string, mixed>|string|int|float|bool|null $expected */
        $expected = normalize_validation_errors(["type" => "https://spikard.dev/errors/validation-error", "title" => "Request Validation Failed", "status" => 422, "detail" => "1 validation error in request", "errors" => [["type" => "string_pattern_mismatch", "loc" => ["query", "q"], "msg" => "String should match pattern '^[a-zA-Z0-9_-]+\$'", "input" => "invalid!", "ctx" => ["pattern" => "^[a-zA-Z0-9_-]+\$"]]]]);
        $this->assertEquals($expected, $body);
    }

    public function test_sse_notifications_1(): void
    {
        $app = AppFactory::create_sse_notifications_1();
        $client = TestClient::create($app);
        $stream = $client->connectSse('/notifications');

        /** @var array<mixed> $events */
        $events = $stream->eventsAsJson();
        $this->assertEquals([["level" => "critical", "message" => "Database connection pool exhausted", "source" => "database-service", "timestamp" => "2024-01-15T10:30:00Z", "type" => "system_alert"]], $events);
        $client->close();
    }

    public function test_sse_notifications_2(): void
    {
        $app = AppFactory::create_sse_notifications_2();
        $client = TestClient::create($app);
        $stream = $client->connectSse('/notifications');

        /** @var array<mixed> $events */
        $events = $stream->eventsAsJson();
        $this->assertEquals([[["message" => "example_message", "timestamp" => "2024-01-15T10:30:00Z", "type" => "example_type"], ["message" => "example_message", "timestamp" => "2024-01-15T10:30:00Z", "type" => "example_type"]]], $events);
        $client->close();
    }

    public function test_sse_notifications_3(): void
    {
        $app = AppFactory::create_sse_notifications_3();
        $client = TestClient::create($app);
        $stream = $client->connectSse('/notifications');

        /** @var array<mixed> $events */
        $events = $stream->eventsAsJson();
        $this->assertEquals([["body" => "You have received a new direct message", "priority" => "high", "timestamp" => "2024-01-15T10:30:00Z", "title" => "New message from John", "type" => "user_notification", "userId" => "user_12345"]], $events);
        $client->close();
    }

    public function test_sse_notifications_4(): void
    {
        $app = AppFactory::create_sse_notifications_4();
        $client = TestClient::create($app);
        $stream = $client->connectSse('/notifications');

        /** @var array<mixed> $events */
        $events = $stream->eventsAsJson();
        $this->assertEquals([["message" => "All systems operational", "metadata" => ["region" => "us-east-1", "uptime" => 99.99], "service" => "payment-gateway", "status" => "operational", "timestamp" => "2024-01-15T10:30:00Z", "type" => "status_update"]], $events);
        $client->close();
    }

    public function test_websocket_systemalert_1(): void
    {
        $app = AppFactory::create_websocket_systemalert_1();
        $client = TestClient::create($app);
        $ws = $client->connectWebSocket('systemAlert');
        $ws->sendJson("{\"level\":\"example_level\",\"message\":\"example_message\",\"source\":\"example_source\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"system_alert\"}");
        $this->assertFalse($ws->isClosed());
        $ws->close();
        $this->assertTrue($ws->isClosed());
        $client->close();
    }

    public function test_websocket_chat_2(): void
    {
        $app = AppFactory::create_websocket_chat_2();
        $client = TestClient::create($app);
        $ws = $client->connectWebSocket('/chat');
        $ws->sendJson("{\"text\":\"Hello, everyone!\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"message\",\"user\":\"alice\"}");
        $this->assertFalse($ws->isClosed());
        $ws->close();
        $this->assertTrue($ws->isClosed());
        $client->close();
    }

    public function test_websocket_chatack_3(): void
    {
        $app = AppFactory::create_websocket_chatack_3();
        $client = TestClient::create($app);
        $ws = $client->connectWebSocket('chatAck');
        $ws->sendJson("{\"messageId\":\"ack-123\",\"status\":\"delivered\",\"timestamp\":\"2024-01-15T10:31:00Z\",\"type\":\"chatAck\"}");
        $this->assertFalse($ws->isClosed());
        $ws->close();
        $this->assertTrue($ws->isClosed());
        $client->close();
    }

    public function test_websocket_chat_4(): void
    {
        $app = AppFactory::create_websocket_chat_4();
        $client = TestClient::create($app);
        $ws = $client->connectWebSocket('/chat');
        $ws->sendJson("{\"timestamp\":\"2024-01-15T10:35:00Z\",\"type\":\"userLeft\",\"user\":\"charlie\"}");
        $this->assertFalse($ws->isClosed());
        $ws->close();
        $this->assertTrue($ws->isClosed());
        $client->close();
    }

    public function test_websocket_chat_5(): void
    {
        $app = AppFactory::create_websocket_chat_5();
        $client = TestClient::create($app);
        $ws = $client->connectWebSocket('/chat');
        $ws->sendJson("{\"timestamp\":\"2024-01-15T10:29:55Z\",\"type\":\"userJoined\",\"user\":\"bob\"}");
        $this->assertFalse($ws->isClosed());
        $ws->close();
        $this->assertTrue($ws->isClosed());
        $client->close();
    }

    public function test_websocket_usernotification_6(): void
    {
        $app = AppFactory::create_websocket_usernotification_6();
        $client = TestClient::create($app);
        $ws = $client->connectWebSocket('userNotification');
        $ws->sendJson("{\"body\":\"example_body\",\"priority\":\"example_priority\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"title\":\"example_title\",\"type\":\"user_notification\",\"userId\":\"example_userId\"}");
        $this->assertFalse($ws->isClosed());
        $ws->close();
        $this->assertTrue($ws->isClosed());
        $client->close();
    }

    public function test_websocket_statusupdate_7(): void
    {
        $app = AppFactory::create_websocket_statusupdate_7();
        $client = TestClient::create($app);
        $ws = $client->connectWebSocket('statusUpdate');
        $ws->sendJson("{\"message\":\"example_message\",\"metadata\":{},\"service\":\"example_service\",\"status\":\"example_status\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"status_update\"}");
        $this->assertFalse($ws->isClosed());
        $ws->close();
        $this->assertTrue($ws->isClosed());
        $client->close();
    }

}
