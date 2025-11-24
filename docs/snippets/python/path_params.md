```python
@app.get("/orders/{order_id:int}")
async def get_order(order_id: int, include_details: bool = False) -> dict:
    return {"id": order_id, "details": include_details}
```
