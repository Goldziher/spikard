```php
<?php

use Spikard\App;
use Spikard\Background\BackgroundTask;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

function sendEmail(int $userId): void {
    error_log("send email to {$userId}");
}

$app = $app->addRoute('POST', '/signup', function (Request $request) {
    $user = $request->jsonBody();

    BackgroundTask::run(function () use ($user) {
        sendEmail($user['id']);
    });

    return Response::json($user);
});
```
