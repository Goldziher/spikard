```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Handlers\WebSocketHandlerInterface;

final class ChatHandler implements WebSocketHandlerInterface
{
    public function onConnect(): void
    {
        error_log('Client connected');
    }

    public function onMessage(string $message): void
    {
        $data = json_decode($message, true);
        error_log('Received: ' . json_encode($data));
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        error_log("Client disconnected: {$code}" . ($reason ? " ({$reason})" : ''));
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->addWebSocket('/ws', new ChatHandler());

$app->run();
```
