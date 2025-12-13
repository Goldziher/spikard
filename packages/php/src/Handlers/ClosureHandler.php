<?php

declare(strict_types=1);

namespace Spikard\Handlers;

use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Handler that wraps a closure for simple route definitions.
 *
 * Allows using anonymous functions as route handlers without needing
 * to create full handler classes.
 */
final class ClosureHandler implements HandlerInterface
{
    /**
     * @param \Closure(Request): (Response|array<string, mixed>|string|int|float|bool|null) $closure
     */
    public function __construct(
        private readonly \Closure $closure,
    ) {
    }

    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        $result = ($this->closure)($request);

        // If the closure returned a Response, use it directly
        if ($result instanceof Response) {
            return $result;
        }

        // Otherwise, wrap the result in a Response
        return new Response(
            statusCode: 200,
            body: $result,
            headers: [
                'Content-Type' => 'application/json',
            ],
        );
    }

    public function __invoke(Request $request): Response
    {
        return $this->handle($request);
    }
}
