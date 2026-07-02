```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\StreamingResponse;

$app = new App(new ServerConfig(port: 8000));

final class StreamController
{
    #[Get('/stream')]
    public function stream(): StreamingResponse
    {
        $generator = function (): Generator {
            for ($i = 0; $i < 10; $i++) {
                yield json_encode(['chunk' => $i]) . "\n";
                usleep(100000); // 100ms delay
            }
        };

        return new StreamingResponse(
            $generator(),
            headers: ['Content-Type' => 'application/x-ndjson']
        );
    }
}

$app = $app->registerController(new StreamController());
```
