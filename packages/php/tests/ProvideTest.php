<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\DI\Provide;

final class ProvideTest extends TestCase
{
    public function testFactoryWithNoDepsSingletonFalse(): void
    {
        $provide = Provide::factory(fn () => 'value');

        $this->assertInstanceOf(Provide::class, $provide);
        /** @phpstan-ignore-next-line alreadyNarrowedType */
        $this->assertIsCallable($provide->factory);
        $this->assertSame([], $provide->dependsOn);
        $this->assertFalse($provide->singleton);
    }

    public function testFactoryWithDependencies(): void
    {
        $provide = Provide::factory(
            fn (string $dep1, int $dep2) => "result: $dep1, $dep2",
            ['dep1', 'dep2']
        );

        $this->assertSame(['dep1', 'dep2'], $provide->dependsOn);
        $this->assertFalse($provide->singleton);
    }

    public function testFactoryWithSingleton(): void
    {
        $provide = Provide::factory(fn () => 'value', [], true);

        $this->assertTrue($provide->singleton);
        $this->assertSame([], $provide->dependsOn);
    }

    public function testFactoryWithAllParameters(): void
    {
        $factory = fn (string $db, string $cache) => ['db' => $db, 'cache' => $cache];
        $provide = Provide::factory($factory, ['db', 'cache'], true);

        $this->assertSame($factory, $provide->factory);
        $this->assertSame(['db', 'cache'], $provide->dependsOn);
        $this->assertTrue($provide->singleton);
    }

    public function testConstructorDirectUsage(): void
    {
        $factory = fn () => 'test';
        $provide = new Provide($factory, ['dep1'], true);

        $this->assertSame($factory, $provide->factory);
        $this->assertSame(['dep1'], $provide->dependsOn);
        $this->assertTrue($provide->singleton);
    }

    public function testFactoryIsCallable(): void
    {
        $provide = Provide::factory(fn () => 42);

        $result = ($provide->factory)();
        $this->assertSame(42, $result);
    }

    public function testFactoryWithClosureCapture(): void
    {
        $captured = 'captured_value';
        $provide = Provide::factory(fn () => $captured);

        $result = ($provide->factory)();
        $this->assertSame('captured_value', $result);
    }

    public function testEmptyDependsOnArray(): void
    {
        $provide = Provide::factory(fn () => 'value', []);

        $this->assertSame([], $provide->dependsOn);
        $this->assertCount(0, $provide->dependsOn);
    }

    public function testMultipleDependencies(): void
    {
        $provide = Provide::factory(
            fn () => 'result',
            ['dep1', 'dep2', 'dep3', 'dep4']
        );

        $this->assertCount(4, $provide->dependsOn);
        $this->assertContains('dep1', $provide->dependsOn);
        $this->assertContains('dep4', $provide->dependsOn);
    }
}
