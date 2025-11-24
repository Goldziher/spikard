```python
from spikard import Spikard

app = Spikard()

@app.on_request
async def logging_hook(request):
    print(f"{request['method']} {request['path']}")
    return request
```
