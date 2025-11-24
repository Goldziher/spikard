```python
from msgspec import Struct

class Payment(Struct):
    id: str
    amount: float

@app.post("/payments")
async def create_payment(payment: Payment) -> Payment:
    return payment
```
