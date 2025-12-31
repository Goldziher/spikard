```typescript
import { Queue, Job } from "bull";
import { jest } from "@jest/globals";

describe("Upload Queue", () => {
  let testQueue: Queue;

  beforeEach(() => {
    testQueue = new Queue("test-upload", {
      redis: { host: "localhost", port: 6379, db: 1 },
    });
  });

  afterEach(async () => {
    await testQueue.close();
  });

  it("enqueues upload job with correct data", async () => {
    const job = await testQueue.add({ fileId: 123 });

    expect(job.id).toBeDefined();
    expect(job.data.fileId).toBe(123);
  });

  it("processes upload successfully", async () => {
    const getFile = jest.fn().mockResolvedValue({ id: 123, name: "test.txt" });
    const processFile = jest.fn().mockResolvedValue({ success: true });

    testQueue.process(async (job: Job) => {
      const file = await getFile(job.data.fileId);
      return await processFile(file);
    });

    const job = await testQueue.add({ fileId: 123 });
    await job.finished();

    expect(getFile).toHaveBeenCalledWith(123);
    expect(processFile).toHaveBeenCalled();
  });

  it("retries on failure", async () => {
    let attempts = 0;

    testQueue.process(async (job: Job) => {
      attempts++;
      if (attempts < 3) {
        throw new Error("Temporary failure");
      }
      return { success: true };
    });

    const job = await testQueue.add(
      { fileId: 123 },
      { attempts: 3, backoff: 100 }
    );

    const result = await job.finished();
    expect(attempts).toBe(3);
    expect(result.success).toBe(true);
  });

  it("moves to failed after max retries", async () => {
    testQueue.process(async (job: Job) => {
      throw new Error("Persistent failure");
    });

    const job = await testQueue.add(
      { fileId: 123 },
      { attempts: 2 }
    );

    await expect(job.finished()).rejects.toThrow("Persistent failure");
    const state = await job.getState();
    expect(state).toBe("failed");
  });
});
```
