```python
from spikard import Spikard
from celery import Celery
from msgspec import Struct
import redis

# celery_app.py
celery_app = Celery(
    'tasks',
    broker='redis://localhost:6379/0',
    backend='redis://localhost:6379/1'
)

celery_app.conf.update(
    task_serializer='json',
    accept_content=['json'],
    result_serializer='json',
    task_track_started=True,
    task_acks_late=True,
    worker_prefetch_multiplier=1,
)

@celery_app.task(bind=True, max_retries=3)
def process_upload(self, file_id: int):
    try:
        # Heavy file processing
        file = get_file(file_id)
        result = perform_analysis(file)
        notify_completion(file_id, result)
        return {'status': 'completed', 'file_id': file_id}
    except Exception as e:
        # Exponential backoff retry
        raise self.retry(exc=e, countdown=2 ** self.request.retries)

@celery_app.task(max_retries=5)
def send_email(user_id: int, template: str, params: dict):
    user = get_user(user_id)
    email_service = EmailService(user.email)
    email_service.send_template(template, params)

# main.py
app = Spikard()

class User(Struct):
    id: int
    email: str
    name: str

@app.post("/upload")
async def upload_file(file_id: int) -> dict:
    task = process_upload.delay(file_id)
    return {"status": "processing", "task_id": task.id}

@app.post("/signup")
async def signup(user: User) -> User:
    # Save user first
    saved_user = save_user(user)
    send_email.delay(saved_user.id, "welcome", {"name": user.name})
    return saved_user
```
