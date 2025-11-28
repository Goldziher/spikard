<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Config\HookResult;
use Spikard\Http\Response;

final class HookResultTest extends TestCase
{
    public function testContinueVariant(): void
    {
        $result = HookResult::continue();

        $this->assertInstanceOf(HookResult::class, $result);
        $this->assertFalse($result->isShortCircuit());
        $this->assertNull($result->response());
    }

    public function testShortCircuitVariant(): void
    {
        $response = Response::json(['status' => 'blocked'], 403);
        $result = HookResult::shortCircuit($response);

        $this->assertInstanceOf(HookResult::class, $result);
        $this->assertTrue($result->isShortCircuit());
        $this->assertSame($response, $result->response());
    }

    public function testShortCircuitWithTextResponse(): void
    {
        $response = Response::text('Unauthorized', 401);
        $result = HookResult::shortCircuit($response);

        $this->assertTrue($result->isShortCircuit());
        $this->assertSame($response, $result->response());
        $this->assertSame(401, $result->response()->statusCode);
    }

    public function testShortCircuitWithCustomResponse(): void
    {
        $response = new Response(
            body: '<html>Error</html>',
            statusCode: 500,
            headers: ['Content-Type' => 'text/html']
        );
        $result = HookResult::shortCircuit($response);

        $this->assertTrue($result->isShortCircuit());
        $this->assertSame($response, $result->response());
        $resultResponse = $result->response();
        if ($resultResponse !== null) {
            $this->assertSame(500, $resultResponse->statusCode);
        }
    }

    public function testContinueHasNoResponse(): void
    {
        $result = HookResult::continue();

        $this->assertNull($result->response());
    }

    public function testMultipleContinueInstancesAreIndependent(): void
    {
        $result1 = HookResult::continue();
        $result2 = HookResult::continue();

        $this->assertNotSame($result1, $result2);
        $this->assertFalse($result1->isShortCircuit());
        $this->assertFalse($result2->isShortCircuit());
    }

    public function testMultipleShortCircuitInstancesAreIndependent(): void
    {
        $response1 = Response::json(['a' => 1], 200);
        $response2 = Response::json(['b' => 2], 201);

        $result1 = HookResult::shortCircuit($response1);
        $result2 = HookResult::shortCircuit($response2);

        $this->assertNotSame($result1, $result2);
        $this->assertSame($response1, $result1->response());
        $this->assertSame($response2, $result2->response());
    }

    public function testShortCircuitCanHaveEmptyBody(): void
    {
        $response = new Response(null, 204);
        $result = HookResult::shortCircuit($response);

        $this->assertTrue($result->isShortCircuit());
        $resultResponse = $result->response();
        if ($resultResponse !== null) {
            $this->assertNull($resultResponse->body);
            $this->assertSame(204, $resultResponse->statusCode);
        }
    }

    public function testIsContinueMethod(): void
    {
        $continueResult = HookResult::continue();
        $shortCircuitResult = HookResult::shortCircuit(Response::json([], 200));

        // Continue returns false for isShortCircuit
        $isContinue = !$continueResult->isShortCircuit();
        $this->assertTrue($isContinue);

        $isShortCircuit = $shortCircuitResult->isShortCircuit();
        $this->assertTrue($isShortCircuit);
    }
}
