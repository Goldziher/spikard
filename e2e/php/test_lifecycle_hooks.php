<?php

declare(strict_types=1);

require_once __DIR__ . '/../../packages/php/vendor/autoload.php';

use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Test lifecycle hooks functionality
 */

// Test 1: Request hook that continues
echo "Test 1: Request hook that continues\n";
$app1 = new App();
$app1 = $app1->onRequest(function(Request $req): ?Response {
    echo "  onRequest called: {$req->getMethod()} {$req->getPath()}\n";
    return null; // Continue
});
$app1 = $app1->addRoute('GET', '/test', new class implements HandlerInterface {
    public function __invoke(Request $req): Response {
        return new Response(200, ['message' => 'success'], []);
    }
});

try {
    $server1 = $app1->start('127.0.0.1', 18001);
    echo "  Server started on port 18001\n";

    // Make a test request
    $ch = curl_init('http://127.0.0.1:18001/test');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $response = curl_exec($ch);
    $status = curl_getinfo($ch, CURLINFO_HTTP_CODE);
    curl_close($ch);

    echo "  Response status: $status\n";
    echo "  Response body: $response\n";

    $app1->stop($server1);
    echo "  Test 1 PASSED\n\n";
} catch (Exception $e) {
    echo "  Test 1 FAILED: {$e->getMessage()}\n\n";
}

// Test 2: Request hook that short-circuits
echo "Test 2: Request hook that short-circuits\n";
$app2 = new App();
$app2 = $app2->onRequest(function(Request $req): ?Response {
    echo "  onRequest called: {$req->getMethod()} {$req->getPath()}\n";
    if ($req->getPath() === '/blocked') {
        echo "  Short-circuiting with 403\n";
        return new Response(403, ['error' => 'blocked'], []);
    }
    return null;
});
$app2 = $app2->addRoute('GET', '/blocked', new class implements HandlerInterface {
    public function __invoke(Request $req): Response {
        return new Response(200, ['message' => 'should not reach here'], []);
    }
});

try {
    $server2 = $app2->start('127.0.0.1', 18002);
    echo "  Server started on port 18002\n";

    $ch = curl_init('http://127.0.0.1:18002/blocked');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $response = curl_exec($ch);
    $status = curl_getinfo($ch, CURLINFO_HTTP_CODE);
    curl_close($ch);

    echo "  Response status: $status\n";
    echo "  Response body: $response\n";

    if ($status === 403 && strpos($response, 'blocked') !== false) {
        echo "  Test 2 PASSED\n\n";
    } else {
        echo "  Test 2 FAILED: Expected 403 with 'blocked' message\n\n";
    }

    $app2->stop($server2);
} catch (Exception $e) {
    echo "  Test 2 FAILED: {$e->getMessage()}\n\n";
}

// Test 3: Response hook that modifies response
echo "Test 3: Response hook that modifies response\n";
$app3 = new App();
$app3 = $app3->onResponse(function(Response $resp): ?Response {
    echo "  onResponse called: status={$resp->getStatus()}\n";
    $body = $resp->getBody();
    $body['modified'] = true;
    return new Response($resp->getStatus(), $body, $resp->getHeaders());
});
$app3 = $app3->addRoute('GET', '/test', new class implements HandlerInterface {
    public function __invoke(Request $req): Response {
        return new Response(200, ['message' => 'original'], []);
    }
});

try {
    $server3 = $app3->start('127.0.0.1', 18003);
    echo "  Server started on port 18003\n";

    $ch = curl_init('http://127.0.0.1:18003/test');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $response = curl_exec($ch);
    $status = curl_getinfo($ch, CURLINFO_HTTP_CODE);
    curl_close($ch);

    echo "  Response status: $status\n";
    echo "  Response body: $response\n";

    if (strpos($response, 'modified') !== false && strpos($response, 'true') !== false) {
        echo "  Test 3 PASSED\n\n";
    } else {
        echo "  Test 3 FAILED: Expected response with 'modified: true'\n\n";
    }

    $app3->stop($server3);
} catch (Exception $e) {
    echo "  Test 3 FAILED: {$e->getMessage()}\n\n";
}

// Test 4: Error hook
echo "Test 4: Error hook\n";
$app4 = new App();
$app4 = $app4->onError(function(Response $resp): ?Response {
    echo "  onError called: status={$resp->getStatus()}\n";
    $body = $resp->getBody();
    $body['error_handled'] = true;
    return new Response($resp->getStatus(), $body, $resp->getHeaders());
});
$app4 = $app4->addRoute('GET', '/error', new class implements HandlerInterface {
    public function __invoke(Request $req): Response {
        return new Response(500, ['error' => 'internal error'], []);
    }
});

try {
    $server4 = $app4->start('127.0.0.1', 18004);
    echo "  Server started on port 18004\n";

    $ch = curl_init('http://127.0.0.1:18004/error');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $response = curl_exec($ch);
    $status = curl_getinfo($ch, CURLINFO_HTTP_CODE);
    curl_close($ch);

    echo "  Response status: $status\n";
    echo "  Response body: $response\n";

    if ($status === 500 && strpos($response, 'error_handled') !== false) {
        echo "  Test 4 PASSED\n\n";
    } else {
        echo "  Test 4 FAILED: Expected 500 with 'error_handled: true'\n\n";
    }

    $app4->stop($server4);
} catch (Exception $e) {
    echo "  Test 4 FAILED: {$e->getMessage()}\n\n";
}

echo "All lifecycle hook tests completed!\n";
