```python
from spikard import SseEvent, sse

@sse("/events")
async def events():
    for i in range(3):
        yield SseEvent(data={"tick": i})
```
