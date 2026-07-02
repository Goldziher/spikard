```python
from celery import Task
from celery.exceptions import MaxRetriesExceededError

class ResilientTask(Task):
    autoretry_for = (Exception,)
    retry_kwargs = {'max_retries': 5}
    retry_backoff = True
    retry_backoff_max = 600
    retry_jitter = True

@celery_app.task(base=ResilientTask)
def process_with_recovery(data: dict):
    # Check idempotency
    if is_already_processed(data['id']):
        return {'status': 'already_processed'}

    try:
        result = external_api_call(data)
        mark_processed(data['id'])
        return result
    except MaxRetriesExceededError:
        # Send to dead letter queue
        send_to_dlq(data)
        raise
```
