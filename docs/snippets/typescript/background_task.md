```typescript
import { Spikard } from "spikard";
import Bull, { Queue, Job } from "bull";
import Redis from "ioredis";

// queues.ts
const redisConfig = {
  host: process.env.REDIS_HOST || "localhost",
  port: parseInt(process.env.REDIS_PORT || "6379"),
};

export const uploadQueue = new Queue("upload-processing", {
  redis: redisConfig,
});

export const emailQueue = new Queue("email-sending", {
  redis: redisConfig,
  defaultJobOptions: {
    attempts: 5,
    backoff: {
      type: "exponential",
      delay: 2000,
    },
  },
});

// Process upload jobs
uploadQueue.process(async (job: Job) => {
  const { fileId } = job.data;
  try {
    await job.progress(10);
    const file = await getFile(fileId);

    await job.progress(50);
    const result = await processFile(file);

    await job.progress(90);
    await notifyCompletion(fileId, result);

    await job.progress(100);
    return { status: "completed", fileId };
  } catch (error) {
    throw new Error(`Upload processing failed: ${error.message}`);
  }
});

// Process email jobs
emailQueue.process(async (job: Job) => {
  const { userId, template, params } = job.data;
  const user = await getUser(userId);
  const emailService = new EmailService(user.email);
  await emailService.sendTemplate(template, params);
});

// main.ts
const app = new Spikard();

interface User {
  id: number;
  email: string;
  name: string;
}

app.addRoute(
  { method: "POST", path: "/upload", handler_name: "upload", is_async: true },
  async (req) => {
    const { fileId } = req.json<{ fileId: number }>();
    const job = await uploadQueue.add({ fileId });
    return { status: "processing", jobId: job.id };
  }
);

app.addRoute(
  { method: "POST", path: "/signup", handler_name: "signup", is_async: true },
  async (req) => {
    const user = req.json<User>();
    const savedUser = await saveUser(user);
    await emailQueue.add({
      userId: savedUser.id,
      template: "welcome",
      params: { name: user.name }
    });
    return savedUser;
  }
);
```
