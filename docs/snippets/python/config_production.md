```python
import os
from spikard import Spikard
from spikard.config import (
    CompressionConfig,
    OpenApiConfig,
    RateLimitConfig,
    ServerConfig,
)

config = ServerConfig(
    host="0.0.0.0",
    port=8080,
    workers=4,
    request_timeout=60,
    max_body_size=10 * 1024 * 1024,

    # High-quality compression
    compression=CompressionConfig(
        gzip=True,
        brotli=True,
        min_size=1024,
        quality=6,
    ),

    # Protect against abuse
    rate_limit=RateLimitConfig(
        per_second=100,
        burst=200,
        ip_based=True,
    ),

    # Auto-generated docs
    openapi=OpenApiConfig(
        enabled=True,
        title="Production API",
        version="1.0.0",
    ),

    # Graceful shutdown
    graceful_shutdown=True,
    shutdown_timeout=30,
)

app = Spikard(config=config)
```
