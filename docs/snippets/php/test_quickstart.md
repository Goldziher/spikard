```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Testing\TestClient;

final class HelloTest extends TestCase
{
    public function testHelloEndpoint(): void
    {
        $app = new App();
        $app->get('/hello', fn() => ['message' => 'Hello, World!']);

        $client = new TestClient($app);
        $response = $client->get('/hello');

        $this->assertEquals(200, $response->getStatusCode());
        $this->assertEquals(
            ['message' => 'Hello, World!'],
            json_decode($response->getBody(), true)
        );
    }
}
```
