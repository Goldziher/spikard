```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Testing\TestClient;

final class UserWorkflowTest extends TestCase
{
    public function testCompletesUserWorkflow(): void
    {
        $usersDb = [];
        $app = new App();

        $app->post('/users', function ($request) use (&$usersDb) {
            $id = count($usersDb) + 1;
            $user = ['id' => $id, 'name' => $request->body['name']];
            $usersDb[$id] = $user;
            return $user;
        });

        $app->get('/users/{id}', function ($request) use (&$usersDb) {
            $id = (int) $request->params['id'];
            return $usersDb[$id] ?? ['error' => 'Not found'];
        });

        $client = new TestClient($app);

        // Create user
        $createRes = $client->post('/users', ['name' => 'Alice']);
        $user = json_decode($createRes->getBody(), true);
        $this->assertEquals('Alice', $user['name']);

        // Retrieve user
        $getRes = $client->get('/users/' . $user['id']);
        $this->assertEquals($user, json_decode($getRes->getBody(), true));
    }
}
```
