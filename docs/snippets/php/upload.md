```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Http\UploadFile;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('POST', '/upload', function (Request $request) {
    $file = $request->file('file');

    return Response::json([
        'filename' => $file->filename,
        'size' => $file->size
    ]);
});
```
