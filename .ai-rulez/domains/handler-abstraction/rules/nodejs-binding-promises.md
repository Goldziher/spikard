---
name: nodejs-binding-promises
priority: high
---
Node.js binding must:
- Use ThreadsafeFunction for async callbacks
- Convert RequestData to JavaScript object
- Support async/await and Promise in user code
- Properly handle Promise rejection
- Serialize responses back to Rust
- Integrate with libuv event loop
