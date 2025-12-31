```php
<?php

declare(strict_types=1);

use Spikard\Background\BackgroundTask;
use Spikard\Background\DeadLetterQueue;

// Error recovery with retry logic
final class ResilientTask
{
    private int $maxRetries = 5;
    private int $retryCount = 0;

    public function execute(array $data): array
    {
        // Idempotency check
        if ($this->isAlreadyProcessed($data['id'])) {
            return ['status' => 'already_processed'];
        }

        try {
            $result = $this->externalApiCall($data);
            $this->markProcessed($data['id']);
            return $result;
        } catch (\Exception $e) {
            if ($this->retryCount >= $this->maxRetries) {
                DeadLetterQueue::push($data, $e->getMessage());
            }
            throw $e;
        }
    }

    private function isAlreadyProcessed(string $id): bool
    {
        // Check if already processed
        return false;
    }

    private function markProcessed(string $id): void
    {
        // Mark as processed
    }

    private function externalApiCall(array $data): array
    {
        // Call external API
        return ['success' => true];
    }
}
```
