```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\StreamingResponse;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('GET', '/stream', function () {
    $generator = function (): Generator {
        for ($i = 0; $i < 3; $i++) {
            yield "data: " . json_encode(['tick' => $i]) . "\n\n";
            sleep(1);
        }
    };

    return StreamingResponse::sse($generator());
});
```
