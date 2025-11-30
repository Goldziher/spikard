<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\Background\BackgroundTask;

final class BackgroundTaskTest extends TestCase
{
    private static bool $executedWithoutArgs = false;
    private static ?int $executedWithArgs = null;
    private static ?string $executedWithString = null;
    private static ?int $executedWithMultiple = null;
    private static ?string $executedWithMultipleName = null;
    private static ?string $executedWithNull = null;
    private static bool $staticHelperCalled = false;

    protected function setUp(): void
    {
        // Reset static variables before each test
        self::$executedWithoutArgs = false;
        self::$executedWithArgs = null;
        self::$executedWithString = null;
        self::$executedWithMultiple = null;
        self::$executedWithMultipleName = null;
        self::$executedWithNull = null;
        self::$staticHelperCalled = false;
    }

    public function testRunAcceptsCallableWithoutArgs(): void
    {
        BackgroundTask::run(function (): void {
            BackgroundTaskTest::$executedWithoutArgs = true;
        });

        $this->assertTrue(self::$executedWithoutArgs);
    }

    public function testRunAcceptsCallableWithEmptyArgs(): void
    {
        BackgroundTask::run(function (): void {
            BackgroundTaskTest::$executedWithoutArgs = true;
        }, []);

        $this->assertTrue(self::$executedWithoutArgs);
    }

    public function testRunAcceptsCallableWithArgs(): void
    {
        BackgroundTask::run(function (int $value): void {
            BackgroundTaskTest::$executedWithArgs = $value;
        }, [42]);

        $this->assertSame(42, self::$executedWithArgs);
    }

    public function testRunAcceptsCallableWithMultipleArgs(): void
    {
        BackgroundTask::run(function (string $name, int $count): void {
            BackgroundTaskTest::$executedWithMultipleName = $name;
            BackgroundTaskTest::$executedWithMultiple = $count;
        }, ['test', 10]);

        $this->assertSame('test', self::$executedWithMultipleName);
        $this->assertSame(10, self::$executedWithMultiple);
    }

    public function testRunAcceptsCallableWithNullInArgs(): void
    {
        BackgroundTask::run(function (?string $value): void {
            BackgroundTaskTest::$executedWithNull = $value ?? 'null_received';
        }, [null]);

        // Should execute without error, null is valid
        $this->assertSame('null_received', self::$executedWithNull);
    }

    public function testRunAcceptsNamedFunction(): void
    {
        // Test with a built-in PHP function that's safe to call
        BackgroundTask::run('strlen', ['test']);

        // If we got here, the function was called successfully
        /** @phpstan-ignore-next-line method.alreadyNarrowedType */
        $this->assertTrue(true);
    }

    public function testRunAcceptsStaticMethod(): void
    {
        BackgroundTask::run([self::class, 'staticHelper'], []);

        $this->assertTrue(self::$staticHelperCalled);
    }

    public function testRunWithClosure(): void
    {
        $testValue = 'closure_test';
        BackgroundTask::run(function (string $value): void {
            BackgroundTaskTest::$executedWithString = $value;
        }, [$testValue]);

        $this->assertSame('closure_test', self::$executedWithString);
    }

    public function testRunWithMultipleMixedTypes(): void
    {
        $result = null;
        BackgroundTask::run(function (int $a, string $b, ?bool $c, array $d): void {
            BackgroundTaskTest::$executedWithArgs = $a;
        }, [99, 'text', null, ['key' => 'value']]);

        $this->assertSame(99, self::$executedWithArgs);
    }

    public static function staticHelper(): void
    {
        self::$staticHelperCalled = true;
    }
}
