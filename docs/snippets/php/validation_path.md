```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Validation\Validator;

final class PostsController
{
    #[Get('/users/{user_id}/posts/{post_id}')]
    public function show(Request $request, string $user_id, int $post_id): Response
    {
        $rules = [
            'user_id' => ['required', 'string', 'uuid'],
            'post_id' => ['required', 'integer', 'min:1'],
        ];

        $params = [
            'user_id' => $user_id,
            'post_id' => $post_id,
        ];

        $validator = new Validator($params, $rules);

        if ($validator->fails()) {
            return Response::json(['errors' => $validator->errors()], 400);
        }

        return Response::json([
            'user_id' => $user_id,
            'post_id' => $post_id,
            'title' => 'Sample Post',
        ]);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new PostsController());
```
