```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Testing\TestClient;

final class WebSocketTest extends TestCase
{
    public function testEchoesWebSocketMessages(): void
    {
        $app = new App();
        $app->websocket('/echo', fn(string $message) => $message);

        $client = new TestClient($app);
        $ws = $client->websocketConnect('/echo');

        $ws->sendJson(['hello' => 'world']);
        $response = $ws->receiveJson();

        $this->assertEquals(['hello' => 'world'], $response);
    }
}
```
