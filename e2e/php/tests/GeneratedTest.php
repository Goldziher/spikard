<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;
use E2E\Php\AppFactory;

/**
 * Generated from testing_data fixtures.
 * Tests are marked incomplete until the PHP bindings are wired.
 */
final class GeneratedTest extends TestCase
{
    protected function setUp(): void
    {
        $this->markTestIncomplete('PHP bindings not implemented yet.');
    }

    public function test_auth_api_key_authentication_invalid_key(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'invalid_key_12345']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'The provided API key is not valid', 'status' => 401, 'title' => 'Invalid API key', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_api_key_authentication_missing_header(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', []);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'Expected \'X-API-Key\' header or \'api_key\' query parameter with valid API key', 'status' => 401, 'title' => 'Missing API key', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_api_key_authentication_valid_key(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'sk_test_123456']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['data' => 'sensitive information', 'message' => 'Access granted'], $response->body);
    }

    public function test_auth_api_key_in_query_parameter(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data?api_key=sk_test_123456', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['data' => 'sensitive information', 'message' => 'Access granted'], $response->body);
    }

    public function test_auth_api_key_rotation_old_key_still_valid(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'sk_test_old_123456']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['data' => 'sensitive information', 'message' => 'Access granted'], $response->body);
    }

    public function test_auth_api_key_with_custom_header_name(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Token' => 'sk_test_123456']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['data' => 'sensitive information', 'message' => 'Access granted'], $response->body);
    }

    public function test_auth_bearer_token_without_prefix(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ['Authorization' => 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDZ9.8yXqZ9jKCR0BwqJc7pN_QvD3mYLxHfWzUeIaGkTnOsA']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'Authorization header must use Bearer scheme: \'Bearer <token>\'', 'status' => 401, 'title' => 'Invalid Authorization header format', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_jwt_authentication_expired_token(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', ['headers' => ['Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNjAwMDAwMDAwLCJpYXQiOjE1OTAwMDAwMDB9.n4oBw9XuO2aAJWi1e4Bz9Y_m2iEyJHGAODcetNuwYFo']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'Token has expired', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_jwt_authentication_invalid_audience(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', ['headers' => ['Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTk5LCJpYXQiOjE3MzEyNTIwMDAsImF1ZCI6WyJodHRwczovL3dyb25nLXNlcnZpY2UuY29tIl19.YR2a9fSJjhen7ksYFI2djSBSC7Pc29FDCloBGhkj3kU']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'Token audience is invalid', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_jwt_authentication_invalid_signature(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', ['headers' => ['Authorization' => 'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMH0.invalid_signature_here']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'Token signature is invalid', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_jwt_authentication_missing_authorization_header(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', []);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'Expected \'Authorization: Bearer <token>\'', 'status' => 401, 'title' => 'Missing or invalid Authorization header', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_jwt_authentication_valid_token(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', ['headers' => ['Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Access granted', 'user_id' => 'user123'], $response->body);
    }

    public function test_auth_jwt_invalid_issuer(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ['Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2V2aWwuY29tIn0.mbL5L04_hpaaiz0SPABap6ZWfBLu18aiexBjzwQ1nnA']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'Token issuer is invalid, expected \'https://auth.example.com\'', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_jwt_malformed_token_format(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ['Authorization' => 'Bearer invalid.token']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'Malformed JWT token: expected 3 parts separated by dots, found 2', 'status' => 401, 'title' => 'Malformed JWT token', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_jwt_missing_required_custom_claims(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/admin', ['headers' => ['Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg']]);

        $this->assertSame(403, $response->statusCode);
        $this->assertEquals(['detail' => 'Required claims \'role\' and \'permissions\' missing from JWT', 'status' => 403, 'title' => 'Forbidden', 'type' => 'https://spikard.dev/errors/forbidden'], $response->body);
    }

    public function test_auth_jwt_not_before_claim_in_future(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ['Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsIm5iZiI6MjYyNjc4Mzk0NiwiYXVkIjpbImh0dHBzOi8vYXBpLmV4YW1wbGUuY29tIl0sImlzcyI6Imh0dHBzOi8vYXV0aC5leGFtcGxlLmNvbSJ9.hG4I76_3kJfsbJ_jmxoP1NSYnkcqdyBFcPpdo-jYU4E']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'JWT not valid yet, not before claim is in the future', 'status' => 401, 'title' => 'JWT validation failed', 'type' => 'https://spikard.dev/errors/unauthorized'], $response->body);
    }

    public function test_auth_jwt_with_multiple_audiences(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', ['headers' => ['Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSIsImh0dHBzOi8vYWRtaW4uZXhhbXBsZS5jb20iXSwiaXNzIjoiaHR0cHM6Ly9hdXRoLmV4YW1wbGUuY29tIn0.9MBL_XccGXfu9cDUnCpQruDMOl2hHYydzeGn-20dQOs']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Access granted', 'user_id' => 'user123'], $response->body);
    }

    public function test_auth_multiple_authentication_schemes_jwt_precedence(): void
    {
        $app = AppFactory::create_auth();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg', 'X-API-Key' => 'sk_test_123456']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['auth_method' => 'jwt', 'message' => 'Access granted', 'user_id' => 'user123'], $response->body);
    }

    public function test_background_background_event_logging(): void
    {
        $app = AppFactory::create_background();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/background/events', ['body' => ['event' => 'alpha']]);

        $this->assertSame(202, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_background_background_event_logging_second_payload(): void
    {
        $app = AppFactory::create_background();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/background/events', ['body' => ['event' => 'beta']]);

        $this->assertSame(202, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_body_limits_body_over_limit_returns_413(): void
    {
        $app = AppFactory::create_body_limits();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/body-limit/over', ['body' => ['note' => 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx']]);

        $this->assertSame(413, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_body_limits_body_under_limit_succeeds(): void
    {
        $app = AppFactory::create_body_limits();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/body-limit/under', ['body' => ['note' => 'small']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['accepted' => true, 'note' => 'small'], $response->body);
    }

    public function test_compression_compression_gzip_applied(): void
    {
        $app = AppFactory::create_compression();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/compression/gzip', ['headers' => ['Accept-Encoding' => 'gzip']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Compressed payload', 'payload' => 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'], $response->body);
    }

    public function test_compression_compression_payload_below_min_size_is_not_compressed(): void
    {
        $app = AppFactory::create_compression();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/compression/skip', ['headers' => ['Accept-Encoding' => 'gzip']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Small payload', 'payload' => 'tiny'], $response->body);
    }

    public function test_content_types_13_json_with_charset_utf16(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ['Content-Type' => 'application/json; charset=utf-16'], 'body' => ['value' => 'test']]);

        $this->assertSame(415, $response->statusCode);
        $this->assertEquals(['detail' => 'Unsupported charset \'utf-16\' for JSON. Only UTF-8 is supported.', 'status' => 415, 'title' => 'Unsupported Charset', 'type' => 'https://spikard.dev/errors/unsupported-charset'], $response->body);
    }

    public function test_content_types_14_content_type_case_insensitive(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ['Content-Type' => 'APPLICATION/JSON'], 'body' => ['name' => 'test']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['name' => 'test'], $response->body);
    }

    public function test_content_types_15_multipart_boundary_required(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['headers' => ['Content-Type' => 'multipart/form-data']]);

        $this->assertSame(400, $response->statusCode);
        $this->assertEquals(['error' => 'multipart/form-data requires \'boundary\' parameter'], $response->body);
    }

    public function test_content_types_16_text_plain_not_accepted(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ['Content-Type' => 'text/plain'], 'body' => '{"data": "value"}']);

        $this->assertSame(415, $response->statusCode);
        $this->assertEquals(['detail' => 'Unsupported media type', 'status' => 415, 'title' => 'Unsupported Media Type', 'type' => 'https://spikard.dev/errors/unsupported-media-type'], $response->body);
    }

    public function test_content_types_17_vendor_json_accepted(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/v1/resource', ['headers' => ['Content-Type' => 'application/vnd.api+json'], 'body' => ['data' => 'value']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['data' => 'value'], $response->body);
    }

    public function test_content_types_18_content_type_with_multiple_params(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ['Content-Type' => 'application/json; charset=utf-8; boundary=something'], 'body' => ['value' => 'test']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['value' => 'test'], $response->body);
    }

    public function test_content_types_19_missing_content_type_default_json(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['body' => ['name' => 'test']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['name' => 'test'], $response->body);
    }

    public function test_content_types_20_content_length_mismatch(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ['Content-Length' => '100', 'Content-Type' => 'application/json'], 'body' => ['value' => 'short']]);

        $this->assertSame(400, $response->statusCode);
        $this->assertEquals(['error' => 'Content-Length header does not match actual body size'], $response->body);
    }

    public function test_content_types_415_unsupported_media_type(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/xml'], 'body' => '<?xml version="1.0"?><item><name>Item</name></item>']);

        $this->assertSame(415, $response->statusCode);
        $this->assertEquals(['detail' => 'Unsupported media type'], $response->body);
    }

    public function test_content_types_binary_response_application_octet_stream(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/download/file.bin', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('binary_data_placeholder', $response->body);
    }

    public function test_content_types_csv_response_text_csv(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/export/data.csv', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('id,name,price
1,Item A,10.0
2,Item B,20.0', $response->body);
    }

    public function test_content_types_content_negotiation_accept_header(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/accept-test/1', ['headers' => ['Accept' => 'application/json']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => 1, 'name' => 'Item'], $response->body);
    }

    public function test_content_types_html_response_text_html(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/html', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('<html><body><h1>Hello</h1></body></html>', $response->body);
    }

    public function test_content_types_jpeg_image_response_image_jpeg(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/images/photo.jpg', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('jpeg_binary_data', $response->body);
    }

    public function test_content_types_json_response_application_json(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/json', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['name' => 'Item', 'price' => 42.0], $response->body);
    }

    public function test_content_types_json_with_utf_8_charset(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/unicode', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['emoji' => '☕', 'name' => 'Café'], $response->body);
    }

    public function test_content_types_pdf_response_application_pdf(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/download/document.pdf', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('pdf_binary_data', $response->body);
    }

    public function test_content_types_png_image_response_image_png(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/images/logo.png', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('png_binary_data', $response->body);
    }

    public function test_content_types_plain_text_response_text_plain(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/text', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('Hello, World!', $response->body);
    }

    public function test_content_types_xml_response_application_xml(): void
    {
        $app = AppFactory::create_content_types();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/xml', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('<?xml version="1.0"?><item><name>Item</name><price>42.0</price></item>', $response->body);
    }

    public function test_cookies_24_cookie_samesite_strict(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/secure', ['cookies' => ['session_id' => 'abc123xyz789']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cookies_25_cookie_samesite_lax(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data', ['cookies' => ['tracking' => 'track123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cookies_26_cookie_secure_flag(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/secure', ['cookies' => ['auth_token' => 'secure_token_xyz']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cookies_27_cookie_httponly_flag(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/secure', ['cookies' => ['session' => 'session_abc123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cookies_apikey_cookie_authentication_missing(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me/auth', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['cookie', 'key'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_cookies_apikey_cookie_authentication_success(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['cookies' => ['key' => 'secret']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['username' => 'secret'], $response->body);
    }

    public function test_cookies_cookie_regex_pattern_validation_fail(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookies/pattern', ['cookies' => ['tracking_id' => 'invalid-format']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[A-Z0-9]{8}$'], 'input' => 'invalid-format', 'loc' => ['cookie', 'tracking_id'], 'msg' => 'String should match pattern \'^[A-Z0-9]{8}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_cookies_cookie_regex_pattern_validation_success(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookies/pattern', ['cookies' => ['tracking_id' => 'ABC12345']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['tracking_id' => 'ABC12345'], $response->body);
    }

    public function test_cookies_cookie_validation_max_length_constraint_fail(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookies/validated', ['cookies' => ['session_id' => 'this_cookie_value_is_way_too_long']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 20], 'input' => 'this_cookie_value_is_way_too_long', 'loc' => ['cookie', 'session_id'], 'msg' => 'String should have at most 20 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_cookies_cookie_validation_min_length_constraint_success(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookies/min-length', ['cookies' => ['token' => 'abc']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['token' => 'abc'], $response->body);
    }

    public function test_cookies_cookie_validation_min_length_failure(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['cookies' => ['tracking_id' => 'ab']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['cookie', 'tracking_id'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_cookies_multiple_cookies_success(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['cookies' => ['googall_tracker' => 'ga789', 'fatebook_tracker' => 'tracker456', 'session_id' => 'session123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['fatebook_tracker' => 'tracker456', 'googall_tracker' => 'ga789', 'session_id' => 'session123'], $response->body);
    }

    public function test_cookies_optional_apikey_cookie_missing(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['msg' => 'Create an account first'], $response->body);
    }

    public function test_cookies_optional_cookie_parameter_missing(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['ads_id' => null], $response->body);
    }

    public function test_cookies_optional_cookie_parameter_success(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['cookies' => ['ads_id' => 'abc123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['ads_id' => 'abc123'], $response->body);
    }

    public function test_cookies_required_cookie_missing(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/cookies', ['cookies' => ['fatebook_tracker' => 'tracker456']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['cookie', 'session_id'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_cookies_response_delete_cookie(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/delete', ['cookies' => ['session' => 'old_session_123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Cookie deleted'], $response->body);
    }

    public function test_cookies_response_multiple_cookies(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/multiple', ['body' => ['session' => 'session123', 'user' => 'john']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Multiple cookies set'], $response->body);
    }

    public function test_cookies_response_session_cookie_no_max_age(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/session', ['body' => ['value' => 'session_abc123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Session cookie set'], $response->body);
    }

    public function test_cookies_response_cookie_with_samesite_lax(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/samesite-lax', ['body' => ['value' => 'lax_cookie']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Cookie set with SameSite=Lax'], $response->body);
    }

    public function test_cookies_response_cookie_with_samesite_none(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/samesite-none', ['body' => ['value' => 'none_cookie']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Cookie set with SameSite=None'], $response->body);
    }

    public function test_cookies_response_cookie_with_samesite_strict(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/samesite-strict', ['body' => ['value' => 'strict_cookie']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Cookie set with SameSite=Strict'], $response->body);
    }

    public function test_cookies_response_cookie_with_attributes(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/cookie/set', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Cookie set'], $response->body);
    }

    public function test_cookies_response_cookie_with_domain_attribute(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/set-with-domain', ['body' => ['value' => 'domain_test']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Cookie set with domain'], $response->body);
    }

    public function test_cookies_response_cookie_with_path_attribute(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookies/set-with-path', ['body' => ['value' => 'path_test']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Cookie set with path'], $response->body);
    }

    public function test_cookies_response_set_cookie_basic(): void
    {
        $app = AppFactory::create_cookies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/cookie/', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Come to the dark side, we have cookies'], $response->body);
    }

    public function test_cors_06_cors_preflight_method_not_allowed(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/data', ['headers' => ['Access-Control-Request-Headers' => 'Content-Type', 'Origin' => 'https://example.com', 'Access-Control-Request-Method' => 'DELETE']]);

        $this->assertSame(403, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cors_07_cors_preflight_header_not_allowed(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/data', ['headers' => ['Access-Control-Request-Method' => 'POST', 'Access-Control-Request-Headers' => 'X-Custom-Header', 'Origin' => 'https://example.com']]);

        $this->assertSame(403, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cors_08_cors_max_age(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/data', ['headers' => ['Origin' => 'https://example.com', 'Access-Control-Request-Method' => 'POST', 'Access-Control-Request-Headers' => 'Content-Type']]);

        $this->assertSame(204, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cors_09_cors_expose_headers(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['Origin' => 'https://example.com']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cors_10_cors_origin_null(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['Origin' => 'null']]);

        $this->assertSame(403, $response->statusCode);
        $this->assertEquals(['error' => 'Origin \'null\' is not allowed'], $response->body);
    }

    public function test_cors_cors_private_network_access(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/local-resource', ['headers' => ['Access-Control-Request-Method' => 'GET', 'Access-Control-Request-Private-Network' => 'true', 'Origin' => 'https://public.example.com']]);

        $this->assertSame(204, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cors_cors_vary_header_for_proper_caching(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/cached-resource', ['headers' => ['Origin' => 'https://app.example.com', 'Cache-Control' => 'max-age=3600']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['data' => 'cacheable resource'], $response->body);
    }

    public function test_cors_cors_multiple_allowed_origins(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['Origin' => 'https://admin.example.com']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['data' => 'resource data'], $response->body);
    }

    public function test_cors_cors_origin_case_sensitivity(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['Origin' => 'https://EXAMPLE.COM']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cors_cors_preflight_for_delete_method(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/resource/456', ['headers' => ['Access-Control-Request-Method' => 'DELETE', 'Origin' => 'https://app.example.com']]);

        $this->assertSame(204, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cors_cors_preflight_for_put_method(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/api/resource/123', ['headers' => ['Access-Control-Request-Headers' => 'Content-Type, X-Custom-Header', 'Origin' => 'https://app.example.com', 'Access-Control-Request-Method' => 'PUT']]);

        $this->assertSame(204, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cors_cors_preflight_request(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/items/', ['headers' => ['Access-Control-Request-Method' => 'POST', 'Origin' => 'https://example.com', 'Access-Control-Request-Headers' => 'Content-Type, X-Custom-Header']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_cors_cors_regex_pattern_matching_for_origins(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['Origin' => 'https://subdomain.example.com']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['data' => 'resource data'], $response->body);
    }

    public function test_cors_cors_request_blocked(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ['Origin' => 'https://malicious-site.com']]);

        $this->assertSame(403, $response->statusCode);
        $this->assertEquals(['detail' => 'CORS request from origin \'https://malicious-site.com\' not allowed'], $response->body);
    }

    public function test_cors_cors_safelisted_headers_without_preflight(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/form', ['headers' => ['Accept' => 'application/json', 'Accept-Language' => 'en-US', 'Origin' => 'https://app.example.com', 'Content-Type' => 'text/plain']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Success'], $response->body);
    }

    public function test_cors_cors_wildcard_origin(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/public/data', ['headers' => ['Origin' => 'https://random-site.com']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['data' => 'public'], $response->body);
    }

    public function test_cors_cors_with_credentials(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/user/profile', ['headers' => ['Cookie' => 'session=abc123', 'Origin' => 'https://app.example.com']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['username' => 'john'], $response->body);
    }

    public function test_cors_simple_cors_request(): void
    {
        $app = AppFactory::create_cors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ['Origin' => 'https://example.com']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['items' => []], $response->body);
    }

    public function test_di_async_factory_dependency_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/db-status', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['max_size' => 10, 'pool_status' => 'connected'], $response->body);
    }

    public function test_di_circular_dependency_detection_error(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/circular', []);

        $this->assertSame(500, $response->statusCode);
        $this->assertEquals(['detail' => 'Circular dependency detected', 'errors' => [['cycle' => ['service_a', 'service_b', 'service_a'], 'msg' => 'Circular dependency detected in dependency graph', 'type' => 'circular_dependency']], 'status' => 500, 'title' => 'Dependency Resolution Failed', 'type' => 'https://spikard.dev/errors/dependency-error'], $response->body);
    }

    public function test_di_dependency_injection_in_lifecycle_hooks_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/hook-di-test', ['headers' => ['authorization' => 'Bearer valid_token']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['authenticated' => true, 'logged' => true], $response->body);
    }

    public function test_di_factory_dependency_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/timestamp', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['timestamp' => '<<present>>'], $response->body);
    }

    public function test_di_missing_dependency_error(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/missing-dep', []);

        $this->assertSame(500, $response->statusCode);
        $this->assertEquals(['detail' => 'Required dependency not found', 'errors' => [['dependency_key' => 'non_existent_service', 'msg' => 'Dependency \'non_existent_service\' is not registered', 'type' => 'missing_dependency']], 'status' => 500, 'title' => 'Dependency Resolution Failed', 'type' => 'https://spikard.dev/errors/dependency-error'], $response->body);
    }

    public function test_di_mixed_singleton_and_per_request_caching_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/mixed-caching', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['app_name' => 'MyApp', 'context_id' => '<<uuid>>', 'pool_id' => '<<uuid>>'], $response->body);
    }

    public function test_di_multiple_dependencies_with_cleanup_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/multi-cleanup-test', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['session_active' => true], $response->body);
    }

    public function test_di_nested_dependencies_3_levels_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/auth-status', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['auth_enabled' => true, 'has_cache' => true, 'has_db' => true], $response->body);
    }

    public function test_di_node_js_object_destructuring_injection_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/node-destructure', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['db_name' => 'PostgreSQL', 'log_level' => 'info'], $response->body);
    }

    public function test_di_per_request_dependency_caching_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/request-id', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['first_id' => '<<uuid>>', 'second_id' => '<<same_as:first_id>>'], $response->body);
    }

    public function test_di_python_parameter_name_based_injection_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/python-name-inject', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['cache_status' => 'ready', 'db_status' => 'connected'], $response->body);
    }

    public function test_di_python_type_annotation_based_injection_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/python-type-inject', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['cache_type' => 'Redis', 'pool_type' => 'PostgreSQL'], $response->body);
    }

    public function test_di_resource_cleanup_after_request_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/cleanup-test', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['session_id' => '<<uuid>>', 'status' => 'completed'], $response->body);
    }

    public function test_di_route_level_dependency_override_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/override-test', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['mode' => 'test', 'strict' => false], $response->body);
    }

    public function test_di_ruby_keyword_argument_injection_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/ruby-kwargs', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['adapter' => 'postgresql', 'user_id' => 42], $response->body);
    }

    public function test_di_singleton_dependency_caching_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/app-counter', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['count' => 1, 'counter_id' => '<<uuid>>'], $response->body);
    }

    public function test_di_type_mismatch_in_dependency_resolution_error(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/type-mismatch', []);

        $this->assertSame(500, $response->statusCode);
        $this->assertEquals(['detail' => 'Dependency type mismatch', 'errors' => [['actual_type' => 'string', 'dependency_key' => 'config', 'expected_type' => 'object', 'msg' => 'Dependency \'config\' type mismatch: expected object, got string', 'type' => 'type_mismatch']], 'status' => 500, 'title' => 'Dependency Resolution Failed', 'type' => 'https://spikard.dev/errors/dependency-error'], $response->body);
    }

    public function test_di_value_dependency_injection_success(): void
    {
        $app = AppFactory::create_di();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/config', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['app_name' => 'SpikardApp', 'max_connections' => 100, 'version' => '1.0.0'], $response->body);
    }

    public function test_edge_cases_11_utf8_query_parameter(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=%22caf%C3%A9%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['term' => 'café'], $response->body);
    }

    public function test_edge_cases_12_percent_encoded_special_chars(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=hi%20there?term=%22hi%20there%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['term' => 'hi there'], $response->body);
    }

    public function test_edge_cases_13_empty_string_query_param_preserved(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?filter=?filter=%22%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['filter' => ''], $response->body);
    }

    public function test_edge_cases_14_large_integer_boundary(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?id=%229007199254740991%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => 9007199254740991], $response->body);
    }

    public function test_edge_cases_15_float_precision_preservation(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/calculate', ['body' => ['value' => 3.141592653589793]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['value' => 3.141592653589793], $response->body);
    }

    public function test_edge_cases_16_negative_zero_handling(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['body' => ['offset' => -0.0]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['offset' => 0], $response->body);
    }

    public function test_edge_cases_17_extremely_long_string(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/text', ['body' => ['content' => 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 10001, 'max_length' => 10000], 'loc' => ['body', 'content'], 'msg' => 'String length must not exceed 10000', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_edge_cases_18_unicode_normalization(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ['name' => 'café']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['name' => 'café'], $response->body);
    }

    public function test_edge_cases_19_emoji_in_strings(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/messages', ['body' => ['text' => 'Hello 👋 World 🌍']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['text' => 'Hello 👋 World 🌍'], $response->body);
    }

    public function test_edge_cases_20_null_byte_in_string(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files', ['body' => ['filename' => 'file .txt']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['value' => 'file\\u0000.txt'], 'loc' => ['body', 'filename'], 'msg' => 'String contains null byte character', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_edge_cases_21_scientific_notation_number(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/calculate', ['body' => ['value' => 123000.0]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['value' => 123000], $response->body);
    }

    public function test_edge_cases_22_leading_zeros_integer(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data?value=%220123%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['value' => 123], $response->body);
    }

    public function test_edge_cases_23_deeply_nested_json_limit(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['body' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['nested' => ['value' => 'deep']]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]);

        $this->assertSame(400, $response->statusCode);
        $this->assertEquals(['error' => 'Request body exceeds maximum nesting depth of 32'], $response->body);
    }

    public function test_edge_cases_24_array_with_holes(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'items[0]=first&items[2]=third&items[5]=sixth']);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['items' => ['first', 'third', 'sixth']], $response->body);
    }

    public function test_edge_cases_deeply_nested_structure_10_levels(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/nested/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['level1' => ['level2' => ['level3' => ['level4' => ['level5' => ['level6' => ['level7' => ['level8' => ['level9' => ['level10' => ['depth' => 10, 'value' => 'deep']]]]]]]]]]]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['max_depth' => 10, 'message' => 'Processed deeply nested structure', 'value_found' => 'deep'], $response->body);
    }

    public function test_edge_cases_empty_and_null_value_handling(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/nulls/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['empty_array' => [], 'empty_object' => [], 'empty_string' => '', 'explicit_null' => null, 'false_boolean' => false, 'zero_number' => 0]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['empty_array_length' => 0, 'empty_object_keys' => 0, 'empty_string_length' => 0, 'explicit_null_is_null' => true, 'false_is_false' => true, 'zero_is_falsy' => true], $response->body);
    }

    public function test_edge_cases_float_precision_and_rounding(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/calculations/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['expected_sum' => 0.3, 'precise_value' => 3.141592653589793, 'value1' => 0.1, 'value2' => 0.2, 'very_large' => 1.7976931348623157e308, 'very_small' => 1e-10]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['precise_value' => 3.141592653589793, 'sum' => 0.30000000000000004, 'very_large' => 1.7976931348623157e308, 'very_small' => 1e-10], $response->body);
    }

    public function test_edge_cases_large_integer_boundary_values(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/numbers/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['large_int' => 9223372036854775807, 'max_safe_int' => 9007199254740991, 'negative_large' => -9223372036854775808]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['large_int' => 9223372036854775807, 'max_safe_int' => 9007199254740991, 'negative_large' => -9223372036854775808], $response->body);
    }

    public function test_edge_cases_special_string_values_and_escaping(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/strings/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['backslashes' => 'C:\\\\Users\\\\Path', 'empty_string' => '', 'quotes' => 'He said "hello" and \'goodbye\'', 'special_chars' => '!@#$%^&*()_+-=[]{}|;\':",./<>?', 'tabs_newlines' => 'line1
	line2
line3', 'unicode_escapes' => '\\u0048\\u0065\\u006c\\u006c\\u006f', 'whitespace' => '   ']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['backslashes' => 'C:\\\\Users\\\\Path', 'empty_string' => '', 'quotes' => 'He said "hello" and \'goodbye\'', 'special_chars' => '!@#$%^&*()_+-=[]{}|;\':",./<>?', 'tabs_newlines' => 'line1
	line2
line3', 'unicode_escapes' => 'Hello', 'whitespace' => '   '], $response->body);
    }

    public function test_edge_cases_unicode_and_emoji_handling(): void
    {
        $app = AppFactory::create_edge_cases();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json; charset=utf-8'], 'body' => ['description' => 'Best café in München 🇩🇪', 'emoji_reactions' => '👍❤️😂🎉', 'name' => 'Coffee Shop ☕', 'tags' => ['食べ物', '音楽', '💰']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['description' => 'Best café in München 🇩🇪', 'emoji_reactions' => '👍❤️😂🎉', 'id' => 1, 'name' => 'Coffee Shop ☕', 'tags' => ['食べ物', '音楽', '💰']], $response->body);
    }

    public function test_headers_30_bearer_token_format_valid(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected', ['headers' => ['Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_headers_31_bearer_token_format_invalid(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected', ['headers' => ['Authorization' => 'Bearer invalid token with spaces']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^Bearer [A-Za-z0-9-._~+/]+=*$', 'value' => 'Bearer invalid token with spaces'], 'loc' => ['headers', 'authorization'], 'msg' => 'Invalid Bearer token format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_32_bearer_token_missing_prefix(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected', ['headers' => ['Authorization' => 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^Bearer [A-Za-z0-9-._~+/]+=*$', 'value' => 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9'], 'loc' => ['headers', 'authorization'], 'msg' => 'Invalid Bearer token format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_33_api_key_header_valid(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_headers_34_api_key_header_invalid(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'invalid-key']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-f0-9]{32}$', 'value' => 'invalid-key'], 'loc' => ['headers', 'x-api-key'], 'msg' => 'Invalid API key format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_accept_header_json(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/accept', ['headers' => ['Accept' => 'application/json']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['accept' => 'application/json'], $response->body);
    }

    public function test_headers_accept_encoding_header(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/accept-encoding', ['headers' => ['Accept-Encoding' => 'gzip, deflate, br']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['accept_encoding' => 'gzip, deflate, br'], $response->body);
    }

    public function test_headers_accept_language_header(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/accept-language', ['headers' => ['Accept-Language' => 'en-US,en;q=0.9']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['accept_language' => 'en-US,en;q=0.9'], $response->body);
    }

    public function test_headers_authorization_header_missing(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'authorization'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_authorization_header_success(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['headers' => ['Authorization' => 'Digest foobar']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['credentials' => 'foobar', 'scheme' => 'Digest'], $response->body);
    }

    public function test_headers_authorization_header_wrong_scheme(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['headers' => ['Authorization' => 'Other invalidauthorization']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'Other invalidauthorization', 'loc' => ['headers', 'authorization'], 'msg' => 'String should match pattern \'^Digest .+\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_basic_authentication_success(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/basic-auth', ['headers' => ['Authorization' => 'Basic dXNlcm5hbWU6cGFzc3dvcmQ=']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['password' => 'password', 'username' => 'username'], $response->body);
    }

    public function test_headers_bearer_token_authentication_missing(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/bearer-auth', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'authorization'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_bearer_token_authentication_success(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/bearer-auth', ['headers' => ['Authorization' => 'Bearer valid_token_123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['token' => 'valid_token_123'], $response->body);
    }

    public function test_headers_content_type_header_application_json(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/content-type', ['headers' => ['Content-Type' => 'application/json']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['content_type' => 'application/json'], $response->body);
    }

    public function test_headers_header_case_insensitivity_access(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/echo', ['headers' => ['content-type' => 'application/json'], 'body' => ['test' => 'data']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['content_type_lower' => 'application/json', 'content_type_mixed' => 'application/json', 'content_type_upper' => 'application/json'], $response->body);
    }

    public function test_headers_header_regex_validation_fail(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/pattern', ['headers' => ['X-Request-Id' => 'invalid-format']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[0-9]{3,}$'], 'input' => 'invalid-format', 'loc' => ['headers', 'x-request-id'], 'msg' => 'String should match pattern \'^[0-9]{3,}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_header_regex_validation_success(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/pattern', ['headers' => ['X-Request-Id' => '12345']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['x_request_id' => '12345'], $response->body);
    }

    public function test_headers_header_validation_max_length_constraint_fail(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/max-length', ['headers' => ['X-Session-Id' => 'this_is_way_too_long_for_validation']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 20], 'input' => 'this_is_way_too_long_for_validation', 'loc' => ['headers', 'x-session-id'], 'msg' => 'String should have at most 20 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_header_validation_min_length_constraint(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/validated', ['headers' => ['X-Token' => 'ab']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['headers', 'x-token'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_header_with_underscore_conversion_explicit(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/underscore', ['headers' => ['X-Token' => 'secret123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['x_token' => 'secret123'], $response->body);
    }

    public function test_headers_host_header(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/host', ['headers' => ['Host' => 'example.com:8080']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['host' => 'example.com:8080'], $response->body);
    }

    public function test_headers_multiple_custom_headers(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/multiple', ['headers' => ['X-Trace-Id' => 'trace-abc', 'X-Request-Id' => 'req-12345', 'X-Client-Version' => '1.2.3']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['x_client_version' => '1.2.3', 'x_request_id' => 'req-12345', 'x_trace_id' => 'trace-abc'], $response->body);
    }

    public function test_headers_multiple_header_values_x_token(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ['x-token' => 'foo, bar']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['X-Token values' => ['foo', 'bar']], $response->body);
    }

    public function test_headers_optional_header_with_none_default_missing(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['strange_header' => null], $response->body);
    }

    public function test_headers_origin_header(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/origin', ['headers' => ['Origin' => 'https://example.com']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['origin' => 'https://example.com'], $response->body);
    }

    public function test_headers_referer_header(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/headers/referer', ['headers' => ['Referer' => 'https://example.com/page']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['referer' => 'https://example.com/page'], $response->body);
    }

    public function test_headers_user_agent_header_custom_value(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ['User-Agent' => 'Mozilla/5.0 Custom Browser']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['User-Agent' => 'Mozilla/5.0 Custom Browser'], $response->body);
    }

    public function test_headers_user_agent_header_default_value(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['User-Agent' => 'testclient'], $response->body);
    }

    public function test_headers_x_api_key_optional_header_missing(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['msg' => 'Hello World'], $response->body);
    }

    public function test_headers_x_api_key_optional_header_success(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['headers' => ['key' => 'secret']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['msg' => 'Hello secret'], $response->body);
    }

    public function test_headers_x_api_key_required_header_missing(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'x-api-key'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_headers_x_api_key_required_header_success(): void
    {
        $app = AppFactory::create_headers();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', ['headers' => ['key' => 'secret']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['username' => 'secret'], $response->body);
    }

    public function test_http_methods_delete_remove_resource(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('DELETE', '/items/1', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals([], $response->body);
    }

    public function test_http_methods_delete_resource_not_found(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('DELETE', '/items/999', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals([], $response->body);
    }

    public function test_http_methods_delete_with_response_body(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('DELETE', '/items/1', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => 1, 'message' => 'Item deleted successfully', 'name' => 'Deleted Item'], $response->body);
    }

    public function test_http_methods_head_get_metadata_without_body(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('HEAD', '/items/1', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_http_methods_options_cors_preflight_request(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('OPTIONS', '/items/', ['headers' => ['Origin' => 'https://example.com', 'Access-Control-Request-Method' => 'POST', 'Access-Control-Request-Headers' => 'Content-Type']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_http_methods_patch_partial_update(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('PATCH', '/items/1', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['price' => 79.99]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => 1, 'in_stock' => true, 'name' => 'Existing Item', 'price' => 79.99], $response->body);
    }

    public function test_http_methods_patch_update_multiple_fields(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('PATCH', '/items/1', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['in_stock' => false, 'name' => 'Updated Name', 'price' => 89.99]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => 1, 'in_stock' => false, 'name' => 'Updated Name', 'price' => 89.99], $response->body);
    }

    public function test_http_methods_put_complete_resource_replacement(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/1', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['description' => 'Completely replaced', 'id' => 1, 'in_stock' => true, 'name' => 'Updated Item', 'price' => 99.99]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['description' => 'Completely replaced', 'id' => 1, 'in_stock' => true, 'name' => 'Updated Item', 'price' => 99.99], $response->body);
    }

    public function test_http_methods_put_create_resource_if_doesn_t_exist(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/999', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['id' => 999, 'name' => 'New Item', 'price' => 49.99]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => 999, 'name' => 'New Item', 'price' => 49.99], $response->body);
    }

    public function test_http_methods_put_idempotent_operation(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/1', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['id' => 1, 'name' => 'Fixed Name', 'price' => 50.0]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => 1, 'name' => 'Fixed Name', 'price' => 50.0], $response->body);
    }

    public function test_http_methods_put_missing_required_field(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/1', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['id' => 1, 'name' => 'Item Name']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => '1', 'loc' => ['body', 'price'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_http_methods_put_validation_error(): void
    {
        $app = AppFactory::create_http_methods();
        $client = TestClient::create($app);
        $response = $client->request('PUT', '/items/1', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['id' => 1, 'name' => 'X', 'price' => -10]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '2 validation errors in request', 'errors' => [['input' => 'X', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short'], ['input' => -10, 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than 0', 'type' => 'greater_than']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_29_nested_object_validation_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ['profile' => ['email' => 'john@example.com', 'name' => 'John Doe']]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_30_nested_object_missing_field(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ['profile' => ['name' => 'John Doe']]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['required' => true], 'loc' => ['body', 'profile', 'email'], 'msg' => 'Field required', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_31_nullable_property_null_value(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ['description' => null, 'name' => 'Test User']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_32_schema_ref_definitions(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/products', ['body' => ['product' => ['name' => 'Widget', 'price' => 9.99]]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_33_allof_schema_composition(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items', ['body' => ['name' => 'Product', 'price' => 29.99]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_34_additional_properties_false(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ['email' => 'john@example.com', 'extra_field' => 'should fail', 'name' => 'John']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['additional_properties' => false, 'unexpected_field' => 'extra_field'], 'loc' => ['body', 'extra_field'], 'msg' => 'Additional properties are not allowed', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_35_oneof_schema_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/payment', ['body' => ['credit_card' => '1234567812345678']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_36_oneof_schema_multiple_match_failure(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/payment', ['body' => ['credit_card' => '1234567812345678', 'paypal_email' => 'user@example.com']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['matched_schemas' => 2], 'loc' => ['body'], 'msg' => 'Must match exactly one schema (oneOf), but matched 2', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_37_oneof_schema_no_match_failure(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/payment', ['body' => ['bitcoin_address' => '1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['matched_schemas' => 0], 'loc' => ['body'], 'msg' => 'Must match exactly one schema (oneOf), but matched 0', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_38_anyof_schema_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/contact', ['body' => ['email' => 'john@example.com', 'name' => 'John Doe']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_39_anyof_schema_multiple_match_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/contact', ['body' => ['email' => 'john@example.com', 'name' => 'John Doe', 'phone' => '+1-555-0100']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_40_anyof_schema_failure(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/contact', ['body' => ['name' => 'John Doe']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['matched_schemas' => 0], 'loc' => ['body'], 'msg' => 'Must match at least one schema (anyOf), but matched 0', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_41_not_schema_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ['username' => 'john_doe']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_42_not_schema_failure(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ['username' => 'admin']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['prohibited_value' => 'admin'], 'loc' => ['body', 'username'], 'msg' => 'Must not match the schema', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_43_const_validation_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/v1/data', ['body' => ['data' => 'test', 'version' => '1.0']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_44_const_validation_failure(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/v1/data', ['body' => ['data' => 'test', 'version' => '2.0']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['const' => '1.0', 'value' => '2.0'], 'loc' => ['body', 'version'], 'msg' => 'Value must be exactly \'1.0\'', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_45_minproperties_validation_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/config', ['body' => ['host' => 'localhost', 'port' => 8080]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_46_minproperties_validation_failure(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/config', ['body' => ['host' => 'localhost']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_properties' => 1, 'min_properties' => 2], 'loc' => ['body'], 'msg' => 'Object must have at least 2 properties', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_47_maxproperties_validation_failure(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/config', ['body' => ['debug' => false, 'host' => 'localhost', 'port' => 8080, 'ssl' => true]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_properties' => 4, 'max_properties' => 3], 'loc' => ['body'], 'msg' => 'Object must have at most 3 properties', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_48_dependencies_validation_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/billing', ['body' => ['billing_address' => '123 Main St', 'credit_card' => '1234567812345678', 'name' => 'John Doe']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_49_dependencies_validation_failure(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/billing', ['body' => ['credit_card' => '1234567812345678', 'name' => 'John Doe']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['dependency' => 'credit_card', 'required_fields' => ['billing_address']], 'loc' => ['body'], 'msg' => 'When \'credit_card\' is present, \'billing_address\' is required', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_50_deep_nesting_4_levels(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['body' => ['user' => ['profile' => ['contact' => ['address' => ['street' => '123 Main St']]]]]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_json_bodies_array_of_objects_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/list', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['images' => [['name' => 'Front', 'url' => 'https://example.com/img1.jpg'], ['name' => 'Back', 'url' => 'https://example.com/img2.jpg']], 'name' => 'Product Bundle', 'tags' => ['electronics', 'gadget']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['images' => [['name' => 'Front', 'url' => 'https://example.com/img1.jpg'], ['name' => 'Back', 'url' => 'https://example.com/img2.jpg']], 'name' => 'Product Bundle', 'tags' => ['electronics', 'gadget']], $response->body);
    }

    public function test_json_bodies_array_of_primitive_values(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Product', 'ratings' => [4.5, 4.8, 5.0, 4.2], 'tags' => ['electronics', 'gadget', 'new']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['name' => 'Product', 'ratings' => [4.5, 4.8, 5.0, 4.2], 'tags' => ['electronics', 'gadget', 'new']], $response->body);
    }

    public function test_json_bodies_body_with_query_parameters(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/?limit=10?limit=10', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item', 'price' => 42.0]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item' => ['name' => 'Item', 'price' => 42.0], 'limit' => 10], $response->body);
    }

    public function test_json_bodies_boolean_field_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['in_stock' => true, 'name' => 'Item', 'price' => 42.0]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['in_stock' => true, 'name' => 'Item', 'price' => 42.0], $response->body);
    }

    public function test_json_bodies_date_field_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/events/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['event_date' => '2024-03-15', 'name' => 'Conference']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['event_date' => '2024-03-15', 'name' => 'Conference'], $response->body);
    }

    public function test_json_bodies_datetime_field_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/events/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['created_at' => '2024-03-15T10:30:00Z', 'name' => 'Meeting']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['created_at' => '2024-03-15T10:30:00Z', 'name' => 'Meeting'], $response->body);
    }

    public function test_json_bodies_deeply_nested_objects(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/nested', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Product', 'price' => 100.0, 'seller' => ['address' => ['city' => 'Springfield', 'country' => ['code' => 'US', 'name' => 'USA'], 'street' => '123 Main St'], 'name' => 'John Doe']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['name' => 'Product', 'price' => 100.0, 'seller' => ['address' => ['city' => 'Springfield', 'country' => ['code' => 'US', 'name' => 'USA'], 'street' => '123 Main St'], 'name' => 'John Doe']], $response->body);
    }

    public function test_json_bodies_empty_json_object(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/optional-all', ['headers' => ['Content-Type' => 'application/json'], 'body' => []]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['description' => null, 'name' => null, 'price' => null, 'tax' => null], $response->body);
    }

    public function test_json_bodies_empty_array_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/list-validated', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Product', 'tags' => []]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 1], 'input' => [], 'loc' => ['body', 'tags'], 'msg' => 'List should have at least 1 item after validation', 'type' => 'too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_enum_field_invalid_value(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['category' => 'furniture', 'name' => 'Item']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'electronics\', \'clothing\' or \'books\''], 'input' => 'furniture', 'loc' => ['body', 'category'], 'msg' => 'Input should be \'electronics\', \'clothing\' or \'books\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_enum_field_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['category' => 'electronics', 'name' => 'Item']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['category' => 'electronics', 'name' => 'Item'], $response->body);
    }

    public function test_json_bodies_extra_fields_ignored_no_additionalproperties(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['another_extra' => 123, 'extra_field' => 'this should be ignored', 'name' => 'Item', 'price' => 42.0]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['name' => 'Item', 'price' => 42.0], $response->body);
    }

    public function test_json_bodies_field_type_validation_invalid_type(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['description' => 'A very nice Item', 'name' => 'Foo', 'price' => 'not a number', 'tax' => 3.2]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'not a number', 'loc' => ['body', 'price'], 'msg' => 'Input should be a valid number', 'type' => 'float_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_nested_object_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/nested', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['image' => ['name' => 'Product Image', 'url' => 'https://example.com/image.jpg'], 'name' => 'Foo', 'price' => 42.0]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['image' => ['name' => 'Product Image', 'url' => 'https://example.com/image.jpg'], 'name' => 'Foo', 'price' => 42.0], $response->body);
    }

    public function test_json_bodies_null_value_for_optional_field(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['description' => null, 'name' => 'Item', 'price' => 42.0, 'tax' => null]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['description' => null, 'name' => 'Item', 'price' => 42.0, 'tax' => null], $response->body);
    }

    public function test_json_bodies_numeric_ge_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item', 'price' => 0.5]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['ge' => 1], 'input' => 0.5, 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than or equal to 1', 'type' => 'greater_than_equal']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_numeric_le_validation_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item', 'price' => 100.0]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['name' => 'Item', 'price' => 100.0], $response->body);
    }

    public function test_json_bodies_optional_fields_omitted(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Foo', 'price' => 35.4]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['description' => null, 'name' => 'Foo', 'price' => 35.4, 'tax' => null], $response->body);
    }

    public function test_json_bodies_patch_partial_update(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('PATCH', '/items/1', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['price' => 45.0]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['description' => 'Original description', 'name' => 'Original Item', 'price' => 45.0], $response->body);
    }

    public function test_json_bodies_required_field_missing_validation_error(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['description' => 'A very nice Item', 'price' => 35.4]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['body', 'name'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_simple_json_object_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['description' => 'A very nice Item', 'name' => 'Foo', 'price' => 35.4, 'tax' => 3.2]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['description' => 'A very nice Item', 'name' => 'Foo', 'price' => 35.4, 'tax' => 3.2], $response->body);
    }

    public function test_json_bodies_string_max_length_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'This is a very long name that exceeds the maximum length', 'price' => 35.4]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 50], 'input' => 'This is a very long name that exceeds the maximum length', 'loc' => ['body', 'name'], 'msg' => 'String should have at most 50 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_string_min_length_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'ab', 'price' => 35.4]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_string_pattern_validation_fail(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item', 'sku' => 'ABC-123']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[A-Z]{3}[0-9]{4}$'], 'input' => 'ABC-123', 'loc' => ['body', 'sku'], 'msg' => 'String should match pattern \'^[A-Z]{3}[0-9]{4}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_string_pattern_validation_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/validated', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item', 'sku' => 'ABC1234']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['name' => 'Item', 'sku' => 'ABC1234'], $response->body);
    }

    public function test_json_bodies_uuid_field_invalid_format(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['item_id' => 'not-a-valid-uuid', 'name' => 'Item']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-valid-uuid', 'loc' => ['body', 'item_id'], 'msg' => 'Input should be a valid UUID', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_json_bodies_uuid_field_success(): void
    {
        $app = AppFactory::create_json_bodies();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['item_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716', 'name' => 'Item']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716', 'name' => 'Item'], $response->body);
    }

    public function test_lifecycle_hooks_hook_execution_order(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-hook-order', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['execution_order' => ['first_hook', 'second_hook', 'third_hook'], 'message' => 'Hooks executed in order'], $response->body);
    }

    public function test_lifecycle_hooks_multiple_hooks_all_phases(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/full-lifecycle', ['headers' => ['Authorization' => 'Bearer valid-token-12345', 'Content-Type' => 'application/json'], 'body' => ['action' => 'update_profile', 'user_id' => 'user-123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['action' => 'update_profile', 'message' => 'Action completed successfully', 'request_id' => '.*', 'user_id' => 'user-123'], $response->body);
    }

    public function test_lifecycle_hooks_onerror_error_logging(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-error', []);

        $this->assertSame(500, $response->statusCode);
        $this->assertEquals(['error' => 'Internal Server Error', 'error_id' => '.*', 'message' => 'An unexpected error occurred'], $response->body);
    }

    public function test_lifecycle_hooks_onrequest_request_logging(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-on-request', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['has_request_id' => true, 'message' => 'onRequest hooks executed', 'request_logged' => true], $response->body);
    }

    public function test_lifecycle_hooks_onresponse_response_timing(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-timing', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Response with timing info'], $response->body);
    }

    public function test_lifecycle_hooks_onresponse_security_headers(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/test-security-headers', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Response with security headers'], $response->body);
    }

    public function test_lifecycle_hooks_prehandler_authentication_failed_short_circuit(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected-resource-fail', ['headers' => ['Authorization' => 'Bearer invalid-token']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['error' => 'Unauthorized', 'message' => 'Invalid or expired authentication token'], $response->body);
    }

    public function test_lifecycle_hooks_prehandler_authentication_success(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected-resource', ['headers' => ['Authorization' => 'Bearer valid-token-12345']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['authenticated' => true, 'message' => 'Access granted', 'user_id' => 'user-123'], $response->body);
    }

    public function test_lifecycle_hooks_prehandler_authorization_check(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/admin-only', ['headers' => ['Authorization' => 'Bearer admin-token-67890']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Admin access granted', 'role' => 'admin', 'user_id' => 'admin-456'], $response->body);
    }

    public function test_lifecycle_hooks_prehandler_authorization_forbidden_short_circuit(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/admin-only-forbidden', ['headers' => ['Authorization' => 'Bearer user-token-11111']]);

        $this->assertSame(403, $response->statusCode);
        $this->assertEquals(['error' => 'Forbidden', 'message' => 'Admin role required for this endpoint'], $response->body);
    }

    public function test_lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/test-rate-limit-exceeded', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['data' => 'test']]);

        $this->assertSame(429, $response->statusCode);
        $this->assertEquals(['error' => 'Rate limit exceeded', 'message' => 'Too many requests, please try again later'], $response->body);
    }

    public function test_lifecycle_hooks_prevalidation_rate_limiting(): void
    {
        $app = AppFactory::create_lifecycle_hooks();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/api/test-rate-limit', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['data' => 'test']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['message' => 'Request accepted', 'rate_limit_checked' => true], $response->body);
    }

    public function test_multipart_17_file_magic_number_png_success(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [['content_type' => 'image/png', 'field_name' => 'image', 'filename' => 'test.png', 'magic_bytes' => '89504e470d0a1a0a']]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_multipart_18_file_magic_number_jpeg_success(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [['content_type' => 'image/jpeg', 'field_name' => 'image', 'filename' => 'test.jpg', 'magic_bytes' => 'ffd8ffe0']]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_multipart_19_file_mime_spoofing_png_as_jpeg(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [['content_type' => 'image/jpeg', 'field_name' => 'image', 'filename' => 'fake.jpg', 'magic_bytes' => '89504e470d0a1a0a']]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['declared_mime' => 'image/jpeg', 'detected_type' => 'image/png', 'magic_bytes' => '89504e470d0a1a0a'], 'loc' => ['files', 'image'], 'msg' => 'File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_multipart_20_file_mime_spoofing_jpeg_as_png(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [['content_type' => 'image/png', 'field_name' => 'image', 'filename' => 'fake.png', 'magic_bytes' => 'ffd8ffe0']]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['declared_mime' => 'image/png', 'detected_type' => 'image/jpeg', 'magic_bytes' => 'ffd8ffe0'], 'loc' => ['files', 'image'], 'msg' => 'File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_multipart_21_file_pdf_magic_number_success(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [['content_type' => 'application/pdf', 'field_name' => 'document', 'filename' => 'test.pdf', 'magic_bytes' => '25504446']]]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_multipart_22_file_empty_buffer(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['files' => [['content_type' => 'text/plain', 'field_name' => 'file', 'filename' => 'empty.txt', 'magic_bytes' => '']]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['buffer_size' => 0], 'loc' => ['files', 'file'], 'msg' => 'File buffer is empty', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_multipart_content_type_validation_invalid_type(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/images-only', ['files' => [['content' => '#!/bin/bash
echo hello', 'content_type' => 'application/x-sh', 'field_name' => 'file', 'filename' => 'script.sh']]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_multipart_empty_file_upload(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/upload', ['files' => [['content' => '', 'content_type' => 'text/plain', 'field_name' => 'file', 'filename' => 'empty.txt']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['filename' => 'empty.txt', 'size' => 0], $response->body);
    }

    public function test_multipart_file_list_upload_array_of_files(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/list', ['files' => [['content' => 'content of file 1', 'content_type' => 'text/plain', 'field_name' => 'files', 'filename' => 'file1.txt'], ['content' => 'content of file 2', 'content_type' => 'text/plain', 'field_name' => 'files', 'filename' => 'file2.txt']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['filenames' => ['file1.txt', 'file2.txt'], 'total_size' => 35], $response->body);
    }

    public function test_multipart_file_size_validation_too_large(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/validated', ['files' => [['content' => 'x', 'content_type' => 'text/plain', 'field_name' => 'file', 'filename' => 'large.txt']]]);

        $this->assertSame(413, $response->statusCode);
        $this->assertEquals(['detail' => 'File too large. Maximum size is 1MB'], $response->body);
    }

    public function test_multipart_file_upload_with_custom_headers(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['files' => [['content' => '<file2 content>', 'content_encoding' => 'text', 'content_type' => 'text/plain', 'field_name' => 'test2', 'filename' => 'test2.txt']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['test2' => ['content' => '<file2 content>', 'content_type' => 'text/plain', 'filename' => 'test2.txt', 'headers' => [['content-disposition', 'form-data; name="test2"; filename="test2.txt"'], ['content-type', 'text/plain'], ['x-custom', 'f2']], 'size' => 15]], $response->body);
    }

    public function test_multipart_file_upload_without_filename(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['files' => [['content' => '<file1 content>', 'content_encoding' => 'text', 'field_name' => 'test1']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['test1' => '<file1 content>'], $response->body);
    }

    public function test_multipart_form_data_without_files(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['some' => 'data'], $response->body);
    }

    public function test_multipart_image_file_upload(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/image', ['files' => [['content' => 'fake_jpeg_content_here', 'content_type' => 'image/jpeg', 'field_name' => 'image', 'filename' => 'photo.jpg']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['content_type' => 'image/jpeg', 'filename' => 'photo.jpg', 'size' => 22], $response->body);
    }

    public function test_multipart_mixed_files_and_form_data(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['files' => [['content' => 'file data here', 'content_encoding' => 'text', 'content_type' => 'text/plain', 'field_name' => 'file', 'filename' => 'upload.txt']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['active' => 'true', 'age' => '25', 'file' => ['content' => 'file data here', 'content_type' => 'text/plain', 'filename' => 'upload.txt', 'size' => 14], 'username' => 'testuser'], $response->body);
    }

    public function test_multipart_multiple_file_uploads(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['files' => [['content' => '<file1 content>', 'content_encoding' => 'text', 'content_type' => 'text/plain', 'field_name' => 'test1', 'filename' => 'test1.txt'], ['content' => '<file2 content>', 'content_encoding' => 'text', 'content_type' => 'text/plain', 'field_name' => 'test2', 'filename' => 'test2.txt']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['test1' => ['content' => '<file1 content>', 'content_type' => 'text/plain', 'filename' => 'test1.txt', 'size' => 15], 'test2' => ['content' => '<file2 content>', 'content_type' => 'text/plain', 'filename' => 'test2.txt', 'size' => 15]], $response->body);
    }

    public function test_multipart_multiple_values_for_same_field_name(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['files' => [['content' => 'first file', 'content_encoding' => 'text', 'content_type' => 'text/plain', 'field_name' => 'files', 'filename' => 'file1.txt'], ['content' => 'second file', 'content_encoding' => 'text', 'content_type' => 'text/plain', 'field_name' => 'files', 'filename' => 'file2.txt']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['files' => [['content' => 'first file', 'content_type' => 'text/plain', 'filename' => 'file1.txt', 'size' => 10], ['content' => 'second file', 'content_type' => 'text/plain', 'filename' => 'file2.txt', 'size' => 11]], 'tags' => ['python', 'rust', 'web']], $response->body);
    }

    public function test_multipart_optional_file_upload_missing(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/optional', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['file' => null], $response->body);
    }

    public function test_multipart_optional_file_upload_provided(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/optional', ['files' => [['content' => 'optional file content', 'content_type' => 'text/plain', 'field_name' => 'file', 'filename' => 'optional.txt']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['content_type' => 'text/plain', 'filename' => 'optional.txt', 'size' => 21], $response->body);
    }

    public function test_multipart_pdf_file_upload(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/document', ['files' => [['content' => 'fake_pdf_content', 'content_type' => 'application/pdf', 'field_name' => 'document', 'filename' => 'report.pdf']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['content_type' => 'application/pdf', 'filename' => 'report.pdf', 'size' => 16], $response->body);
    }

    public function test_multipart_required_file_upload_missing(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/files/required', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'required', 'loc' => ['body', 'file'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_multipart_simple_file_upload(): void
    {
        $app = AppFactory::create_multipart();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/', ['files' => [['content' => '<file content>', 'content_encoding' => 'text', 'content_type' => 'text/plain', 'field_name' => 'test', 'filename' => 'test.txt']]]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['test' => ['content' => '<file content>', 'content_type' => 'text/plain', 'filename' => 'test.txt', 'size' => 14]], $response->body);
    }

    public function test_path_params_20_uuid_v3_path_param_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/e8b5a51d-11c8-3310-a6ab-367563f20686', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => 'e8b5a51d-11c8-3310-a6ab-367563f20686'], $response->body);
    }

    public function test_path_params_21_uuid_v5_path_param_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/630eb68f-e0fa-5ecc-887a-7c7a62614681', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => '630eb68f-e0fa-5ecc-887a-7c7a62614681'], $response->body);
    }

    public function test_path_params_24_date_format_path_param_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/events/2025-10-30', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['date' => '2025-10-30'], $response->body);
    }

    public function test_path_params_25_date_format_invalid_failure(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/events/2025-13-45', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'date', 'value' => '2025-13-45'], 'loc' => ['path', 'date'], 'msg' => 'Invalid date format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_27_datetime_format_path_param_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/bookings/2025-10-30T14:30:00Z', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['timestamp' => '2025-10-30T14:30:00Z'], $response->body);
    }

    public function test_path_params_28_duration_format_path_param_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/delays/P1DT2H30M', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['duration' => 'P1DT2H30M'], $response->body);
    }

    public function test_path_params_29_decimal_path_param_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/prices/19.99', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['amount' => '19.99'], $response->body);
    }

    public function test_path_params_30_string_minlength_path_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/alice', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['username' => 'alice'], $response->body);
    }

    public function test_path_params_31_string_minlength_path_failure(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/ab', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 2, 'min_length' => 3], 'loc' => ['path', 'username'], 'msg' => 'String length must be at least 3', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_32_string_maxlength_path_failure(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/this_username_is_way_too_long_to_be_valid', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 42, 'max_length' => 20], 'loc' => ['path', 'username'], 'msg' => 'String length must not exceed 20', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_33_string_pattern_path_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/repos/spikard-labs/spikard-http', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['owner' => 'spikard-labs', 'repo' => 'spikard-http'], $response->body);
    }

    public function test_path_params_34_string_pattern_path_failure(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/repos/invalid@owner', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9-]+$', 'value' => 'invalid@owner'], 'loc' => ['path', 'owner'], 'msg' => 'String does not match pattern', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_35_negative_integer_path_param(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/offset/-100', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['value' => -100], $response->body);
    }

    public function test_path_params_boolean_path_parameter_true(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/bool/True', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => true], $response->body);
    }

    public function test_path_params_boolean_path_parameter_numeric_1(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/bool/1', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => true], $response->body);
    }

    public function test_path_params_date_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/date/2023-07-15', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['date_param' => '2023-07-15'], $response->body);
    }

    public function test_path_params_enum_path_parameter_invalid_value(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/models/foo', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\''], 'input' => 'foo', 'loc' => ['path', 'model_name'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_enum_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/models/alexnet', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['model_name' => 'alexnet'], $response->body);
    }

    public function test_path_params_float_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/float/42.5', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 42.5], $response->body);
    }

    public function test_path_params_integer_path_parameter_invalid_string(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/int/foobar', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'foobar', 'loc' => ['path', 'item_id'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_integer_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/int/42', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 42], $response->body);
    }

    public function test_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-lt-gt/2', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 2], $response->body);
    }

    public function test_path_params_integer_path_parameter_with_ge_constraint_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-ge/3', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 3], $response->body);
    }

    public function test_path_params_integer_path_parameter_with_gt_constraint_failure(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-gt/2', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['gt' => 3], 'input' => 2, 'loc' => ['path', 'item_id'], 'msg' => 'Input should be greater than 3', 'type' => 'greater_than']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_integer_path_parameter_with_gt_constraint_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-gt/42', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 42], $response->body);
    }

    public function test_path_params_integer_path_parameter_with_le_constraint_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-le/3', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 3], $response->body);
    }

    public function test_path_params_integer_path_parameter_with_lt_constraint_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-lt/2', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 2], $response->body);
    }

    public function test_path_params_multiple_path_parameters_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['order_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716', 'service_id' => 1, 'user_id' => 'abc', 'version' => 1.0], $response->body);
    }

    public function test_path_params_path_parameter_type_syntax_invalid_uuid(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/type-syntax/items/not-a-uuid', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-uuid', 'loc' => ['path', 'id'], 'msg' => 'Input should be a valid UUID', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_path_parameter_type_syntax_with_override(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/type-syntax/items-count/50', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['count' => '50'], $response->body);
    }

    public function test_path_params_path_parameter_with_type_syntax_uuid(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/type-syntax/items/550e8400-e29b-41d4-a716-446655440000', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => '550e8400-e29b-41d4-a716-446655440000'], $response->body);
    }

    public function test_path_params_path_parameter_with_type_syntax_integer(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/type-syntax/users/42', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['user_id' => '42'], $response->body);
    }

    public function test_path_params_path_type_parameter_file_path(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/files/home/johndoe/myfile.txt', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['file_path' => 'home/johndoe/myfile.txt'], $response->body);
    }

    public function test_path_params_string_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/str/foobar', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 'foobar'], $response->body);
    }

    public function test_path_params_string_path_parameter_with_max_length_failure(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-maxlength/foobar', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 3], 'input' => 'foobar', 'loc' => ['path', 'item_id'], 'msg' => 'String should have at most 3 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_string_path_parameter_with_min_length_failure(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/path/param-minlength/fo', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'fo', 'loc' => ['path', 'item_id'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_path_params_uuid_path_parameter_success(): void
    {
        $app = AppFactory::create_path_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 'ec38df32-ceda-4cfa-9b4a-1aeb94ad551a'], $response->body);
    }

    public function test_query_params_42_negative_integer_query_param(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/negative?offset=%22-10%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['offset' => -10], $response->body);
    }

    public function test_query_params_43_scientific_notation_float(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/stats?threshold=%221.5e-3%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['threshold' => 0.0015], $response->body);
    }

    public function test_query_params_44_string_minlength_validation_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=%22foo%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['term' => 'foo'], $response->body);
    }

    public function test_query_params_45_string_minlength_validation_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=%22ab%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 2, 'min_length' => 3], 'loc' => ['query', 'term'], 'msg' => 'String length must be at least 3', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_46_string_maxlength_validation_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?term=%22this_is_way_too_long%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 21, 'max_length' => 10], 'loc' => ['query', 'term'], 'msg' => 'String length must not exceed 10', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_47_pattern_validation_email_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/subscribe?email=%22user%40example.com%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['email' => 'user@example.com'], $response->body);
    }

    public function test_query_params_48_pattern_validation_email_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/subscribe?email=%22invalid-email%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$', 'value' => 'invalid-email'], 'loc' => ['query', 'email'], 'msg' => 'String does not match pattern', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_49_integer_gt_constraint_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?limit=%225%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['limit' => 5], $response->body);
    }

    public function test_query_params_50_integer_gt_constraint_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?limit=%220%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['exclusive_minimum' => 0, 'value' => 0], 'loc' => ['query', 'limit'], 'msg' => 'Value must be greater than 0', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_51_integer_ge_constraint_boundary(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?offset=%220%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['offset' => 0], $response->body);
    }

    public function test_query_params_52_integer_le_constraint_boundary(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?limit=%22100%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['limit' => 100], $response->body);
    }

    public function test_query_params_53_integer_le_constraint_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?limit=%22101%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['maximum' => 100, 'value' => 101], 'loc' => ['query', 'limit'], 'msg' => 'Value must not exceed 100', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_54_array_minitems_constraint_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?ids=%221%22&ids=%222%22&ids=%223%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['ids' => [1, 2, 3]], $response->body);
    }

    public function test_query_params_55_array_minitems_constraint_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?ids=%221%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_items' => 1, 'min_items' => 2], 'loc' => ['query', 'ids'], 'msg' => 'Array must contain at least 2 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_56_array_maxitems_constraint_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?tags=%22a%22&tags=%22b%22&tags=%22c%22&tags=%22d%22&tags=%22e%22&tags=%22f%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_items' => 6, 'max_items' => 5], 'loc' => ['query', 'tags'], 'msg' => 'Array must not contain more than 5 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_57_boolean_empty_string_coercion(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?active=%22%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['active' => false], $response->body);
    }

    public function test_query_params_58_format_email_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/subscribe?email=%22user%40example.com%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['email' => 'user@example.com'], $response->body);
    }

    public function test_query_params_59_format_email_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/subscribe?email=%22not-an-email%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'email', 'value' => 'not-an-email'], 'loc' => ['query', 'email'], 'msg' => 'Invalid email format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_60_format_ipv4_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/network?ip=%22192.168.1.1%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['ip' => '192.168.1.1'], $response->body);
    }

    public function test_query_params_61_format_ipv4_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/network?ip=%22999.999.999.999%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'ipv4', 'value' => '999.999.999.999'], 'loc' => ['query', 'ip'], 'msg' => 'Invalid IPv4 address format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_62_format_ipv6_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/network/ipv6?ip=%222001%3A0db8%3A85a3%3A0000%3A0000%3A8a2e%3A0370%3A7334%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['ip' => '2001:0db8:85a3:0000:0000:8a2e:0370:7334'], $response->body);
    }

    public function test_query_params_63_format_uri_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/redirect?url=%22https%3A%2F%2Fexample.com%2Fpath%3Fquery%3Dvalue%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['url' => 'https://example.com/path?query=value'], $response->body);
    }

    public function test_query_params_64_format_uri_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/redirect?url=%22not%20a%20uri%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'uri', 'value' => 'not a uri'], 'loc' => ['query', 'url'], 'msg' => 'Invalid URI format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_65_format_hostname_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/dns?host=%22api.example.com%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['host' => 'api.example.com'], $response->body);
    }

    public function test_query_params_66_multipleof_constraint_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?quantity=%2215%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['quantity' => 15], $response->body);
    }

    public function test_query_params_67_multipleof_constraint_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?quantity=%2217%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['multiple_of' => 5, 'value' => 17], 'loc' => ['query', 'quantity'], 'msg' => 'Value must be a multiple of 5', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_68_array_uniqueitems_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?ids=%221%22&ids=%222%22&ids=%223%22&ids=%224%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['ids' => [1, 2, 3, 4]], $response->body);
    }

    public function test_query_params_69_array_uniqueitems_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?ids=%221%22&ids=%222%22&ids=%222%22&ids=%223%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['duplicate_index' => 2, 'duplicate_value' => 2, 'unique_items' => true], 'loc' => ['query', 'ids'], 'msg' => 'Array items must be unique', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_70_array_separator_pipe(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?tags=python|rust|typescript?tags=%22python%7Crust%7Ctypescript%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['tags' => ['python', 'rust', 'typescript']], $response->body);
    }

    public function test_query_params_71_array_separator_semicolon(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items?colors=red;green;blue?colors=%22red%3Bgreen%3Bblue%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['colors' => ['red', 'green', 'blue']], $response->body);
    }

    public function test_query_params_72_array_separator_space(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/search?keywords=rust%20web%20framework?keywords=%22rust%20web%20framework%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['keywords' => ['rust', 'web', 'framework']], $response->body);
    }

    public function test_query_params_array_query_parameter_empty_array(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list-default', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals([], $response->body);
    }

    public function test_query_params_array_query_parameter_single_value(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list-default?tags=%22apple%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['apple'], $response->body);
    }

    public function test_query_params_boolean_query_parameter_numeric_1(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/bool?flag=%221%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['flag' => true], $response->body);
    }

    public function test_query_params_boolean_query_parameter_true(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/bool?flag=%22true%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['flag' => true], $response->body);
    }

    public function test_query_params_date_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/date?event_date=%222024-01-15%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['event_date' => '2024-01-15'], $response->body);
    }

    public function test_query_params_datetime_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/datetime?timestamp=%222024-01-15T10%3A30%3A00Z%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['timestamp' => '2024-01-15T10:30:00Z'], $response->body);
    }

    public function test_query_params_enum_query_parameter_invalid_value(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/enum?model=%22vgg16%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\''], 'input' => 'vgg16', 'loc' => ['query', 'model'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_enum_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/enum?model=%22alexnet%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['model' => 'alexnet'], $response->body);
    }

    public function test_query_params_float_query_param_with_ge_constraint_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/float-ge?price=%220.01%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['price' => 0.01], $response->body);
    }

    public function test_query_params_integer_query_param_with_ge_constraint_boundary(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int-ge?value=%2210%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['value' => 10], $response->body);
    }

    public function test_query_params_integer_query_param_with_gt_constraint_valid(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int-gt?value=%221%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['value' => 1], $response->body);
    }

    public function test_query_params_integer_query_param_with_le_constraint_boundary(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int-le?value=%22100%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['value' => 100], $response->body);
    }

    public function test_query_params_integer_query_param_with_lt_constraint_valid(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int-lt?value=%2249%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['value' => 49], $response->body);
    }

    public function test_query_params_integer_with_default_value_not_provided(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int/default', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('foo bar 10', $response->body);
    }

    public function test_query_params_integer_with_default_value_override(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int/default?query=50', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('foo bar 50', $response->body);
    }

    public function test_query_params_list_of_integers_multiple_values(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list?device_ids=1&device_ids=2', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals([1, 2], $response->body);
    }

    public function test_query_params_list_of_strings_multiple_values(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=%22foo%22&q=%22bar%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['q' => ['foo', 'bar']], $response->body);
    }

    public function test_query_params_list_query_parameter_required_but_missing(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'device_ids'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_list_with_default_empty_array_no_values_provided(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/list-default', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals([], $response->body);
    }

    public function test_query_params_multiple_query_parameters_with_different_types(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/multi-type?score=%2295.5%22&active=%22true%22&name=%22john%22&age=%2230%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['active' => true, 'age' => 30, 'name' => 'john', 'score' => 95.5], $response->body);
    }

    public function test_query_params_optional_integer_query_parameter_missing(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int/optional', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('foo bar None', $response->body);
    }

    public function test_query_params_optional_query_parameter_with_default_value(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/optional-default', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['limit' => 10], $response->body);
    }

    public function test_query_params_optional_string_query_parameter_missing(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/optional', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('foo bar None', $response->body);
    }

    public function test_query_params_optional_string_query_parameter_provided(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/optional?query=%22baz%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('foo bar baz', $response->body);
    }

    public function test_query_params_query_parameter_with_url_encoded_space(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/basic?name=%22hello%20world%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['name' => 'hello world'], $response->body);
    }

    public function test_query_params_query_parameter_with_url_encoded_special_characters(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/basic?name=%22test%26value%3D123%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['name' => 'test&value=123'], $response->body);
    }

    public function test_query_params_query_parameter_with_special_characters_url_encoding(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/test?email=%22x%40test.com%22&special=%22%26%40A.ac%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['email' => 'x@test.com', 'special' => '&@A.ac'], $response->body);
    }

    public function test_query_params_required_integer_query_parameter_float_value(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int?query=%2242.5%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 42.5, 'loc' => ['query', 'query'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_required_integer_query_parameter_invalid_type(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int?query=%22baz%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'baz', 'loc' => ['query', 'query'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_required_integer_query_parameter_missing(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'query'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_required_integer_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/int?query=42', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('foo bar 42', $response->body);
    }

    public function test_query_params_required_string_query_parameter_missing(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'query'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_required_string_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query?query=%22baz%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('foo bar baz', $response->body);
    }

    public function test_query_params_string_query_param_with_max_length_constraint_fail(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/str-max-length?name=%22this_is_way_too_long%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 10], 'input' => 'this_is_way_too_long', 'loc' => ['query', 'name'], 'msg' => 'String should have at most 10 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_string_query_param_with_min_length_constraint_fail(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/str-min-length?name=%22ab%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['query', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_string_query_param_with_regex_pattern_fail(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/pattern?code=%22abc123%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[0-9]{3,}$'], 'input' => 'abc123', 'loc' => ['query', 'code'], 'msg' => 'String should match pattern \'^[0-9]{3,}$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_string_validation_with_regex_failure(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?item_query=%22nonregexquery%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^fixedquery$'], 'input' => 'nonregexquery', 'loc' => ['query', 'item_query'], 'msg' => 'String should match pattern \'^fixedquery$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_string_validation_with_regex_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?item_query=%22fixedquery%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_query' => 'fixedquery'], $response->body);
    }

    public function test_query_params_uuid_query_parameter_invalid_format(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/uuid?item_id=%22not-a-uuid%22', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-uuid', 'loc' => ['query', 'item_id'], 'msg' => 'Input should be a valid UUID', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_query_params_uuid_query_parameter_success(): void
    {
        $app = AppFactory::create_query_params();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/query/uuid?item_id=%22c892496f-b1fd-4b91-bdb8-b46f92df1716%22', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['item_id' => 'c892496f-b1fd-4b91-bdb8-b46f92df1716'], $response->body);
    }

    public function test_rate_limit_rate_limit_below_threshold_succeeds(): void
    {
        $app = AppFactory::create_rate_limit();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/rate-limit/basic', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['request' => 'under-limit', 'status' => 'ok'], $response->body);
    }

    public function test_rate_limit_rate_limit_exceeded_returns_429(): void
    {
        $app = AppFactory::create_rate_limit();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/rate-limit/exceeded', []);

        $this->assertSame(429, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_request_id_request_id_header_is_preserved(): void
    {
        $app = AppFactory::create_request_id();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/request-id/preserved', ['headers' => ['X-Request-ID' => 'trace-123']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['echo' => 'trace-123', 'status' => 'preserved'], $response->body);
    }

    public function test_request_id_request_id_is_generated_when_not_provided(): void
    {
        $app = AppFactory::create_request_id();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/request-id/generated', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['status' => 'generated'], $response->body);
    }

    public function test_request_id_request_id_middleware_can_be_disabled(): void
    {
        $app = AppFactory::create_request_id();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/request-id/disabled', ['headers' => ['X-Request-ID' => 'external-id']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['status' => 'no-request-id'], $response->body);
    }

    public function test_request_timeout_request_completes_before_timeout(): void
    {
        $app = AppFactory::create_request_timeout();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/timeouts/fast', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['duration' => 'fast', 'status' => 'ok'], $response->body);
    }

    public function test_request_timeout_request_exceeds_timeout(): void
    {
        $app = AppFactory::create_request_timeout();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/timeouts/slow', []);

        $this->assertSame(408, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_static_files_static_file_server_returns_text_file(): void
    {
        $app = AppFactory::create_static_files();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/public/hello.txt', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('Hello from static storage', $response->body);
    }

    public function test_static_files_static_server_returns_index_html_for_directory(): void
    {
        $app = AppFactory::create_static_files();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/app/', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('<!doctype html><h1>Welcome</h1>', $response->body);
    }

    public function test_status_codes_19_413_payload_too_large(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/upload', ['body' => ['data' => '{{ repeat \'x\' 2000 times }}']]);

        $this->assertSame(413, $response->statusCode);
        $this->assertEquals(['error' => 'Payload Too Large', 'message' => 'Request body size exceeds maximum allowed size of 1024 bytes'], $response->body);
    }

    public function test_status_codes_200_ok_success(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/status-test/200', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => 1, 'name' => 'Item 1'], $response->body);
    }

    public function test_status_codes_201_created_resource_created(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'New Item']]);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['id' => 1, 'name' => 'New Item'], $response->body);
    }

    public function test_status_codes_202_accepted_request_accepted_for_processing(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/tasks/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['task' => 'process_data']]);

        $this->assertSame(202, $response->statusCode);
        $this->assertEquals(['message' => 'Task accepted for processing', 'task_id' => 'abc123'], $response->body);
    }

    public function test_status_codes_204_no_content_success_with_no_body(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('DELETE', '/status-test/204', []);

        $this->assertSame(204, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_status_codes_206_partial_content(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/files/document.pdf', ['headers' => ['Range' => 'bytes=0-1023']]);

        $this->assertSame(206, $response->statusCode);
        $this->assertEquals('binary_data_1024_bytes', $response->body);
    }

    public function test_status_codes_20_414_uri_too_long(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data?skip_template_expansion=true', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals([], $response->body);
    }

    public function test_status_codes_21_431_request_header_fields_too_large(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data', ['headers' => ['X-Large-Header' => '{{ repeat \'x\' 10000 times }}']]);

        $this->assertSame(431, $response->statusCode);
        $this->assertEquals(['error' => 'Request Header Fields Too Large', 'message' => 'Request headers exceed maximum allowed size of 8192 bytes'], $response->body);
    }

    public function test_status_codes_22_501_not_implemented(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('TRACE', '/data', []);

        $this->assertSame(405, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_status_codes_23_503_service_unavailable(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/data', []);

        $this->assertSame(503, $response->statusCode);
        $this->assertEquals(['error' => 'Service Unavailable', 'message' => 'The service is temporarily unavailable. Please try again later.'], $response->body);
    }

    public function test_status_codes_301_moved_permanently_permanent_redirect(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/old-path', []);

        $this->assertSame(301, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_status_codes_302_found_temporary_redirect(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/temp-redirect', []);

        $this->assertSame(302, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_status_codes_304_not_modified_cached_content_valid(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/status-test/304', ['headers' => ['If-None-Match' => '"abc123"']]);

        $this->assertSame(304, $response->statusCode);
        $this->assertEquals(null, $response->body);
    }

    public function test_status_codes_307_temporary_redirect_method_preserved(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/redirect-post', ['headers' => ['Content-Type' => 'application/json'], 'body' => []]);

        $this->assertSame(307, $response->statusCode);
        $this->assertEquals([], $response->body);
    }

    public function test_status_codes_400_bad_request_invalid_request(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => 'not valid json']);

        $this->assertSame(400, $response->statusCode);
        $this->assertEquals(['detail' => 'Invalid request format'], $response->body);
    }

    public function test_status_codes_401_unauthorized_missing_authentication(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/me', []);

        $this->assertSame(401, $response->statusCode);
        $this->assertEquals(['detail' => 'Not authenticated'], $response->body);
    }

    public function test_status_codes_403_forbidden_insufficient_permissions(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/admin/users', ['headers' => ['Authorization' => 'Bearer valid_token']]);

        $this->assertSame(403, $response->statusCode);
        $this->assertEquals(['detail' => 'Not enough permissions'], $response->body);
    }

    public function test_status_codes_404_not_found_resource_not_found(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/status-test/404', []);

        $this->assertSame(404, $response->statusCode);
        $this->assertEquals(['detail' => 'Item not found'], $response->body);
    }

    public function test_status_codes_408_request_timeout(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/slow-endpoint', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['data' => 'large_data']]);

        $this->assertSame(408, $response->statusCode);
        $this->assertEquals(['detail' => 'Request timeout'], $response->body);
    }

    public function test_status_codes_422_unprocessable_entity_validation_error(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['price' => 'not a number']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['body', 'name'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_status_codes_429_too_many_requests(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/resource', []);

        $this->assertSame(429, $response->statusCode);
        $this->assertEquals(['detail' => 'Rate limit exceeded. Try again in 60 seconds.'], $response->body);
    }

    public function test_status_codes_500_internal_server_error_server_error(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/error', []);

        $this->assertSame(500, $response->statusCode);
        $this->assertEquals(['detail' => 'Internal server error', 'status' => 500, 'title' => 'Internal Server Error', 'type' => 'https://spikard.dev/errors/internal-server-error'], $response->body);
    }

    public function test_status_codes_503_service_unavailable_server_overload(): void
    {
        $app = AppFactory::create_status_codes();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/health', []);

        $this->assertSame(503, $response->statusCode);
        $this->assertEquals(['detail' => 'Service temporarily unavailable'], $response->body);
    }

    public function test_streaming_binary_log_download(): void
    {
        $app = AppFactory::create_streaming();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/stream/logfile', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('LOG:\\u0000\\u0001\\u0002\\u0003|TAIL|\\u0007\\n', $response->body);
    }

    public function test_streaming_chunked_csv_export(): void
    {
        $app = AppFactory::create_streaming();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/stream/csv-report', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('id,name,value\\n1,Alice,42\\n2,Bob,7\\n', $response->body);
    }

    public function test_streaming_stream_json_lines(): void
    {
        $app = AppFactory::create_streaming();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/stream/json-lines', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals('{"index":0,"payload":"alpha"}\\n{"index":1,"payload":"beta"}\\n{"index":2,"payload":"gamma"}\\n', $response->body);
    }

    public function test_url_encoded_13_array_field_success(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/register', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'tags[]=python&tags[]=rust&tags[]=typescript']);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['tags' => ['python', 'rust', 'typescript']], $response->body);
    }

    public function test_url_encoded_14_nested_object_bracket_notation(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/profile', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'user[name]=John%20Doe&user[email]=john@example.com&user[age]=30']);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['user' => ['age' => 30, 'email' => 'john@example.com', 'name' => 'John Doe']], $response->body);
    }

    public function test_url_encoded_15_special_characters_field_names(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/data', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'user-name=JohnDoe&contact.email=john%40example.com']);

        $this->assertSame(201, $response->statusCode);
        $this->assertEquals(['contact.email' => 'john@example.com', 'user-name' => 'JohnDoe'], $response->body);
    }

    public function test_url_encoded_16_minlength_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'username=ab']);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_length' => 2, 'min_length' => 3, 'value' => 'ab'], 'loc' => ['body', 'username'], 'msg' => 'String length must be at least 3', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_17_pattern_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/accounts', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'account_id=INVALID123']);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^ACC-[0-9]{6}$', 'value' => 'INVALID123'], 'loc' => ['body', 'account_id'], 'msg' => 'String does not match pattern \'^ACC-[0-9]{6}$\'', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_18_integer_minimum_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/products', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'quantity=0']);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_value' => 0, 'minimum' => 1], 'loc' => ['body', 'quantity'], 'msg' => 'Value must be at least 1', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_19_array_minitems_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/tags', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'tags[]=single']);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['actual_items' => 1, 'min_items' => 2], 'loc' => ['body', 'tags'], 'msg' => 'Array must contain at least 2 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_20_format_email_validation_failure(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/subscribe', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'email=not-an-email']);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['format' => 'email', 'value' => 'not-an-email'], 'loc' => ['body', 'email'], 'msg' => 'Invalid email format', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_21_integer_type_coercion_failure(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/products', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'price=not-a-number']);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['value' => 'not-a-number'], 'loc' => ['body', 'price'], 'msg' => 'Value is not a valid integer', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_22_additional_properties_strict_failure(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/settings', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded'], 'body' => 'theme=dark&unknown_field=value']);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['property' => 'unknown_field'], 'loc' => ['body', 'unknown_field'], 'msg' => 'Additional properties are not allowed', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_boolean_field_conversion(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['subscribe' => true, 'username' => 'johndoe'], $response->body);
    }

    public function test_url_encoded_empty_string_value(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['description' => '', 'username' => 'johndoe'], $response->body);
    }

    public function test_url_encoded_multiple_values_for_same_field(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/tags', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['tags' => ['python', 'fastapi', 'web']], $response->body);
    }

    public function test_url_encoded_numeric_field_type_conversion(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['age' => 30, 'username' => 'johndoe'], $response->body);
    }

    public function test_url_encoded_oauth2_password_grant_flow(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/token', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['access_token' => 'johndoe', 'token_type' => 'bearer'], $response->body);
    }

    public function test_url_encoded_optional_field_missing_success(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/register/', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['email' => null, 'username' => 'johndoe'], $response->body);
    }

    public function test_url_encoded_pattern_validation_fail(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/validated', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-z0-9_]+$'], 'input' => 'john doe', 'loc' => ['body', 'username'], 'msg' => 'String should match pattern \'^[a-z0-9_]+$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_required_field_missing_validation_error(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/login/', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => '', 'loc' => ['body', 'username'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_simple_form_submission_success(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/login/', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['username' => 'johndoe'], $response->body);
    }

    public function test_url_encoded_special_characters_encoding(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['description' => 'Test & Development', 'name' => 'John Doe'], $response->body);
    }

    public function test_url_encoded_string_max_length_validation_fail(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/validated', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 20], 'input' => 'this_is_a_very_long_username_that_exceeds_limit', 'loc' => ['body', 'username'], 'msg' => 'String should have at most 20 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_url_encoded_string_min_length_validation_fail(): void
    {
        $app = AppFactory::create_url_encoded();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/form/validated', ['headers' => ['Content-Type' => 'application/x-www-form-urlencoded']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['body', 'username'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_09_multiple_validation_errors(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/users', ['body' => ['age' => 15, 'email' => 'invalid-email', 'name' => 'ab']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '3 validation errors in request', 'errors' => [['ctx' => ['ge' => 18], 'input' => 15, 'loc' => ['body', 'age'], 'msg' => 'Input should be greater than or equal to 18', 'type' => 'greater_than_equal'], ['ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'], 'input' => 'invalid-email', 'loc' => ['body', 'email'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'', 'type' => 'string_pattern_mismatch'], ['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_10_nested_error_path(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/profiles', ['body' => ['profile' => ['contact' => ['email' => 'invalid']]]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'], 'input' => 'invalid', 'loc' => ['body', 'profile', 'contact', 'email'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_array_item_validation_error(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item', 'price' => 10.0, 'tags' => ['tag1', 'tag2', 123]]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 123, 'loc' => ['body', 'tags', '2'], 'msg' => 'Input should be a valid unknown', 'type' => 'type_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_array_max_items_constraint_violation(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item', 'price' => 10.0, 'tags' => ['tag1', 'tag2', 'tag3', 'tag4', 'tag5', 'tag6', 'tag7', 'tag8', 'tag9', 'tag10', 'tag11']]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => ['tag1', 'tag2', 'tag3', 'tag4', 'tag5', 'tag6', 'tag7', 'tag8', 'tag9', 'tag10', 'tag11'], 'loc' => ['body', 'tags'], 'msg' => '["tag1","tag2","tag3","tag4","tag5","tag6","tag7","tag8","tag9","tag10","tag11"] has more than 10 items', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_array_min_items_constraint_violation(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item', 'price' => 10.0, 'tags' => []]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => [], 'loc' => ['body', 'tags'], 'msg' => '[] has less than 1 item', 'type' => 'validation_error']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_body_field_type_error_string_for_float(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item', 'price' => 'not_a_float']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'not_a_float', 'loc' => ['body', 'price'], 'msg' => 'Input should be a valid number, unable to parse string as a number', 'type' => 'float_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_header_validation_error(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=test', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['headers', 'x-token'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_invalid_uuid_format(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/not-a-uuid', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-uuid', 'loc' => ['path', 'item_id'], 'msg' => 'Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0', 'type' => 'uuid_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_invalid_boolean_value(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=test&is_active=maybe', ['headers' => ['x-token' => 'test-token']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'maybe', 'loc' => ['query', 'is_active'], 'msg' => 'Input should be a valid boolean, unable to interpret input', 'type' => 'bool_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_invalid_datetime_format(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['created_at' => 'not-a-datetime', 'name' => 'Item', 'price' => 10.0]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'not-a-datetime', 'loc' => ['body', 'created_at'], 'msg' => 'Input should be a valid datetime', 'type' => 'datetime_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_invalid_enum_value(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/models/invalid_model', []);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['expected' => '\'alexnet\', \'resnet\' or \'lenet\''], 'input' => 'invalid_model', 'loc' => ['path', 'model_name'], 'msg' => 'Input should be \'alexnet\', \'resnet\' or \'lenet\'', 'type' => 'enum']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_malformed_json_body(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => '{"name": "Item", "price": }']);

        $this->assertSame(400, $response->statusCode);
        $this->assertEquals(['detail' => 'Invalid request format'], $response->body);
    }

    public function test_validation_errors_missing_required_body_field(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Item']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => ['name' => 'Item'], 'loc' => ['body', 'price'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_missing_required_query_parameter(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/', ['headers' => ['x-token' => 'test-token']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => null, 'loc' => ['query', 'q'], 'msg' => 'Field required', 'type' => 'missing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_multiple_validation_errors(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'X', 'price' => -10, 'quantity' => 'not_a_number']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '3 validation errors in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'X', 'loc' => ['body', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short'], ['ctx' => ['gt' => 0], 'input' => -10, 'loc' => ['body', 'price'], 'msg' => 'Input should be greater than 0', 'type' => 'greater_than'], ['input' => 'not_a_number', 'loc' => ['body', 'quantity'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_nested_object_validation_error(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('POST', '/items/', ['headers' => ['Content-Type' => 'application/json'], 'body' => ['name' => 'Product', 'price' => 10.0, 'seller' => ['address' => ['city' => 'SF', 'zip_code' => '123'], 'name' => 'Jo']]]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '3 validation errors in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'SF', 'loc' => ['body', 'seller', 'address', 'city'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short'], ['ctx' => ['min_length' => 5], 'input' => '123', 'loc' => ['body', 'seller', 'address', 'zip_code'], 'msg' => 'String should have at least 5 characters', 'type' => 'string_too_short'], ['ctx' => ['min_length' => 3], 'input' => 'Jo', 'loc' => ['body', 'seller', 'name'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_numeric_constraint_violation_gt_greater_than(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=test&price=0', ['headers' => ['x-token' => 'test-token']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['gt' => 0], 'input' => '0', 'loc' => ['query', 'price'], 'msg' => 'Input should be greater than 0', 'type' => 'greater_than']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_numeric_constraint_violation_le_less_than_or_equal(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=test&limit=101', ['headers' => ['x-token' => 'test-token']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['le' => 100], 'input' => '101', 'loc' => ['query', 'limit'], 'msg' => 'Input should be less than or equal to 100', 'type' => 'less_than_equal']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_query_param_type_error_string_provided_for_int(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=test&skip=not_a_number', ['headers' => ['x-token' => 'test-token']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['input' => 'not_a_number', 'loc' => ['query', 'skip'], 'msg' => 'Input should be a valid integer, unable to parse string as an integer', 'type' => 'int_parsing']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_string_max_length_constraint_violation(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', ['headers' => ['x-token' => 'test-token']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['max_length' => 50], 'input' => 'this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter', 'loc' => ['query', 'q'], 'msg' => 'String should have at most 50 characters', 'type' => 'string_too_long']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_string_min_length_constraint_violation(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=ab', ['headers' => ['x-token' => 'test-token']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['min_length' => 3], 'input' => 'ab', 'loc' => ['query', 'q'], 'msg' => 'String should have at least 3 characters', 'type' => 'string_too_short']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

    public function test_validation_errors_string_regex_pattern_mismatch(): void
    {
        $app = AppFactory::create_validation_errors();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/items/?q=invalid!', ['headers' => ['x-token' => 'test-token']]);

        $this->assertSame(422, $response->statusCode);
        $this->assertEquals(['detail' => '1 validation error in request', 'errors' => [['ctx' => ['pattern' => '^[a-zA-Z0-9_-]+$'], 'input' => 'invalid!', 'loc' => ['query', 'q'], 'msg' => 'String should match pattern \'^[a-zA-Z0-9_-]+$\'', 'type' => 'string_pattern_mismatch']], 'status' => 422, 'title' => 'Request Validation Failed', 'type' => 'https://spikard.dev/errors/validation-error'], $response->body);
    }

}
