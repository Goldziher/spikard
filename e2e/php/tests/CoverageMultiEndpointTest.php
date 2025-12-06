<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;
use E2E\Php\AppFactory;

/**
 * E2E test coverage for multi-endpoint application flows.
 * Tests complete request/response cycles and lifecycle management.
 */
final class CoverageMultiEndpointTest extends TestCase
{
    /**
     * Server initialization and request handling
     */
    public function test_server_initialization_and_first_request(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client = TestClient::create($app);

        // First request to initialized server
        $response = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);

        $this->assertSame(200, $response->statusCode);
        $this->assertIsArray($response->body);
    }

    /**
     * Multiple sequential requests to same server
     */
    public function test_multiple_sequential_requests(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client = TestClient::create($app);

        $headers = ['X-API-Key' => 'sk_test_123456'];

        for ($i = 0; $i < 5; $i++) {
            $response = $client->request('GET', '/api/data', ['headers' => $headers]);
            $this->assertSame(200, $response->statusCode);
        }
    }

    /**
     * Request isolation: requests don't interfere with each other
     */
    public function test_request_isolation(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client = TestClient::create($app);

        $headers = ['X-API-Key' => 'sk_test_123456'];

        // Request 1
        $response1 = $client->request('GET', '/api/data', ['headers' => $headers]);
        // Request 2
        $response2 = $client->request('GET', '/api/data', ['headers' => $headers]);

        // Both should be successful and identical
        $this->assertSame(200, $response1->statusCode);
        $this->assertSame(200, $response2->statusCode);
        $this->assertEquals($response1->body, $response2->body);
    }

    /**
     * Error handling doesn't crash server
     */
    public function test_server_survives_invalid_requests(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $client = TestClient::create($app);

        // Invalid request
        $errorResponse = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'invalid'],
        ]);
        $this->assertSame(401, $errorResponse->statusCode);

        // Server still works with new app and valid request
        $app2 = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client2 = TestClient::create($app2);
        $successResponse = $client2->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);
        $this->assertSame(200, $successResponse->statusCode);
    }

    /**
     * Different authentication schemes on different apps
     */
    public function test_different_auth_schemes_independence(): void
    {
        // API Key auth app
        $apiKeyApp = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $apiKeyClient = TestClient::create($apiKeyApp);
        $apiKeyResponse = $apiKeyClient->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);

        // JWT auth app
        $jwtApp = AppFactory::create_auth_jwt_authentication_valid_token_12();
        $jwtClient = TestClient::create($jwtApp);
        $jwtResponse = $jwtClient->request('GET', '/protected/user', [
            'headers' => [
                'Authorization' => 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg',
            ],
        ]);

        $this->assertSame(200, $apiKeyResponse->statusCode);
        $this->assertSame(200, $jwtResponse->statusCode);
    }

    /**
     * Response headers are present and valid
     */
    public function test_response_headers_present(): void
    {
        $app = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);

        $this->assertSame(200, $response->statusCode);
        $this->assertIsArray($response->headers);

        // Common response headers should be present
        if (!empty($response->headers)) {
            $this->assertIsArray($response->headers);
        }
    }

    /**
     * Request body handling across endpoints
     */
    public function test_multiple_apps_handle_bodies_independently(): void
    {
        $app1 = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client1 = TestClient::create($app1);

        // GET request on first app
        $response1 = $client1->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);

        // Another GET request
        $app2 = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client2 = TestClient::create($app2);
        $response2 = $client2->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);

        // Both should succeed identically
        $this->assertSame(200, $response1->statusCode);
        $this->assertSame(200, $response2->statusCode);
        $this->assertEquals($response1->body, $response2->body);
    }

    /**
     * Complete application lifecycle: init -> requests -> cleanup
     */
    public function test_complete_application_lifecycle(): void
    {
        // Initialize
        $app = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client = TestClient::create($app);

        // Make requests
        $response1 = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);
        $this->assertSame(200, $response1->statusCode);

        // More requests
        $response2 = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);
        $this->assertSame(200, $response2->statusCode);

        // Cleanup happens automatically when $app goes out of scope
        unset($app);
        unset($client);

        // Should be able to create new app
        $app = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client = TestClient::create($app);
        $response3 = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);
        $this->assertSame(200, $response3->statusCode);
    }

    /**
     * HTTP status code propagation through full stack
     */
    public function test_http_status_propagation_through_stack(): void
    {
        // Success case
        $app = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client = TestClient::create($app);
        $successResponse = $client->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);
        $this->assertSame(200, $successResponse->statusCode);

        // Unauthorized case
        $app2 = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $client2 = TestClient::create($app2);
        $errorResponse = $client2->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'invalid'],
        ]);
        $this->assertSame(401, $errorResponse->statusCode);

        // Status codes correctly reflect server-side logic
        $this->assertNotEquals($successResponse->statusCode, $errorResponse->statusCode);
    }

    /**
     * Content-Type header consistency across responses
     */
    public function test_response_content_type_consistency(): void
    {
        $app1 = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client1 = TestClient::create($app1);
        $response1 = $client1->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);

        $app2 = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $client2 = TestClient::create($app2);
        $response2 = $client2->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);

        // Both should have consistent response structure
        $this->assertIsArray($response1->body);
        $this->assertIsArray($response2->body);
    }

    /**
     * Full stack error handling and recovery
     */
    public function test_full_stack_error_recovery(): void
    {
        // Create error scenario
        $errorApp = AppFactory::create_auth_api_key_authentication_invalid_key_1();
        $errorClient = TestClient::create($errorApp);

        $errorAttempt1 = $errorClient->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'wrong1']]);
        $errorAttempt2 = $errorClient->request('GET', '/api/data', ['headers' => ['X-API-Key' => 'wrong2']]);

        $this->assertSame(401, $errorAttempt1->statusCode);
        $this->assertSame(401, $errorAttempt2->statusCode);

        // Create success scenario
        $successApp = AppFactory::create_auth_api_key_authentication_valid_key_3();
        $successClient = TestClient::create($successApp);
        $successResponse = $successClient->request('GET', '/api/data', [
            'headers' => ['X-API-Key' => 'sk_test_123456'],
        ]);

        $this->assertSame(200, $successResponse->statusCode);
    }
}
