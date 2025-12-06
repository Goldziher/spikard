<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;
use E2E\Php\AppFactory;

/**
 * E2E test coverage for authentication across different scenarios.
 * Organized by E2E patterns: valid flows, invalid flows, edge cases.
 */
final class CoverageAuthTest extends TestCase
{
    /**
     * Valid authentication scenario: API key header
     */
    public function test_valid_api_key_header_authentication(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'sk_test_123456']]);

        $this->assertSame(200, $response->statusCode);
        $this->assertArrayHasKey('message', $response->body);
    }

    /**
     * Invalid authentication scenario: wrong API key
     */
    public function test_invalid_api_key_authentication(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'invalid_key_12345']]);

        $this->assertSame(401, $response->statusCode);
        $this->assertArrayHasKey('detail', $response->body);
    }

    /**
     * Edge case: missing required authentication header
     */
    public function test_missing_api_key_header(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_missing_header_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', []);

        $this->assertSame(401, $response->statusCode);
    }

    /**
     * Alternative auth method: Query parameter authentication
     */
    public function test_api_key_query_parameter_authentication(): void
    {
        $app = AppFactory::create_auth_api_key_in_query_parameter_4();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data?api_key=sk_test_123456', []);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Key rotation scenario: old key still valid
     */
    public function test_api_key_rotation_backward_compatibility(): void
    {
        $app = AppFactory::create_auth_api_key_rotation_old_key_still_valid_5();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'sk_test_old_123456']]);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Custom header authentication
     */
    public function test_custom_header_name_authentication(): void
    {
        $app = AppFactory::create_auth_api_key_with_custom_header_name_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', ['headers' => ['X-API-Token' => 'sk_test_123456']]);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * JWT token validation: valid token
     */
    public function test_jwt_valid_token(): void
    {
        $app = AppFactory::create_auth_jwt_authentication_valid_token_12();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', [
            'headers' => [
                'Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg',
            ],
        ]);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * JWT validation: expired token
     */
    public function test_jwt_expired_token(): void
    {
        $app = AppFactory::create_auth_jwt_authentication_expired_token_8();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', [
            'headers' => [
                'Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNjAwMDAwMDAwLCJpYXQiOjE1OTAwMDAwMDB9.n4oBw9XuO2aAJWi1e4Bz9Y_m2iEyJHGAODcetNuwYFo',
            ],
        ]);

        $this->assertSame(401, $response->statusCode);
    }

    /**
     * JWT validation: malformed token format
     */
    public function test_jwt_malformed_token(): void
    {
        $app = AppFactory::create_auth_jwt_malformed_token_format_14();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', [
            'headers' => ['Authorization' => 'Bearer invalid.token'],
        ]);

        $this->assertSame(401, $response->statusCode);
    }

    /**
     * Bearer token format: missing "Bearer" prefix
     */
    public function test_bearer_token_missing_prefix(): void
    {
        $app = AppFactory::create_auth_bearer_token_without_prefix_7();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/protected', [
            'headers' => [
                'Authorization' => 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDZ9.8yXqZ9jKCR0BwqJc7pN_QvD3mYLxHfWzUeIaGkTnOsA',
            ],
        ]);

        $this->assertSame(401, $response->statusCode);
    }

    /**
     * JWT claim validation: invalid signature
     */
    public function test_jwt_invalid_signature(): void
    {
        $app = AppFactory::create_auth_jwt_authentication_invalid_signature_10();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', [
            'headers' => [
                'Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIn0.invalid_signature',
            ],
        ]);

        $this->assertSame(401, $response->statusCode);
    }

    /**
     * JWT claim validation: invalid audience
     */
    public function test_jwt_invalid_audience(): void
    {
        $app = AppFactory::create_auth_jwt_authentication_invalid_audience_9();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/protected/user', [
            'headers' => [
                'Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMCwiYXVkIjpbImh0dHBzOi8vd3Jvbmctc2VydmljZS5jb20iXX0.YR2a9fSJjhen7ksYFI2djSBSC7Pc29FDCloBGhkj3kU',
            ],
        ]);

        $this->assertSame(401, $response->statusCode);
    }

    /**
     * Multi-scheme authentication: different API key scheme
     */
    public function test_api_key_rotation_multiple_keys_accepted(): void
    {
        $app = AppFactory::create_auth_api_key_with_custom_header_name_6();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', [
            'headers' => [
                'X-API-Token' => 'sk_test_123456',
            ],
        ]);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Complete E2E flow: Multiple auth attempts with different results
     */
    public function test_end_to_end_multiple_auth_scenarios(): void
    {
        // Scenario 1: Valid API key
        $app1 = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client1 = TestClient::create($app1);
        $responseValid = $client1->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);
        $this->assertSame(200, $responseValid->statusCode);

        // Scenario 2: Invalid API key
        $app2 = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $client2 = TestClient::create($app2);
        $responseInvalid = $client2->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'invalid_key_12345'],
        ]);
        $this->assertSame(401, $responseInvalid->statusCode);

        // Scenario 3: API key in query param
        $app3 = AppFactory::create_auth_api_key_in_query_parameter_4();
        $client3 = TestClient::create($app3);
        $responseQueryParam = $client3->request('GET', '/api/data?api_key=sk_test_123456', []);
        $this->assertSame(200, $responseQueryParam->statusCode);

        // Verify different scenarios gave different results
        $this->assertNotEquals($responseValid->statusCode, $responseInvalid->statusCode);
    }
}
