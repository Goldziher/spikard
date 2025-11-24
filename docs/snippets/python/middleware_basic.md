```python
def logging_middleware(ctx, next_fn):
    print(f"{ctx.method} {ctx.path}")
    return next_fn()

app.use(logging_middleware)
```
