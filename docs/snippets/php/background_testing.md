```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\Background\BackgroundTask;
use Spikard\Testing\QueueFake;

final class BackgroundTaskTest extends TestCase
{
    protected function setUp(): void
    {
        QueueFake::fake(); // Don't actually process jobs
    }

    public function testEnqueuesJobWithCorrectArguments(): void
    {
        $this->assertCount(0, QueueFake::jobs());

        BackgroundTask::run(function (): void {
            sendEmail(123);
        });

        $this->assertCount(1, QueueFake::jobs());
    }

    public function testProcessesTaskSuccessfully(): void
    {
        QueueFake::processSync(); // Actually run jobs

        $result = BackgroundTask::run(function (): array {
            return ['status' => 'completed'];
        });

        $this->assertEquals('completed', $result['status']);
    }

    public function testHandlesFailureGracefully(): void
    {
        QueueFake::processSync();

        $this->expectException(\RuntimeException::class);

        BackgroundTask::run(function (): void {
            throw new \RuntimeException('Processing failed');
        });
    }
}
```
