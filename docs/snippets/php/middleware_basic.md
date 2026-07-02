```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\HookResult;
use Spikard\Http\Request;

$hooks = LifecycleHooks::builder()
    ->withOnRequest(function (Request $request): HookResult {
        error_log(sprintf(
            "[%s] %s %s",
            date('Y-m-d H:i:s'),
            $request->method,
            $request->path
        ));
        return HookResult::continue();
    })
    ->build();

$app = (new App(new ServerConfig(port: 8000)))
    ->withLifecycleHooks($hooks);
```
