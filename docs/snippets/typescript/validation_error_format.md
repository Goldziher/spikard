```typescript
import { ZodError } from "zod";

app.setErrorHandler((error, req, res) => {
  if (error instanceof ZodError) {
    return res.status(422).json({
      error: "validation_failed",
      message: "Request validation failed",
      details: error.errors.map(err => ({
        field: err.path.join("."),
        message: err.message,
        type: err.code
      }))
    });
  }

  // Handle other errors
  return res.status(500).json({
    error: "internal_error",
    message: error.message
  });
});
```
