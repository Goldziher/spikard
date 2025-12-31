```python
@pytest.mark.asyncio
async def test_sse_stream():
    from spikard.streaming import SseEvent

    app = Spikard()

    @app.sse("/notifications")
    async def notifications():
        for i in range(3):
            yield SseEvent(data={"count": i})

    async with TestClient(app) as client:
        async with client.sse("/notifications") as event_stream:
            events = []
            async for event in event_stream:
                import json
                data = json.loads(event.data)
                events.append(data)
                if len(events) >= 3:
                    break

            assert len(events) == 3
            assert events[0] == {"count": 0}
            assert events[2] == {"count": 2}
```
