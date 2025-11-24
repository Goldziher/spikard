```python
def auth_middleware(ctx, next_fn):
    token = ctx.headers.get("authorization")
    if token != "Bearer dev-token":
        return {"error": "unauthorized"}, 401
    return next_fn()

app.use(auth_middleware)
```
