<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;
use E2E\Php\AppFactory;

/**
 * E2E test coverage for error handling and HTTP error responses across the full stack.
 */
final class CoverageErrorHandlingTest extends TestCase
{
    /**
     * 400/401 Bad Request or Unauthorized: invalid authentication scenario
     */
    public function test_missing_required_auth_returns_error(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_missing_header_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', []);

        // Missing required auth should return error status
        $this->assertTrue($response->statusCode >= 400 && $response->statusCode < 500);
        $this->assertSame(401, $response->statusCode);
    }

    /**
     * 401 Unauthorized: invalid credentials
     */
    public function test_401_unauthorized_invalid_credentials(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'invalid_key_12345'],
        ]);

        $this->assertSame(401, $response->statusCode);
        $this->assertIsArray($response->body);
        $this->assertTrue(
            isset($response->body['error']) || isset($response->body['detail']) || isset($response->body['title'])
        );
    }

    /**
     * 401 Unauthorized: expired JWT token
     */
    public function test_401_unauthorized_expired_jwt(): void
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
     * Error response structure validation: consistent fields
     */
    public function test_error_response_structure_consistency(): void
    {
        $app1 = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $app2 = AppFactory::create_auth_jwt_authentication_expired_token_8();

        $client1 = TestClient::create($app1);
        $client2 = TestClient::create($app2);

        $response1 = $client1->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'invalid'],
        ]);
        $response2 = $client2->request('GET', '/protected/user', [
            'headers' => [
                'Authorization' => 'Bearer expired_token',
            ],
        ]);

        // Both should be 401 with error structure
        $this->assertSame(401, $response1->statusCode);
        $this->assertSame(401, $response2->statusCode);

        // Both should have similar error structure
        $this->assertTrue(is_array($response1->body));
        $this->assertTrue(is_array($response2->body));
    }

    /**
     * E2E error propagation: validation error flows through stack
     */
    public function test_error_propagation_through_stack(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $client = TestClient::create($app);

        // Make invalid request
        $response = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'invalid_key'],
        ]);

        // Should have error status
        $this->assertSame(401, $response->statusCode);

        // Error should have details
        $this->assertIsArray($response->body);

        // Error body should have meaningful content
        if (isset($response->body['detail'])) {
            $this->assertNotEmpty($response->body['detail']);
        }
    }

    /**
     * Complete E2E: Recovery from error to success
     */
    public function test_end_to_end_error_recovery_flow(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $client = TestClient::create($app);

        // First: error case
        $errorResponse = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'invalid'],
        ]);
        $this->assertSame(401, $errorResponse->statusCode);

        // Second: same endpoint with valid auth
        $app2 = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client2 = TestClient::create($app2);
        $successResponse = $client2->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);
        $this->assertSame(200, $successResponse->statusCode);

        // Status codes should differ
        $this->assertNotEquals($errorResponse->statusCode, $successResponse->statusCode);
    }
}
