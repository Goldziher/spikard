<?php

declare(strict_types=1);

namespace Spikard\Config;

use Closure;
use Spikard\Http\Request;

final class LifecycleHooks
{
    /**
     * @param null|Closure(Request): HookResult $onRequest
     * @param null|Closure(Request): HookResult $preValidation
     * @param null|Closure(Request): HookResult $preHandler
     * @param null|Closure(Request, \Throwable): HookResult $onError
     * @param null|Closure(Request, HookResult): HookResult $onResponse
     */
    public function __construct(
        public readonly ?Closure $onRequest = null,
        public readonly ?Closure $preValidation = null,
        public readonly ?Closure $preHandler = null,
        public readonly ?Closure $onError = null,
        public readonly ?Closure $onResponse = null,
    ) {
    }

    public static function builder(): LifecycleHooksBuilder
    {
        return new LifecycleHooksBuilder();
    }
}

final class LifecycleHooksBuilder
{
    private ?Closure $onRequest = null;
    private ?Closure $preValidation = null;
    private ?Closure $preHandler = null;
    private ?Closure $onError = null;
    private ?Closure $onResponse = null;

    /** @param Closure(Request): HookResult $callback */
    public function onRequest(Closure $callback): self
    {
        $this->onRequest = $callback;
        return $this;
    }

    /** @param Closure(Request): HookResult $callback */
    public function withOnRequest(Closure $callback): self
    {
        $this->onRequest = $callback;
        return $this;
    }

    /** @param Closure(Request): HookResult $callback */
    public function preValidation(Closure $callback): self
    {
        $this->preValidation = $callback;
        return $this;
    }

    /** @param Closure(Request): HookResult $callback */
    public function withPreValidation(Closure $callback): self
    {
        $this->preValidation = $callback;
        return $this;
    }

    /** @param Closure(Request): HookResult $callback */
    public function preHandler(Closure $callback): self
    {
        $this->preHandler = $callback;
        return $this;
    }

    /** @param Closure(Request): HookResult $callback */
    public function withPreHandler(Closure $callback): self
    {
        $this->preHandler = $callback;
        return $this;
    }

    /** @param Closure(Request, \Throwable): HookResult $callback */
    public function onError(Closure $callback): self
    {
        $this->onError = $callback;
        return $this;
    }

    /** @param Closure(Request, \Throwable): HookResult $callback */
    public function withOnError(Closure $callback): self
    {
        $this->onError = $callback;
        return $this;
    }

    /** @param Closure(Request, HookResult): HookResult $callback */
    public function onResponse(Closure $callback): self
    {
        $this->onResponse = $callback;
        return $this;
    }

    /** @param Closure(Request, HookResult): HookResult $callback */
    public function withOnResponse(Closure $callback): self
    {
        $this->onResponse = $callback;
        return $this;
    }

    public function build(): LifecycleHooks
    {
        return new LifecycleHooks(
            onRequest: $this->onRequest,
            preValidation: $this->preValidation,
            preHandler: $this->preHandler,
            onError: $this->onError,
            onResponse: $this->onResponse,
        );
    }
}
