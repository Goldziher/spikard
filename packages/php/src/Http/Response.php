<?php

declare(strict_types=1);

namespace Spikard\Http;

if (\class_exists(\Spikard\Internal\Response::class, false)) {
    \class_alias(\Spikard\Internal\Response::class, __NAMESPACE__ . '\Response');
} else {
    if (!\class_exists(\Spikard\Generated\Response::class, false)) {
        require_once __DIR__ . '/../Generated/Response.php';
    }

    if (!\class_exists(\Spikard\Generated\Response::class, false)) {
        throw new \RuntimeException(
            'Generated Response DTO is missing; run php packages/php/bin/generate-dto.php.'
        );
    }

    if (!\class_exists(__NAMESPACE__ . '\Response', false)) {
        \class_alias(\Spikard\Generated\Response::class, __NAMESPACE__ . '\Response');
    }
}
