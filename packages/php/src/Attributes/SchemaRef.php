<?php

declare(strict_types=1);

namespace Spikard\Attributes;

use Attribute;

/**
 * Schema registry references for attribute-driven routes.
 *
 * Use this when schema maps are loaded at runtime. The App instance must be
 * configured with withSchemas(...) before registerController is called.
 */
#[Attribute(Attribute::TARGET_METHOD)]
final class SchemaRef
{
    public function __construct(
        public readonly ?string $request = null,
        public readonly ?string $response = null,
        public readonly ?string $parameters = null,
    ) {
    }
}
