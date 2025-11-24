```python
from typing import Optional

@app.post("/orders/{order_id:int}")
async def update_order(order_id: int, order: dict, verbose: Optional[bool] = False) -> dict:
    if verbose:
        print("updating", order_id)
    return {**order, "id": order_id, "verbose": verbose or False}
```
