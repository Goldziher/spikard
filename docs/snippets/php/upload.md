```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class UploadController
{
    #[Post('/upload')]
    public function upload(Request $request): Response
    {
        $file = $request->files['file'] ?? null;

        if ($file === null) {
            return Response::json(['error' => 'No file uploaded'], 400);
        }

        $filename = $file['filename'] ?? 'unknown';
        $size = $file['size'] ?? 0;
        $contentType = $file['content_type'] ?? 'application/octet-stream';

        return Response::json([
            'filename' => $filename,
            'size' => $size,
            'content_type' => $contentType,
            'received' => true
        ]);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new UploadController());
```
