---
name: python-binding-async
priority: high
---
Python binding must:
- Use pyo3_async_runtimes for asyncio integration
- Convert RequestData to Python dict/object
- Support async def handle() in user code
- Return dict or response object
- Handle Python exceptions -> HandlerError
- Use raw_body to avoid double JSON parsing
