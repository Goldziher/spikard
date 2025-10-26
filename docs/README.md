# Spikard Documentation

> A modular HTTP toolkit with Rust performance and high-level language ergonomics

## Overview

Spikard is a high-performance HTTP framework with a **Rust core** and bindings for **Python** and **TypeScript**. It combines the performance of Rust with the developer experience of modern frameworks like Litestar and NestJS.

## Design Documents

### Architecture
- [**00 - Architecture Overview**](design/00-architecture.md) - Core architecture, layers, and design principles
- [**01 - Validation Strategy**](design/01-validation-strategy.md) - Request/response validation using JSON Schema

## Key Features

- 🚀 **Rust-powered performance** - HTTP parsing, routing, and validation in Rust
- 🐍 **Natural Python API** - Decorators, type hints, async/await support
- 📘 **TypeScript-first** - Full type safety with decorators and DI
- ✅ **Existing ecosystem integration** - Pydantic, msgspec, Zod, etc.
- 🔌 **Modular architecture** - Middleware, guards, plugins, DI
- 📊 **OpenAPI generation** - Automatic from type annotations
- ⚡ **Sync + Async** - Both paradigms supported transparently

## Quick Start

### Python

```python
from spikard import Spikard, get, post
from pydantic import BaseModel

class User(BaseModel):
    name: str
    email: str

app = Spikard()

@app.get("/users/{user_id}")
async def get_user(user_id: int) -> User:
    return User(name="Alice", email="alice@example.com")

@app.post("/users")
async def create_user(data: User) -> User:
    # Data is validated by Rust before reaching here
    return data

if __name__ == "__main__":
    app.run()
```

### TypeScript

```typescript
import { Spikard, Controller, Get, Post, Body } from '@spikard/node';
import { z } from 'zod';

const UserSchema = z.object({
    name: z.string(),
    email: z.string().email()
});

@Controller('/users')
class UserController {
    @Get('/:userId')
    async getUser(@Param('userId') userId: number): Promise<User> {
        return { name: 'Alice', email: 'alice@example.com' };
    }

    @Post('/')
    @ValidateBody(UserSchema)
    async createUser(@Body() data: z.infer<typeof UserSchema>): Promise<User> {
        return data;
    }
}

const app = new Spikard({
    controllers: [UserController]
});

app.listen(3000);
```

## Project Status

🚧 **Early Development** - API may change

Current focus:
- Core Rust HTTP server and router
- Python bindings (PyO3)
- Request/response validation
- Basic middleware support

## Design Philosophy

1. **Performance by default** - Critical paths in Rust
2. **Developer experience first** - Natural APIs for each language
3. **Leverage existing tools** - Don't reinvent Pydantic, Zod, etc.
4. **Type safety everywhere** - Full type checking and IDE support
5. **Modular and composable** - Easy to extend and customize

## Contributing

See design documents for architectural decisions and implementation guidelines.

## License

MIT
