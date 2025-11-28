<?php

declare(strict_types=1);

require_once __DIR__ . '/../../packages/php/vendor/autoload.php';

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Response;

/**
 * Hello World Example
 *
 * The simplest possible Spikard application.
 * Starts a server on port 8000 with a single route that returns "Hello, World!".
 */

$config = new ServerConfig(port: 8000);
$app = new App($config);

$app = $app->addRoute('GET', '/', function () {
    return Response::text('Hello, World!');
});

echo "Starting server on http://127.0.0.1:8000\n";
echo "Press Ctrl+C to stop\n\n";

$app->run();
