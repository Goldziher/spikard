```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\HookResult;
use Spikard\Http\Request;

$hooks = new LifecycleHooks(
    onRequest: function (Request $request): HookResult {
        error_log("{$request->method} {$request->path}");
        return HookResult::Continue($request);
    }
);

$app = (new App(new ServerConfig(port: 8000)))
    ->withLifecycleHooks($hooks);
```
