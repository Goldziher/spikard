```php
<?php

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

// Note: UploadFile support is in development (P1.5)
final class UploadController
{
    #[Post('/upload')]
    public function upload(Request $request): Response
    {
        $file = $request->files['file'] ?? null;

        if (!$file) {
            return Response::json(['error' => 'No file uploaded'], 400);
        }

        return Response::json([
            'filename' => $file['filename'] ?? 'unknown',
            'received' => true
        ]);
    }
}

$app = $app->registerController(new UploadController());
```
