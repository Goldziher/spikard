```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;

final class UserValidationTest extends TestCase
{
    private TestClient $client;

    protected function setUp(): void
    {
        $this->client = new TestClient($this->createApp());
    }

    public function testAcceptsValidRequests(): void
    {
        $response = $this->client->post('/users', [
            'email' => 'test@example.com',
            'age' => 25,
            'username' => 'testuser',
        ]);

        $this->assertEquals(201, $response->getStatusCode());
    }

    public function testRejectsInvalidEmail(): void
    {
        $response = $this->client->post('/users', [
            'email' => 'not-an-email',
            'age' => 25,
            'username' => 'testuser',
        ]);

        $this->assertEquals(422, $response->getStatusCode());

        $body = json_decode($response->getBody(), true);
        $this->assertArrayHasKey('details', $body);
        $this->assertStringContainsString('email', $body['details'][0]['field']);
    }

    public function testRejectsAgeBelowMinimum(): void
    {
        $response = $this->client->post('/users', [
            'email' => 'test@example.com',
            'age' => 16,
            'username' => 'testuser',
        ]);

        $this->assertEquals(422, $response->getStatusCode());
    }

    public function testRejectsMissingRequiredFields(): void
    {
        $response = $this->client->post('/users', [
            'email' => 'test@example.com',
            'age' => 25,
            // missing username
        ]);

        $this->assertEquals(422, $response->getStatusCode());
    }
}
```
