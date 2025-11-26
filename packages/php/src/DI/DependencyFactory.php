<?php

declare(strict_types=1);

namespace Spikard\DI;

use Spikard\Http\Request;

/** Factory callable used to build a dependency. */
interface DependencyFactory
{
    public function __invoke(Request $request, ResolvedDependencies $resolved): mixed;
}
