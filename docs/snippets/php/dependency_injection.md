```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\DI\DependencyContainer;
use Spikard\DI\Provide;
use Spikard\Http\Response;

// Create dependency container with values and factories
$container = DependencyContainer::builder()
    ->withValue('config', ['db_url' => 'postgresql://localhost/app'])
    ->withFactory(
        'db_pool',
        new Provide(
            factory: fn (array $config) => [
                'url' => $config['db_url'],
                'client' => 'pool'
            ],
            dependsOn: ['config'],
            singleton: true
        )
    )
    ->build();

final class StatsController
{
    #[Get('/stats')]
    public function stats(): Response
    {
        return Response::json(['status' => 'ok', 'db' => 'connected']);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->withDependencies($container)
    ->registerController(new StatsController());
```
