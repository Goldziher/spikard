```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Testing\TestClient;

final class UserCreationTest extends TestCase
{
    public function testCreatesUser(): void
    {
        $app = new App();
        $app->post('/users', function ($request) {
            return [
                'id' => 1,
                'name' => $request->body['name'],
                'email' => $request->body['email'],
            ];
        });

        $client = new TestClient($app);
        $response = $client->post('/users', [
            'name' => 'Alice',
            'email' => 'alice@example.com',
        ]);

        $this->assertEquals(200, $response->getStatusCode());
        $data = json_decode($response->getBody(), true);
        $this->assertEquals('Alice', $data['name']);
        $this->assertEquals('alice@example.com', $data['email']);
    }
}
```
