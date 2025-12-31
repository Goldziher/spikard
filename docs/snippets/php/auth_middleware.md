```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\HookResult;
use Spikard\Http\Request;
use Spikard\Http\Response;

$hooks = LifecycleHooks::builder()
    ->withPreHandler(function (Request $request): HookResult {
        $token = $request->headers['authorization'] ?? null;
        if ($token !== 'Bearer dev-token') {
            return HookResult::shortCircuit(
                Response::json(['error' => 'unauthorized'], 401)
            );
        }
        return HookResult::continue();
    })
    ->build();

$app = (new App(new ServerConfig(port: 8000)))
    ->withLifecycleHooks($hooks);
```
