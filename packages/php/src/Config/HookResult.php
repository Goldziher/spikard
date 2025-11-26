<?php

declare(strict_types=1);

namespace Spikard\Config;

use Spikard\Http\Response;

/**
 * Lifecycle hook result indicating whether to continue or short-circuit.
 */
final class HookResult
{
    private function __construct(
        private readonly string $variant,
        private readonly ?Response $response = null,
    ) {
    }

    public static function continue(): self
    {
        return new self('continue');
    }

    public static function shortCircuit(Response $response): self
    {
        return new self('short_circuit', $response);
    }

    public function isShortCircuit(): bool
    {
        return $this->variant === 'short_circuit';
    }

    public function response(): ?Response
    {
        return $this->response;
    }
}
