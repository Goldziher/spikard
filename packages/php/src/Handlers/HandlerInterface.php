<?php

declare(strict_types=1);

namespace Spikard\Handlers;

use Spikard\Http\Request;
use Spikard\Http\Response;

interface HandlerInterface
{
    public function handle(Request $request): Response;
}
