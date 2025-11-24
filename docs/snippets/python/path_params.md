```python
from spikard import Spikard
from msgspec import Struct

app = Spikard()


class OrderResponse(Struct):
    id: int
    details: bool


@app.get("/orders/{order_id:int}")
async def get_order(order_id: int, include_details: bool = False) -> OrderResponse:
    return OrderResponse(id=order_id, details=include_details)
```
