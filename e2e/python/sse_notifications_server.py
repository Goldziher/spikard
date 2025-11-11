#!/usr/bin/env python3
"""SSE notifications server using Spikard SSE support"""

import asyncio

from spikard import Spikard
from spikard.config import ServerConfig
from spikard.sse import SseEvent, SseEventProducer


class NotificationProducer(SseEventProducer):
    """SSE event producer for notifications"""

    def __init__(self):
        self.count = 0
        self.max_events = 10

    async def next_event(self) -> SseEvent | None:
        """Generate the next notification event"""
        if self.count >= self.max_events:
            print("[NotificationProducer] Reached max events, ending stream")
            return None

        # Wait 1 second between events
        await asyncio.sleep(1)

        # Create notification event
        event = SseEvent(
            data={
                "notification": f"Notification {self.count}",
                "timestamp": self.count,
                "priority": "normal",
            },
            event_type="notification",
            id=str(self.count),
        )

        print(f"[NotificationProducer] Sending event {self.count}")
        self.count += 1
        return event

    async def on_connect(self) -> None:
        """Called when a client connects"""
        print("[NotificationProducer] Client connected to SSE stream")

    async def on_disconnect(self) -> None:
        """Called when a client disconnects"""
        print(f"[NotificationProducer] Client disconnected (sent {self.count} events)")


# Create Spikard app
app = Spikard()


# Register SSE endpoint
@app.sse("/notifications")
def notifications_endpoint():
    """SSE endpoint for notifications"""
    return NotificationProducer()


if __name__ == "__main__":
    print("Starting SSE notifications server on http://localhost:8000/notifications")
    config = ServerConfig(host="0.0.0.0", port=8000)
    app.run(config=config)
