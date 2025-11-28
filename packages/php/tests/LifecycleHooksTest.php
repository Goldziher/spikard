<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Config\HookResult;
use Spikard\Config\LifecycleHooks;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Throwable;

final class LifecycleHooksTest extends TestCase
{
    public function testEmptyLifecycleHooks(): void
    {
        $hooks = new LifecycleHooks();

        $this->assertNull($hooks->onRequest);
        $this->assertNull($hooks->preValidation);
        $this->assertNull($hooks->preHandler);
        $this->assertNull($hooks->onError);
        $this->assertNull($hooks->onResponse);
    }

    public function testBuilderCreatesEmptyHooks(): void
    {
        $hooks = LifecycleHooks::builder()->build();

        $this->assertInstanceOf(LifecycleHooks::class, $hooks);
        $this->assertNull($hooks->onRequest);
    }

    public function testBuilderWithOnRequest(): void
    {
        $callback = fn (Request $req): HookResult => HookResult::continue();

        $hooks = LifecycleHooks::builder()
            ->onRequest($callback)
            ->build();

        $this->assertSame($callback, $hooks->onRequest);
        $this->assertNull($hooks->preValidation);
    }

    public function testBuilderWithPreValidation(): void
    {
        $callback = fn (Request $req): HookResult => HookResult::continue();

        $hooks = LifecycleHooks::builder()
            ->preValidation($callback)
            ->build();

        $this->assertSame($callback, $hooks->preValidation);
    }

    public function testBuilderWithPreHandler(): void
    {
        $callback = fn (Request $req): HookResult => HookResult::continue();

        $hooks = LifecycleHooks::builder()
            ->preHandler($callback)
            ->build();

        $this->assertSame($callback, $hooks->preHandler);
    }

    public function testBuilderWithOnError(): void
    {
        $callback = fn (Request $req, Throwable $error): HookResult => HookResult::continue();

        $hooks = LifecycleHooks::builder()
            ->onError($callback)
            ->build();

        $this->assertSame($callback, $hooks->onError);
    }

    public function testBuilderWithOnResponse(): void
    {
        $callback = fn (Request $req, HookResult $result): HookResult => HookResult::continue();

        $hooks = LifecycleHooks::builder()
            ->onResponse($callback)
            ->build();

        $this->assertSame($callback, $hooks->onResponse);
    }

    public function testBuilderWithAllHooks(): void
    {
        $onRequest = fn (Request $req): HookResult => HookResult::continue();
        $preValidation = fn (Request $req): HookResult => HookResult::continue();
        $preHandler = fn (Request $req): HookResult => HookResult::continue();
        $onError = fn (Request $req, Throwable $e): HookResult => HookResult::continue();
        $onResponse = fn (Request $req, HookResult $r): HookResult => HookResult::continue();

        $hooks = LifecycleHooks::builder()
            ->onRequest($onRequest)
            ->preValidation($preValidation)
            ->preHandler($preHandler)
            ->onError($onError)
            ->onResponse($onResponse)
            ->build();

        $this->assertSame($onRequest, $hooks->onRequest);
        $this->assertSame($preValidation, $hooks->preValidation);
        $this->assertSame($preHandler, $hooks->preHandler);
        $this->assertSame($onError, $hooks->onError);
        $this->assertSame($onResponse, $hooks->onResponse);
    }

    public function testBuilderChainingWorks(): void
    {
        $builder = LifecycleHooks::builder();
        $result1 = $builder->onRequest(fn () => HookResult::continue());
        $result2 = $result1->preValidation(fn () => HookResult::continue());

        $this->assertSame($builder, $result1);
        $this->assertSame($builder, $result2);
    }

    public function testHookCallbacksAreCallable(): void
    {
        $hooks = LifecycleHooks::builder()
            ->onRequest(fn (Request $req): HookResult => HookResult::continue())
            ->build();

        $this->assertIsCallable($hooks->onRequest);

        $request = new Request(
            method: 'GET',
            path: '/test',
            body: null,
            headers: [],
            cookies: []
        );

        if ($hooks->onRequest !== null) {
            $result = ($hooks->onRequest)($request);
            $this->assertInstanceOf(HookResult::class, $result);
        }
    }

    public function testOnRequestCanReturnContinue(): void
    {
        $hooks = LifecycleHooks::builder()
            ->onRequest(fn (Request $req): HookResult => HookResult::continue())
            ->build();

        $request = new Request('GET', '/test', null);
        if ($hooks->onRequest !== null) {
            $result = ($hooks->onRequest)($request);

            $this->assertInstanceOf(HookResult::class, $result);
            $this->assertFalse($result->isShortCircuit());
        }
    }

    public function testOnRequestCanReturnShortCircuit(): void
    {
        $response = Response::json(['early' => true], 200);
        $hooks = LifecycleHooks::builder()
            ->onRequest(fn (Request $req): HookResult => HookResult::shortCircuit($response))
            ->build();

        $request = new Request('GET', '/test', null);
        if ($hooks->onRequest !== null) {
            $result = ($hooks->onRequest)($request);

            $this->assertTrue($result->isShortCircuit());
            $this->assertSame($response, $result->response());
        }
    }

    public function testOnErrorReceivesThrowable(): void
    {
        $capturedError = null;
        $hooks = LifecycleHooks::builder()
            ->onError(function (Request $req, Throwable $error) use (&$capturedError): HookResult {
                $capturedError = $error;
                return HookResult::continue();
            })
            ->build();

        $exception = new \RuntimeException('Test error');
        $request = new Request('GET', '/test', null);

        if ($hooks->onError !== null) {
            $result = ($hooks->onError)($request, $exception);

            $this->assertSame($exception, $capturedError);
            $this->assertInstanceOf(HookResult::class, $result);
        }
    }

    public function testOnResponseReceivesHookResult(): void
    {
        $capturedResult = null;
        $hooks = LifecycleHooks::builder()
            ->onResponse(function (Request $req, HookResult $result) use (&$capturedResult): HookResult {
                $capturedResult = $result;
                return $result;
            })
            ->build();

        $request = new Request('GET', '/test', null);
        $originalResult = HookResult::continue();

        if ($hooks->onResponse !== null) {
            $result = ($hooks->onResponse)($request, $originalResult);

            $this->assertSame($originalResult, $capturedResult);
            $this->assertSame($originalResult, $result);
        }
    }

    public function testBuilderCanOverwriteHooks(): void
    {
        $callback1 = fn (Request $req): HookResult => HookResult::continue();
        $callback2 = fn (Request $req): HookResult => HookResult::shortCircuit(
            Response::json([], 200)
        );

        $hooks = LifecycleHooks::builder()
            ->onRequest($callback1)
            ->onRequest($callback2) // Overwrite
            ->build();

        $this->assertSame($callback2, $hooks->onRequest);
        $this->assertNotSame($callback1, $hooks->onRequest);
    }

    public function testConstructorDirectUsage(): void
    {
        $onRequest = fn (Request $req): HookResult => HookResult::continue();
        $onError = fn (Request $req, Throwable $e): HookResult => HookResult::continue();

        $hooks = new LifecycleHooks(
            onRequest: $onRequest,
            onError: $onError
        );

        $this->assertSame($onRequest, $hooks->onRequest);
        $this->assertNull($hooks->preValidation);
        $this->assertSame($onError, $hooks->onError);
    }
}
