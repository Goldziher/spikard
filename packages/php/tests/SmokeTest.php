<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Http\Response;
use Spikard\Spikard;

final class SmokeTest extends TestCase
{
    public function testVersionMatchesHelper(): void
    {
        $this->assertSame(Spikard::VERSION, Spikard::version());
    }

    public function testResponseJsonHelper(): void
    {
        $response = Response::json(['ok' => true], 201);
        $this->assertSame(201, $response->statusCode);
        $this->assertSame(['Content-Type' => 'application/json'], $response->headers);
        $this->assertSame(['ok' => true], $response->body);
    }
}
