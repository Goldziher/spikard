```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Testing\TestClient;

final class SseTest extends TestCase
{
    public function testStreamsSseEvents(): void
    {
        $app = new App();
        $app->sse('/events', function () {
            for ($i = 0; $i < 3; $i++) {
                yield ['event' => 'message', 'data' => ['count' => $i]];
            }
        });

        $client = new TestClient($app);
        $response = $client->get('/events');

        // SSE responses return status 200
        $this->assertEquals(200, $response->getStatusCode());
    }
}
```
