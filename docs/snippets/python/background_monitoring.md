```python
from celery.result import AsyncResult

@app.get("/health/jobs")
async def job_health():
    inspector = celery_app.control.inspect()
    active = inspector.active() or {}
    scheduled = inspector.scheduled() or {}

    return {
        "active_jobs": sum(len(tasks) for tasks in active.values()),
        "scheduled_jobs": sum(len(tasks) for tasks in scheduled.values()),
        "workers": len(active.keys()),
    }

@app.get("/jobs/{task_id}/status")
async def job_status(task_id: str):
    result = AsyncResult(task_id, app=celery_app)

    return {
        "task_id": task_id,
        "status": result.status,
        "result": result.result if result.ready() else None,
        "traceback": str(result.traceback) if result.failed() else None,
    }
```
