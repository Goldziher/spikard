```python
import os
from spikard import Spikard
from spikard.config import ServerConfig

config = ServerConfig(
    host=os.getenv("SPIKARD_HOST", "127.0.0.1"),
    port=int(os.getenv("SPIKARD_PORT", "8000")),
    workers=int(os.getenv("SPIKARD_WORKERS", "1")),
    request_timeout=int(os.getenv("SPIKARD_TIMEOUT", "30")),
)

app = Spikard(config=config)

# Keep secrets in env
api_key = os.getenv("API_KEY")
db_url = os.getenv("DATABASE_URL")
```
