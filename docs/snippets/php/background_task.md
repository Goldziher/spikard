```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Background\BackgroundTask;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

function sendWelcomeEmail(int $userId, string $email): void
{
    error_log("Sending welcome email to {$email} (user: {$userId})");
    // Simulate email sending
    sleep(2);
    error_log("Email sent to {$email}");
}

final class SignupController
{
    #[Post('/signup')]
    public function signup(Request $request): Response
    {
        $user = $request->body;
        $userId = random_int(1000, 9999);
        $email = $user['email'] ?? 'unknown@example.com';

        // Fire-and-forget background task
        BackgroundTask::run(function () use ($userId, $email): void {
            sendWelcomeEmail($userId, $email);
        });

        return Response::json([
            'id' => $userId,
            'email' => $email,
            'status' => 'created'
        ], 201);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new SignupController());
```
