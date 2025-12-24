```php
<?php

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Background\BackgroundTask;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

function sendEmail(int $userId): void {
    error_log("send email to {$userId}");
}

final class SignupController
{
    #[Post('/signup')]
    public function signup(Request $request): Response
    {
        $user = $request->body;

        BackgroundTask::run(function () use ($user) {
            sendEmail($user['id']);
        });

        return Response::json($user);
    }
}

$app = $app->registerController(new SignupController());
```
