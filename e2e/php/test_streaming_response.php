<?php

declare(strict_types=1);

require_once __DIR__ . '/../../vendor/autoload.php';

use Spikard\Http\StreamingResponse;

/**
 * Test streaming response functionality.
 *
 * Demonstrates:
 * 1. Basic Generator streaming
 * 2. SSE (Server-Sent Events) streaming
 * 3. File streaming
 * 4. JSON Lines streaming
 */

echo "=== Streaming Response Test ===\n\n";

// Test 1: Basic Generator streaming
echo "Test 1: Basic Generator streaming\n";

$basicGenerator = function (): Generator {
    for ($i = 1; $i <= 5; $i++) {
        yield "Chunk {$i}\n";
    }
};

$response = new StreamingResponse($basicGenerator());
echo "✓ Created StreamingResponse with basic generator\n";
echo "  - Status: {$response->statusCode}\n";
echo "  - Headers: " . json_encode($response->headers) . "\n";

// Manually consume generator to verify chunks
echo "  - Chunks: ";
foreach ($response->generator as $chunk) {
    echo json_encode($chunk) . " ";
}
echo "\n";

// Test 2: SSE streaming
echo "\nTest 2: Server-Sent Events (SSE) streaming\n";

$sseGenerator = function (): Generator {
    for ($i = 1; $i <= 3; $i++) {
        $data = json_encode(['count' => $i, 'timestamp' => time()]);
        yield "data: {$data}\n\n";
    }
};

$sseResponse = StreamingResponse::sse($sseGenerator());
echo "✓ Created SSE StreamingResponse\n";
echo "  - Content-Type: {$sseResponse->headers['Content-Type']}\n";
echo "  - Cache-Control: {$sseResponse->headers['Cache-Control']}\n";

// Test 3: File streaming
echo "\nTest 3: File streaming\n";

// Create a temporary file
$tempFile = tempnam(sys_get_temp_dir(), 'stream_test_');
file_put_contents($tempFile, "Line 1\nLine 2\nLine 3\n");

try {
    $fileResponse = StreamingResponse::file($tempFile, chunkSize: 10);
    echo "✓ Created file StreamingResponse\n";
    echo "  - File: {$tempFile}\n";
    echo "  - Chunk size: 10 bytes\n";

    // Count chunks
    $chunkCount = 0;
    foreach ($fileResponse->generator as $chunk) {
        $chunkCount++;
    }
    echo "  - Total chunks: {$chunkCount}\n";
} finally {
    unlink($tempFile);
}

// Test 4: JSON Lines streaming
echo "\nTest 4: JSON Lines (JSONL) streaming\n";

$dataGenerator = function (): Generator {
    $users = [
        ['id' => 1, 'name' => 'Alice'],
        ['id' => 2, 'name' => 'Bob'],
        ['id' => 3, 'name' => 'Charlie'],
    ];

    foreach ($users as $user) {
        yield $user;
    }
};

$jsonLinesResponse = StreamingResponse::jsonLines($dataGenerator());
echo "✓ Created JSON Lines StreamingResponse\n";
echo "  - Content-Type: {$jsonLinesResponse->headers['Content-Type']}\n";
echo "  - Lines:\n";

foreach ($jsonLinesResponse->generator as $line) {
    echo "    " . trim($line) . "\n";
}

// Test 5: Custom headers and status
echo "\nTest 5: Custom headers and status\n";

$customGenerator = function (): Generator {
    yield "Custom response";
};

$customResponse = new StreamingResponse(
    $customGenerator(),
    statusCode: 206,
    headers: [
        'Content-Type' => 'text/plain',
        'X-Custom-Header' => 'CustomValue',
    ]
);

echo "✓ Created StreamingResponse with custom config\n";
echo "  - Status: {$customResponse->statusCode}\n";
echo "  - Headers: " . json_encode($customResponse->headers) . "\n";

echo "\n=== Test Complete ===\n";
