<?php

declare(strict_types=1);

namespace Spikard\Http;

if (\class_exists(\Spikard\Internal\Request::class, false)) {
    \class_alias(\Spikard\Internal\Request::class, __NAMESPACE__ . '\Request');
} else {
    if (!\class_exists(\Spikard\Generated\Request::class, false)) {
        require_once __DIR__ . '/../Generated/Request.php';
    }

    if (!\class_exists(\Spikard\Generated\Request::class, false)) {
        throw new \RuntimeException(
            'Generated Request DTO is missing; run php packages/php/bin/generate-dto.php.'
        );
    }

    if (!\class_exists(__NAMESPACE__ . '\Request', false)) {
        \class_alias(\Spikard\Generated\Request::class, __NAMESPACE__ . '\Request');
    }
}
