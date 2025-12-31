```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\Response;

final class HealthController
{
    #[Get('/health')]
    public function health(): Response
    {
        return Response::json(['status' => 'ok']);
    }
}

$config = ServerConfig::builder()
    ->withHost('0.0.0.0')
    ->withPort(8000)
    ->withWorkers(4)
    ->build();

$app = (new App($config))
    ->registerController(new HealthController());

$app->run();
```
