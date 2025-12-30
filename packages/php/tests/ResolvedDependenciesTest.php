<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\DI\ResolvedDependencies;

final class ResolvedDependenciesTest extends TestCase
{
    public function testGetReturnsResolvedValue(): void
    {
        $resolved = new ResolvedDependencies(['config' => 'value']);

        $this->assertSame('value', $resolved->get('config'));
    }

    public function testGetThrowsWhenMissing(): void
    {
        $resolved = new ResolvedDependencies();

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage("Dependency 'missing' was not resolved.");

        $resolved->get('missing');
    }

    public function testAllReturnsSnapshot(): void
    {
        $data = ['flag' => true, 'count' => 2];

        $resolved = new ResolvedDependencies($data);

        $this->assertSame($data, $resolved->all());
    }
}
