<?php

declare(strict_types=1);

require_once __DIR__ . '/../../vendor/autoload.php';

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\StreamingResponse;

/**
 * Server-Sent Events (SSE) Example
 *
 * Demonstrates real-time server->client updates using SSE.
 * The server pushes updates to connected clients every second.
 */

$config = new ServerConfig(port: 8000);
$app = new App($config);

// SSE endpoint that streams events to clients
$app = $app->addRoute('GET', '/events', function () {
    $generator = function (): Generator {
        $count = 0;

        // Send 10 events, one per second
        while ($count < 10) {
            $data = json_encode([
                'count' => $count,
                'timestamp' => time(),
                'message' => "Event #{$count}",
            ]);

            // SSE format: "data: {json}\n\n"
            yield "data: {$data}\n\n";

            $count++;
            sleep(1); // Wait 1 second between events
        }

        // Send closing message
        yield "data: " . json_encode(['message' => 'Stream complete']) . "\n\n";
    };

    return StreamingResponse::sse($generator());
});

// Serve a simple HTML client
$app = $app->addRoute('GET', '/', function () {
    $html = <<<'HTML'
<!DOCTYPE html>
<html>
<head>
    <title>SSE Demo</title>
</head>
<body>
    <h1>Server-Sent Events Demo</h1>
    <div id="events"></div>
    <script>
        const eventSource = new EventSource('/events');
        const eventsDiv = document.getElementById('events');

        eventSource.onmessage = function(event) {
            const data = JSON.parse(event.data);
            const p = document.createElement('p');
            p.textContent = `${new Date().toLocaleTimeString()}: ${data.message} (count: ${data.count})`;
            eventsDiv.appendChild(p);

            if (data.message === 'Stream complete') {
                eventSource.close();
            }
        };

        eventSource.onerror = function(error) {
            console.error('SSE error:', error);
            eventSource.close();
        };
    </script>
</body>
</html>
HTML;

    return new \Spikard\Http\Response(
        body: $html,
        statusCode: 200,
        headers: ['Content-Type' => 'text/html; charset=utf-8']
    );
});

echo "Starting SSE server on http://127.0.0.1:8000\n";
echo "Open http://127.0.0.1:8000 in your browser to see live updates\n\n";

$app->run();
