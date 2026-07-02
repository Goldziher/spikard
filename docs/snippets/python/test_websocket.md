```python
@pytest.mark.asyncio
async def test_websocket_echo():
    app = Spikard()

    @app.websocket("/echo")
    async def echo(message):
        return message

    async with TestClient(app) as client:
        async with client.websocket("/echo") as ws:
            await ws.send("Hello")
            response = await ws.recv()
            assert response == "Hello"

            # Send JSON
            import json
            await ws.send(json.dumps({"type": "ping"}))
            data = await ws.recv()
            assert json.loads(data) == {"type": "ping"}
```
