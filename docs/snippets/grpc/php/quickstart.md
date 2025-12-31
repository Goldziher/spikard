```php
<?php

declare(strict_types=1);

// UserServiceHandler.php
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;
use Userservice\V1\GetUserRequest;
use Userservice\V1\User;

class UserServiceHandler
{
    public function handleRequest(Request $request): Response
    {
        if ($request->methodName === 'GetUser') {
            // Deserialize
            $req = new GetUserRequest();
            $req->mergeFromString($request->payload);

            // Process
            $user = new User();
            $user->setId($req->getUserId());
            $user->setName('Alice');
            $user->setEmail('alice@example.com');

            // Serialize and return
            return new Response(payload: $user->serializeToString());
        }
    }
}
```
