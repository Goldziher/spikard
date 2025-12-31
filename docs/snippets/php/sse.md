```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\StreamingResponse;

final class EventsController
{
    #[Get('/events')]
    public function events(): StreamingResponse
    {
        $generator = function (): Generator {
            for ($i = 0; $i < 5; $i++) {
                $data = json_encode(['tick' => $i, 'time' => time()]);
                yield "data: {$data}\n\n";
                sleep(1);
            }
            yield "data: " . json_encode(['message' => 'done']) . "\n\n";
        };

        return StreamingResponse::sse($generator());
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new EventsController());
```
