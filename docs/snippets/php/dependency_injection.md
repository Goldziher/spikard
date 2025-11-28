```php
<?php

use Spikard\App;
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

$app = $app->addRoute('GET', '/stats', function () {
    // Note: Full auto-injection pending P1.4
    return Response::json(['status' => 'ok']);
});
```
