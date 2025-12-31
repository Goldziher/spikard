```typescript
app.addRoute(
  { method: "GET", path: "/health/jobs", handler_name: "job_health", is_async: true },
  async (req) => {
    const [uploadCounts, emailCounts] = await Promise.all([
      uploadQueue.getJobCounts(),
      emailQueue.getJobCounts(),
    ]);

    return {
      queues: {
        upload: uploadCounts,
        email: emailCounts,
      },
    };
  }
);

app.addRoute(
  { method: "GET", path: "/jobs/:jobId/status", handler_name: "job_status", is_async: true },
  async (req) => {
    const jobId = req.params.jobId;
    const job = await uploadQueue.getJob(jobId) || await emailQueue.getJob(jobId);

    if (!job) {
      return { error: "Job not found", jobId };
    }

    const state = await job.getState();
    const progress = job.progress();

    return {
      jobId: job.id,
      state,
      progress,
      data: job.data,
      finishedOn: job.finishedOn,
      failedReason: job.failedReason,
    };
  }
);
```
