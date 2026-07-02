<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

<img src="https://raw.githubusercontent.com/Goldziher/spikard/main/docs/assets/spikard-banner.svg" alt="spikard - polyglot web toolkit" width="820">

**Spikard** — Part of the spikard polyglot web toolkit.

Rust-centric polyglot HTTP framework with OpenAPI/AsyncAPI/GraphQL/JSON-RPC codegen, tower-http middleware, and fixture-driven cross-language testing. PHP 8.2+ bindings via ext-php-rs.

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Built with alef -->
  <a href="https://github.com/xberg-io/alef">
    <img src="https://img.shields.io/badge/built%20with-alef%20%D7%90-007ec6?style=flat-square" alt="Built with alef">
  </a>

  <!-- Language bindings -->
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard?color=007ec6&style=flat-square" alt="Rust">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard?color=007ec6&style=flat-square" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node?color=007ec6&style=flat-square" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node-wasm">
    <img src="https://img.shields.io/npm/v/@spikard/node-wasm?color=007ec6&style=flat-square" alt="WASM">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard?color=007ec6&style=flat-square" alt="Ruby">
  </a>
  <a href="https://packagist.org/packages/goldziher/spikard">
    <img src="https://img.shields.io/packagist/v/goldziher/spikard?color=007ec6&style=flat-square" alt="PHP">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard?color=007ec6&style=flat-square" alt="Elixir">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?color=007ec6&style=flat-square" alt="Java">
  </a>
  <a href="https://github.com/Goldziher/spikard/releases">
    <img src="https://img.shields.io/github/v/tag/Goldziher/spikard?label=Go&color=007ec6&style=flat-square" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/Spikard/">
    <img src="https://img.shields.io/nuget/v/Spikard?color=007ec6&style=flat-square" alt="C#">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.spikard/spikard">
    <img src="https://img.shields.io/maven-central/v/dev.spikard/spikard?label=Kotlin&color=007ec6&style=flat-square" alt="Kotlin">
  </a>
  <a href="https://pub.dev/packages/spikard">
    <img src="https://img.shields.io/pub/v/spikard?color=007ec6&style=flat-square" alt="Dart">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/packages/swift">
    <img src="https://img.shields.io/badge/Swift-Spikard-007ec6?style=flat-square" alt="Swift">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/packages/zig">
    <img src="https://img.shields.io/badge/Zig-spikard-007ec6?style=flat-square" alt="Zig">
  </a>
  <a href="https://github.com/Goldziher/spikard/tree/main/crates/spikard-ffi">
    <img src="https://img.shields.io/badge/C%20FFI-007ec6?style=flat-square" alt="C FFI">
  </a>
  <a href="https://github.com/Goldziher/homebrew-tap">
    <img src="https://img.shields.io/badge/Homebrew-007ec6?style=flat-square&logo=homebrew&logoColor=white" alt="Homebrew">
  </a>

  <!-- Project info -->
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6?style=flat-square" alt="License">
  </a>
</div>

[Install](#installation) · [Quick example](#quick-example) · [Features](#features) · [Docs](https://github.com/Goldziher/spikard)

</div>

---

## What this package provides

- **PHP 8.2+ extension** — native ext-php-rs bindings for the shared HTTP core

- **Type-safe routing** — HTTP definitions with path, query, body, and header validation across all bindings
- **Spec-driven codegen** — OpenAPI 3.0, AsyncAPI 3.0, GraphQL SDL, and JSON-RPC 2.0 support
- **Cross-language parity** — same DTOs, fixtures, and error model prevent runtime drift
- **Tower middleware** — compression, rate limiting, timeouts, auth (JWT/API key), static files
- **Lifecycle hooks** — `onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`

## Installation

```bash
composer require goldziher/spikard
```

### System Requirements

- **PHP 8.2+** required

## Quick example

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

| Feature | Support |
|---|---|
| **Type-safe routing** | Path, query, body, and header parameter validation |
| **Request extraction** | Typed structs for JSON, form data, multipart, and raw bodies |
| **Spec support** | OpenAPI 3.0 · AsyncAPI 3.0 · GraphQL SDL · JSON-RPC 2.0 |
| **Middleware** | Compression, rate limiting, timeouts, authentication, static files |
| **Lifecycle hooks** | Request, pre-validation, pre-handler, response, and error hooks |
| **WebSocket & SSE** | Bidirectional streams and server-sent events |
| **Error handling** | Consistent error responses across all bindings via ProblemDetails |
| **Fixture testing** | Shared JSON fixtures for behavioral consistency across languages |

<details>
<summary><strong>Routing</strong></summary>

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

</details>

<details>
<summary><strong>Validation</strong></summary>

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

</details>

<details>
<summary><strong>Middleware & configuration</strong></summary>

See [examples](https://github.com/Goldziher/spikard/tree/main/crates/spikard-http/examples) in the repository.

</details>

## Resources

- **[Repository](https://github.com/Goldziher/spikard)** — source code, examples, and issues
- **[Examples](https://github.com/Goldziher/spikard/tree/main/crates/spikard-http/examples)** — working implementations in all supported languages
- **[Contributing](https://github.com/Goldziher/spikard/blob/main/CONTRIBUTING.md)** — how to contribute

## License

MIT License — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE) for details.
