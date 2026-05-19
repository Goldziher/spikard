# Spikard

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/wasm">
    <img src="https://img.shields.io/npm/v/@spikard/wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://packagist.org/packages/spikard/spikard">
    <img src="https://img.shields.io/packagist/v/spikard/spikard?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://github.com/Goldziher/spikard/releases">
    <img src="https://img.shields.io/github/v/tag/Goldziher/spikard?label=Go&color=007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/Spikard/">
    <img src="https://img.shields.io/nuget/v/Spikard?label=C%23&color=007ec6" alt="C#">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License">
  </a>
  <a href="https://github.com/Goldziher/spikard">
    <img src="https://img.shields.io/badge/docs-GitHub-007ec6" alt="Documentation">
  </a>
</div>

Rust-centric polyglot HTTP framework with OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. PHP 8.2+ bindings via ext-php-rs.

## Installation


```bash
composer require spikard/spikard
```


### System Requirements


- **PHP 8.2+** required


## Quick Start


```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class UsersController
{
    #[Get('/users')]
    public function list(): Response
    {
        return Response::json([
            'users' => [
                ['id' => 1, 'name' => 'Alice'],
                ['id' => 2, 'name' => 'Bob'],
            ]
        ]);
    }

    #[Get('/users/{id}')]
    public function show(Request $request): Response
    {
        $userId = (int) $request->pathParams['id'];
        return Response::json(['id' => $userId, 'name' => 'Alice']);
    }

    #[Post('/users')]
    public function create(Request $request): Response
    {
        $user = $request->body;
        return Response::json($user, 201);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new UsersController());

$app->run();
```


## Features

- **HTTP routing** — type-safe route definitions with path, query, and body parameter validation
- **OpenAPI / AsyncAPI / GraphQL / JSON-RPC** — code generation and spec parsing built in
- **Tower middleware** — compression, rate limiting, timeouts, auth (JWT/API key), static files
- **Lifecycle hooks** — `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`
- **Fixture-driven testing** — shared JSON fixtures drive tests across all language bindings
- **Polyglot** — single Rust core, thin bindings for Python, Node.js, Ruby, PHP, Elixir, Go, Java, C#, Kotlin, Dart, Gleam, WASM, Swift, Zig, and C FFI


## Routing

```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Attributes\Put;
use Spikard\Attributes\Delete;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class ResourceController
{
    #[Get('/items')]
    public function list(): Response
    {
        return Response::json(['items' => []]);
    }

    #[Post('/items')]
    public function create(Request $request): Response
    {
        return Response::json($request->body, 201);
    }

    #[Put('/items/{id}')]
    public function update(Request $request): Response
    {
        $id = (int) $request->pathParams['id'];
        return Response::json(['id' => $id, 'updated' => true]);
    }

    #[Delete('/items/{id}')]
    public function delete(Request $request): Response
    {
        return Response::json(null, 204);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new ResourceController());
```


## Validation

```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class PaymentsController
{
    #[Post('/payments')]
    public function create(Request $request): Response
    {
        $payment = $request->body;

        // Manual validation
        $errors = [];
        if (!isset($payment['id']) || !is_string($payment['id'])) {
            $errors[] = 'id is required and must be a string';
        }
        if (!isset($payment['amount']) || !is_numeric($payment['amount'])) {
            $errors[] = 'amount is required and must be numeric';
        }
        if (isset($payment['amount']) && $payment['amount'] <= 0) {
            $errors[] = 'amount must be positive';
        }

        if (!empty($errors)) {
            return Response::json(['errors' => $errors], 400);
        }

        return Response::json([
            'id' => $payment['id'],
            'amount' => (float) $payment['amount'],
            'status' => 'pending'
        ], 201);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new PaymentsController());
```


## Documentation

- **[Repository](https://github.com/Goldziher/spikard)** — source code, examples, and contributing guide
- **[Examples](https://github.com/Goldziher/spikard/tree/main/examples)** — working examples per language
- **[Issues](https://github.com/Goldziher/spikard/issues)** — bug reports and feature requests

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](https://github.com/Goldziher/spikard/blob/main/CONTRIBUTING.md).

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.
