```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Testing\TestClient;

final class ValidationTest extends TestCase
{
    public function testRejectsInvalidInput(): void
    {
        $app = new App();
        $app->post('/users', function ($request) {
            if (!isset($request->body['name']) || !is_int($request->body['age'])) {
                return ['status' => 400, 'body' => ['error' => 'Validation failed']];
            }
            return ['name' => $request->body['name'], 'age' => $request->body['age']];
        });

        $client = new TestClient($app);
        $response = $client->post('/users', [
            'name' => 'Bob',
            'age' => 'invalid',
        ]);

        $this->assertEquals(400, $response->getStatusCode());
        $body = json_decode($response->getBody(), true);
        $this->assertStringContainsString('Validation', $body['error']);
    }
}
```
