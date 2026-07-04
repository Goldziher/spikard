/**
 * Ergonomic-layer smoke server (Node.js / zod).
 *
 * Exercises the ergonomic typed-handler + DTO API end-to-end: a typed handler
 * whose body is a zod schema, hydrated by the ergonomic layer, with request
 * validation delegated to the Rust core (invalid bodies -> 422 ProblemDetails).
 */

import { z } from "zod";
import { createRequire } from "module";

const require = createRequire(import.meta.url);
const { App } = require("../../../crates/spikard-node/app.cjs");

const CreateUser = z.object({
  name: z.string(),
  age: z.number(),
});

const app = new App();

app.post("/users", { body: CreateUser }, async (req) => {
  // The ergonomic layer must hand us the validated, typed request.
  // req.body is already a validated { name: string, age: number }
  console.log("Handler received:", req.body);
  return {
    statusCode: 200,
    content: req.body,
  };
});

// The server binds to 127.0.0.1:8000 by default
app.run().catch((err) => {
  console.error("Server error:", err);
  process.exit(1);
});
