```php
<?php

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\DI\DependencyContainer;
use Spikard\DI\Provide;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

// Value dependency (singleton)
$container = new DependencyContainer(
    values: ['config' => ['db_url' => 'postgresql://localhost/app']],
    factories: [
        'db_pool' => new Provide(
            factory: fn (array $config) => ['url' => $config['db_url'], 'client' => 'pool'],
            dependsOn: ['config'],
            singleton: true
        ),
    ]
);

$app = $app->withDependencies($container);

final class StatsController
{
    #[Get('/stats')]
    public function stats(): Response
    {
        // Note: Full auto-injection pending P1.4
        return Response::json(['status' => 'ok']);
    }
}

$app = $app->registerController(new StatsController());
```
