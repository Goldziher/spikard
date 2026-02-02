```python
from spikard import Spikard
from msgspec import Struct

app = Spikard()


class Order(Struct):
    id: int
    item: str
    quantity: int
    verbose: bool | None = None


@app.post("/orders/{order_id:int}")
async def update_order(order_id: int, order: Order, verbose: bool | None = False) -> Order:
    return Order(id=order_id, item=order.item, quantity=order.quantity, verbose=verbose or False)
```
