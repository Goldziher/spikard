```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\HookResult;
use Spikard\Http\Request;
use Spikard\Http\Response;

$hooks = new LifecycleHooks(
    preHandler: function (Request $request): HookResult {
        $token = $request->headers['authorization'] ?? null;
        if ($token !== 'Bearer dev-token') {
            return HookResult::ShortCircuit(
                Response::json(['error' => 'unauthorized'], 401)
            );
        }
        return HookResult::Continue($request);
    }
);

$app = (new App(new ServerConfig(port: 8000)))
    ->withLifecycleHooks($hooks);
```
