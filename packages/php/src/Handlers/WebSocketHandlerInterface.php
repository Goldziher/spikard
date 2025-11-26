<?php

declare(strict_types=1);

namespace Spikard\Handlers;

interface WebSocketHandlerInterface
{
    public function onConnect(): void;

    public function onMessage(string $message): void;

    public function onClose(int $code, ?string $reason = null): void;
}
