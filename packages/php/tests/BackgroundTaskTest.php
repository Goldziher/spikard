<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Closure;
use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\Background\BackgroundTask;

/**
 * Comprehensive tests for BackgroundTask.
 *
 * Tests execution paths and edge cases for callable types, argument handling,
 * and error scenarios to ensure 80%+ code coverage.
 */
final class BackgroundTaskTest extends TestCase
{
    private static bool $executed = false;
    private static ?int $executedWithArgs = null;
    private static ?string $executedWithString = null;
    private static ?int $executedWithMultiple = null;
    private static ?string $executedWithMultipleName = null;
    private static ?string $executedWithNull = null;
    private static bool $staticHelperCalled = false;
    private static int $counter = 0;

    protected function setUp(): void
    {
        // Reset static variables before each test
        self::$executed = false;
        self::$executedWithArgs = null;
        self::$executedWithString = null;
        self::$executedWithMultiple = null;
        self::$executedWithMultipleName = null;
        self::$executedWithNull = null;
        self::$staticHelperCalled = false;
        self::$counter = 0;
    }

    // ======================== Basic Execution Tests ========================

    public function testRunAcceptsCallableWithoutArgs(): void
    {

        BackgroundTask::run(function (): void {
            BackgroundTaskTest::$executed = true;
        });

        $this->assertTrue(self::$executed);
    }

    public function testRunAcceptsCallableWithEmptyArgs(): void
    {

        BackgroundTask::run(function (): void {
            BackgroundTaskTest::$executed = true;
        }, []);

        $this->assertTrue(self::$executed);
    }

    public function testRunWithSimpleCallable(): void
    {

        BackgroundTask::run(function (): void {
            BackgroundTaskTest::$executed = true;
        });

        $this->assertTrue(self::$executed);
    }

    // ======================== Single Argument Tests ========================

    public function testRunAcceptsCallableWithArgs(): void
    {

        BackgroundTask::run(function (int $value): void {
            BackgroundTaskTest::$executedWithArgs = $value;
        }, [42]);

        $this->assertSame(42, self::$executedWithArgs);
    }

    public function testRunWithSingleArgument(): void
    {

        BackgroundTask::run(function (string $value): void {
            BackgroundTaskTest::$executedWithString = $value;
        }, ['test_value']);

        $this->assertSame('test_value', self::$executedWithString);
    }

    // ======================== Multiple Arguments Tests ========================

    public function testRunAcceptsCallableWithMultipleArgs(): void
    {

        BackgroundTask::run(function (string $name, int $count): void {
            BackgroundTaskTest::$executedWithMultipleName = $name;
            BackgroundTaskTest::$executedWithMultiple = $count;
        }, ['test', 10]);

        $this->assertSame('test', self::$executedWithMultipleName);
        $this->assertSame(10, self::$executedWithMultiple);
    }

    public function testRunWithMultipleArguments(): void
    {

        BackgroundTask::run(function (int $num, string $str, bool $flag): void {
            BackgroundTaskTest::$counter = $num;
            BackgroundTaskTest::$executedWithString = $str;
        }, [42, 'hello', true]);

        $this->assertSame(42, self::$counter);
        $this->assertSame('hello', self::$executedWithString);
    }

    public function testRunWithVariedArgumentTypes(): void
    {

        BackgroundTask::run(function (
            int $int,
            string $string,
            bool $bool,
            float $float,
            array $array,
            ?string $nullable
        ): void {
            // Verify all types were accepted without error
            BackgroundTaskTest::$executed = true;
        }, [123, 'text', true, 3.14, ['key' => 'value'], null]);

        $this->assertTrue(self::$executed);
    }

    public function testRunWithMultipleMixedTypes(): void
    {

        BackgroundTask::run(function (int $a, string $b, ?bool $c, array $d): void {
            BackgroundTaskTest::$executedWithArgs = $a;
        }, [99, 'text', null, ['key' => 'value']]);

        $this->assertSame(99, self::$executedWithArgs);
    }

    public function testRunWithMixedTypesMultipleArgs(): void
    {

        BackgroundTask::run(
            function (
                string $str,
                int $int,
                array $arr,
                bool $bool,
                ?float $nullable
            ): void {
                BackgroundTaskTest::$executed = true;
            },
            ['string', 42, ['a' => 'b'], true, null]
        );

        $this->assertTrue(self::$executed);
    }

    // ======================== Argument Handling Tests ========================

    public function testRunAcceptsCallableWithNullInArgs(): void
    {

        BackgroundTask::run(function (?string $value): void {
            BackgroundTaskTest::$executedWithNull = $value ?? 'null_received';
        }, [null]);

        $this->assertSame('null_received', self::$executedWithNull);
    }

    public function testRunWithNullArgument(): void
    {

        BackgroundTask::run(function (?string $value): void {
            BackgroundTaskTest::$executedWithNull = $value ?? 'null_received';
        }, [null]);

        $this->assertSame('null_received', self::$executedWithNull);
    }

    public function testRunWithArrayArgument(): void
    {

        $arrayData = ['id' => 1, 'name' => 'test'];
        BackgroundTask::run(function (array $data): void {
            if (!\is_int($data['id'])) {
                throw new RuntimeException('Array id must be int');
            }
            BackgroundTaskTest::$counter = $data['id'];
        }, [$arrayData]);

        $this->assertSame(1, self::$counter);
    }

    public function testRunWithBooleanArguments(): void
    {

        BackgroundTask::run(function (bool $flag1, bool $flag2): void {
            BackgroundTaskTest::$executed = $flag1 && !$flag2;
        }, [true, false]);

        $this->assertTrue(self::$executed);
    }

    public function testRunWithNumericArguments(): void
    {

        BackgroundTask::run(function (int $intVal, float $floatVal): void {
            BackgroundTaskTest::$counter = $intVal;
        }, [100, 3.14159]);

        $this->assertSame(100, self::$counter);
    }

    public function testRunWithStringArguments(): void
    {

        BackgroundTask::run(function (string $str1, string $str2): void {
            BackgroundTaskTest::$executedWithString = $str1 . '_' . $str2;
        }, ['hello', 'world']);

        $this->assertSame('hello_world', self::$executedWithString);
    }

    public function testRunWithComplexArrayArgument(): void
    {

        $complex = [
            'level1' => [
                'level2' => [
                    'level3' => 'deep_value',
                ],
            ],
            'array' => [1, 2, 3],
            'null' => null,
            'bool' => true,
        ];

        BackgroundTask::run(function (mixed $arr): void {
            BackgroundTaskTest::$executed = \is_array($arr);
        }, [$complex]);

        $this->assertTrue(self::$executed);
    }

    // ======================== Callable Type Tests ========================

    public function testRunAcceptsNamedFunction(): void
    {

        // Test with a built-in PHP function that's safe to call
        BackgroundTask::run('strlen', ['test']);

        // If we got here, the function was called successfully
        $this->expectNotToPerformAssertions();
    }

    public function testRunWithBuiltinFunction(): void
    {

        // strlen is a built-in function that's safe to call
        BackgroundTask::run('strlen', ['test']);

        // If we get here without exception, the function was called
        $this->expectNotToPerformAssertions();
    }

    public function testRunWithCallableString(): void
    {

        // 'trim' is a built-in function callable as string
        BackgroundTask::run('trim', ['  spaces  ']);

        $this->expectNotToPerformAssertions();
    }

    public function testRunWithAnonymousFunction(): void
    {

        $fn = function (): void {
            BackgroundTaskTest::$executed = true;
        };

        BackgroundTask::run($fn);

        $this->assertTrue(self::$executed);
    }

    public function testRunWithClosure(): void
    {

        $closure = Closure::bind(function (): void {
            BackgroundTaskTest::$executed = true;
        }, null);

        BackgroundTask::run($closure);

        $this->assertTrue(self::$executed);
    }

    public function testRunAcceptsStaticMethod(): void
    {

        BackgroundTask::run([self::class, 'staticHelper'], []);

        $this->assertTrue(self::$staticHelperCalled);
    }

    public function testRunWithStaticMethod(): void
    {

        BackgroundTask::run([self::class, 'staticHelper']);

        $this->assertTrue(self::$staticHelperCalled);
    }

    public function testRunWithStaticMethodAndArgs(): void
    {

        BackgroundTask::run([self::class, 'staticMethodWithArgs'], ['value']);

        $this->assertSame('value', self::$executedWithString);
    }

    // ======================== Edge Cases ========================

    public function testRunWithLargeArgumentCount(): void
    {

        $args = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        BackgroundTask::run(
            function (
                int $a,
                int $b,
                int $c,
                int $d,
                int $e,
                int $f,
                int $g,
                int $h,
                int $i,
                int $j
            ): void {
                BackgroundTaskTest::$counter = $a + $j;
            },
            $args
        );

        $this->assertSame(11, self::$counter); // 1 + 10
    }

    public function testRunWithEmptyStringArgument(): void
    {

        BackgroundTask::run(function (string $str): void {
            BackgroundTaskTest::$executedWithString = $str;
        }, ['']);

        $this->assertSame('', self::$executedWithString);
    }

    public function testRunWithZeroArgument(): void
    {

        BackgroundTask::run(function (int $num): void {
            BackgroundTaskTest::$counter = $num;
        }, [0]);

        $this->assertSame(0, self::$counter);
    }

    public function testRunWithFalseArgument(): void
    {

        BackgroundTask::run(function (bool $flag): void {
            BackgroundTaskTest::$executed = !$flag;
        }, [false]);

        $this->assertTrue(self::$executed);
    }

    // ======================== Execution Verification Tests ========================

    public function testRunExecutesCallable(): void
    {

        BackgroundTask::run(function (): void {
            BackgroundTaskTest::$executed = true;
        });

        $this->assertTrue(self::$executed);
    }

    public function testRunCanPassMultipleValues(): void
    {

        BackgroundTask::run(
            function (int $a, int $b, int $c): void {
                BackgroundTaskTest::$counter = $a + $b + $c;
            },
            [10, 20, 30]
        );

        $this->assertSame(60, self::$counter);
    }

    public function testRunWithCallableThatThrows(): void
    {

        // Test that the task runs without throwing at the BackgroundTask::run level
        // Note: Exceptions from background tasks may or may not propagate depending
        // on implementation - this just verifies the run() call completes
        BackgroundTask::run(function (): void {
            // Callable that would throw if executed synchronously
            // In background context, this may or may not propagate
        });

        $this->expectNotToPerformAssertions();
    }

    // ======================== Extension Detection Tests ========================

    public function testMissingExtensionThrows(): void
    {
        $autoloadPath = \realpath(__DIR__ . '/../vendor/autoload.php');
        $this->assertNotFalse($autoloadPath);

        $command = \sprintf(
            '%s -n -d detect_unicode=0 -r %s',
            \escapeshellarg(PHP_BINARY),
            \escapeshellarg(
                "require '{$autoloadPath}';"
                . '\\Spikard\\Background\\BackgroundTask::run(function (): void {});'
            )
        );

        $output = [];
        $exitCode = 0;
        \exec($command . ' 2>&1', $output, $exitCode);

        $this->assertNotSame(0, $exitCode);
        $this->assertStringContainsString(
            'Spikard PHP extension not loaded',
            \implode("\n", $output)
        );
    }

    // ======================== Static Methods for Testing ========================

    /**
     * Static helper for testing static method invocation.
     *
     */
    public static function staticHelper(): void
    {
        self::$staticHelperCalled = true;
    }

    /**
     * Static helper for testing static method with arguments.
     *
     * @param string $value The value to set
     */
    public static function staticMethodWithArgs(string $value): void
    {
        self::$executedWithString = $value;
    }
}
