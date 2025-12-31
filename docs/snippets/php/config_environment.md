```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Config\ServerConfig;

$config = ServerConfig::builder()
    ->withHost(getenv('SPIKARD_HOST') ?: '127.0.0.1')
    ->withPort((int) (getenv('SPIKARD_PORT') ?: 8000))
    ->withWorkers((int) (getenv('SPIKARD_WORKERS') ?: 1))
    ->withRequestTimeout((int) (getenv('SPIKARD_TIMEOUT') ?: 30))
    ->build();

$app = new App($config);

// Keep secrets in env
$apiKey = getenv('API_KEY');
$dbUrl = getenv('DATABASE_URL');
```
