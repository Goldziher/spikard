```php
<?php

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\StreamingResponse;

$app = new App(new ServerConfig(port: 8000));

final class EventsController
{
    #[Get('/events')]
    public function events(): StreamingResponse
    {
        $generator = function (): Generator {
            for ($i = 0; $i < 3; $i++) {
                yield "data: " . json_encode(['tick' => $i]) . "\n\n";
            }
        };

        return StreamingResponse::sse($generator());
    }
}

$app = $app->registerController(new EventsController());
```
