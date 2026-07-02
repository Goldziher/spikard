```typescript
import { Job } from "bull";

const resilientQueue = new Queue("resilient-tasks", {
  redis: redisConfig,
  defaultJobOptions: {
    attempts: 5,
    backoff: {
      type: "exponential",
      delay: 2000,
    },
    removeOnComplete: 100,
    removeOnFail: false,
  },
});

resilientQueue.process(async (job: Job) => {
  const { id, data } = job.data;

  // Idempotency check
  if (await isAlreadyProcessed(id)) {
    return { status: "already_processed" };
  }

  try {
    const result = await externalApiCall(data);
    await markProcessed(id);
    return result;
  } catch (error) {
    if (job.attemptsMade >= job.opts.attempts) {
      // Send to dead letter queue
      await deadLetterQueue.add({ originalJob: job.data, error: error.message });
    }
    throw error;
  }
});
```
