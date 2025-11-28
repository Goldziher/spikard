```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Handlers\WebSocketHandlerInterface;

class ChatHandler implements WebSocketHandlerInterface
{
    public function onConnect(): void
    {
        error_log('Client connected');
    }

    public function onMessage(string $message): void
    {
        // Handle incoming message
        $data = json_decode($message, true);
        error_log("Received: " . json_encode($data));
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        error_log("Client disconnected: {$code}");
    }
}

$app = new App(new ServerConfig(port: 8000));
$app = $app->addWebSocket('/ws', new ChatHandler());
```
