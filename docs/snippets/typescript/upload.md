```typescript
import { Spikard } from "spikard";
import { UploadFile } from "spikard";

const app = new Spikard();

app.addRoute(
  { method: "POST", path: "/upload", handler_name: "upload", is_async: true },
  async (req) => {
    const body = req.json<{ file: UploadFile; description?: string }>();
    const size = body.file.size;
    return { filename: body.file.filename, size };
  },
);
```
