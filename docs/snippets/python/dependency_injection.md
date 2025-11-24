```python
from spikard import Spikard
from spikard.di import Provide

app = Spikard()

# Value dependency (singleton)
app.provide("config", {"db_url": "postgresql://localhost/app"})

# Factory dependency (depends on config, cached globally)
async def make_pool(config: dict[str, str]):
    return {"url": config["db_url"], "client": "pool"}

app.provide("db_pool", Provide(make_pool, depends_on=["config"], singleton=True))

@app.get("/stats")
async def stats(config: dict[str, str], db_pool):
    return {"db": db_pool["url"], "env": config["db_url"]}
```
