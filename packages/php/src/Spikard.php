<?php

declare(strict_types=1);

namespace Spikard;

/**
 * Placeholder PHP facade for the upcoming Rust-backed bindings.
 */
final class Spikard
{
    public const VERSION = '0.1.3';

    public static function version(): string
    {
        return self::VERSION;
    }
}
