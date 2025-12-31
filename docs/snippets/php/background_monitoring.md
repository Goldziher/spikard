```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Background\QueueStats;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class JobHealthController
{
    #[Get('/health/jobs')]
    public function jobHealth(Request $request): Response
    {
        $stats = QueueStats::all();

        return Response::json([
            'active_jobs' => $stats->activeCount(),
            'scheduled_jobs' => $stats->scheduledCount(),
            'workers' => $stats->workerCount(),
        ]);
    }

    #[Get('/jobs/{taskId}/status')]
    public function jobStatus(Request $request): Response
    {
        $taskId = $request->pathParams['taskId'];
        $result = QueueStats::getTask($taskId);

        if ($result === null) {
            return Response::json(['error' => 'Job not found', 'taskId' => $taskId], 404);
        }

        return Response::json([
            'task_id' => $taskId,
            'status' => $result->status,
            'result' => $result->isReady() ? $result->result : null,
        ]);
    }
}
```
