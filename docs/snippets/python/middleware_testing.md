```python
import pytest
from your_app import auth_guard

@pytest.mark.asyncio
async def test_auth_guard_valid_token():
    request = {
        "headers": {"authorization": "Bearer valid-jwt-token"},
        "method": "GET",
        "path": "/api/users"
    }

    result = await auth_guard(request)

    assert "context" in result
    assert "user_id" in result["context"]

@pytest.mark.asyncio
async def test_auth_guard_missing_token():
    request = {"headers": {}, "method": "GET", "path": "/api/users"}

    with pytest.raises(HTTPError) as exc:
        await auth_guard(request)

    assert exc.value.status == 401
```
