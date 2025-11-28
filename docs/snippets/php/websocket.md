```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\WebSocket\WebSocket;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addWebSocketRoute('/ws', function (WebSocket $socket) {
    foreach ($socket->messages() as $message) {
        $socket->sendJson(['echo' => $message]);
    }
});
```
