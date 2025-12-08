<?php

declare(strict_types=1);

namespace Spikard\Handlers;

use Spikard\Http\Request;
use Spikard\Http\Response;

interface HandlerInterface
{
    public function matches(\Spikard\Http\Request $request): bool;

    public function handle(Request $request): Response;

    /**
     * Make handler objects callable for Rust FFI compatibility.
     *
     * When a handler object is passed to the Rust extension, it must be callable.
     * This __invoke method allows handler instances to work as PHP callables.
     *
     * @param Request $request The HTTP request
     * @return Response The HTTP response
     */
    public function __invoke(Request $request): Response;
}
