```python
from spikard import websocket

@websocket("/ws")
async def echo(socket):
    async for message in socket:
        await socket.send_json({"echo": message})
```
