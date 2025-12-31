```python
import pytest
from celery import current_app
from unittest.mock import patch, MagicMock

# conftest.py
@pytest.fixture
def celery_config():
    return {
        'broker_url': 'memory://',
        'result_backend': 'cache+memory://',
    }

@pytest.fixture
def celery_worker_parameters():
    return {
        'perform_ping_check': False,
    }

# test_tasks.py
def test_process_upload_enqueued(celery_app, celery_worker):
    result = process_upload.delay(123)
    assert result.task_id is not None

def test_process_upload_success(celery_app, celery_worker):
    with patch('tasks.get_file') as mock_get_file:
        mock_file = MagicMock()
        mock_get_file.return_value = mock_file

        result = process_upload.apply(args=[123])

        assert result.successful()
        assert result.result['status'] == 'completed'
        mock_get_file.assert_called_once_with(123)

def test_process_upload_retry_on_failure(celery_app, celery_worker):
    with patch('tasks.get_file', side_effect=Exception("Temporary error")):
        result = process_upload.apply(args=[123])

        assert result.failed()
        # Check retry was attempted
        assert result.traceback is not None

@pytest.mark.asyncio
async def test_upload_endpoint_creates_task(client):
    response = await client.post("/upload", json={"file_id": 123})

    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "processing"
    assert "task_id" in data
```
