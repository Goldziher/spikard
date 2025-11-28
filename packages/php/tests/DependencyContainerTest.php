<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\DI\DependencyContainer;
use Spikard\DI\DependencyContainerBuilder;
use Spikard\DI\Provide;

final class DependencyContainerTest extends TestCase
{
    public function testEmptyContainerHasNoDependencies(): void
    {
        $container = new DependencyContainer();

        $this->assertSame([], $container->dependencies);
        $this->assertSame([], $container->getDependencies());
    }

    public function testContainerWithValueDependencies(): void
    {
        $container = new DependencyContainer(['db' => 'connection']);

        $this->assertSame(['db' => 'connection'], $container->dependencies);
    }

    public function testContainerWithFactoryDependencies(): void
    {
        $factory = Provide::factory(fn () => 'created');
        $container = new DependencyContainer([], ['service' => $factory]);

        $deps = $container->getDependencies();
        $this->assertArrayHasKey('service', $deps);
        $this->assertInstanceOf(Provide::class, $deps['service']);
    }

    public function testContainerMergesValuesAndFactories(): void
    {
        $factory = Provide::factory(fn () => 'factory_value');
        $container = new DependencyContainer(
            ['value_dep' => 'static'],
            ['factory_dep' => $factory]
        );

        $deps = $container->getDependencies();
        $this->assertArrayHasKey('value_dep', $deps);
        $this->assertArrayHasKey('factory_dep', $deps);
        $this->assertSame('static', $deps['value_dep']);
        $this->assertInstanceOf(Provide::class, $deps['factory_dep']);
    }

    public function testBuilderCreatesEmptyContainer(): void
    {
        $container = DependencyContainer::builder()->build();

        $this->assertInstanceOf(DependencyContainer::class, $container);
        $this->assertSame([], $container->dependencies);
    }

    public function testBuilderWithValueDependency(): void
    {
        $container = DependencyContainer::builder()
            ->provideValue('config', ['key' => 'value'])
            ->build();

        $deps = $container->getDependencies();
        $this->assertArrayHasKey('config', $deps);
        $this->assertSame(['key' => 'value'], $deps['config']);
    }

    public function testBuilderWithFactoryDependency(): void
    {
        $container = DependencyContainer::builder()
            ->provideFactory('service', Provide::factory(fn () => 'created'))
            ->build();

        $deps = $container->getDependencies();
        $this->assertArrayHasKey('service', $deps);
        $this->assertInstanceOf(Provide::class, $deps['service']);
    }

    public function testBuilderWithMultipleDependencies(): void
    {
        $container = DependencyContainer::builder()
            ->provideValue('db', 'connection')
            ->provideValue('cache', 'redis')
            ->provideFactory('logger', Provide::factory(fn () => 'log_instance'))
            ->build();

        $deps = $container->getDependencies();
        $this->assertCount(3, $deps);
        $this->assertSame('connection', $deps['db']);
        $this->assertSame('redis', $deps['cache']);
        $this->assertInstanceOf(Provide::class, $deps['logger']);
    }

    public function testBuilderChaining(): void
    {
        $builder = DependencyContainer::builder();
        $result1 = $builder->provideValue('a', 1);
        $result2 = $result1->provideFactory('b', Provide::factory(fn () => 2));

        $this->assertSame($builder, $result1);
        $this->assertSame($builder, $result2);
    }

    public function testContainerWithComplexValueTypes(): void
    {
        $container = new DependencyContainer([
            'int' => 42,
            'float' => 3.14,
            'string' => 'text',
            'bool' => true,
            'null' => null,
            'array' => [1, 2, 3],
            'nested' => ['a' => ['b' => 'c']],
        ]);

        $deps = $container->getDependencies();
        $this->assertSame(42, $deps['int']);
        $this->assertSame(3.14, $deps['float']);
        $this->assertSame('text', $deps['string']);
        $this->assertTrue($deps['bool']);
        $this->assertNull($deps['null']);
        $this->assertSame([1, 2, 3], $deps['array']);
        $this->assertSame(['a' => ['b' => 'c']], $deps['nested']);
    }
}
