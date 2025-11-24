```python
from spikard import Spikard, UploadFile

app = Spikard()

@app.post("/upload")
async def upload(file: UploadFile) -> dict:
    content = file.read()
    return {"filename": file.filename, "size": len(content)}
```
