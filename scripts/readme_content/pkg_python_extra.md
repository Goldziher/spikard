## Configuration

```python
from spikard import Spikard, ServerConfig, CompressionConfig, RateLimitConfig, JwtConfig

config = ServerConfig(
    host="0.0.0.0",
    port=8080,
    workers=4,
    compression=CompressionConfig(gzip=True, brotli=True),
    rate_limit=RateLimitConfig(per_second=100, burst=200),
    jwt=JwtConfig(secret="key", algorithm="HS256")
)

app = Spikard(config=config)
```

See the documentation for all options.
