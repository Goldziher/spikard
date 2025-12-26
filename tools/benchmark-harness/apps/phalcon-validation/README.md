# Phalcon with Validation

Benchmark application using Phalcon PHP framework with native `Phalcon\Validation` for request validation.

## Framework

- **Phalcon 5.9+**: High-performance PHP framework
- **PHP 8.2+**: Modern PHP with strict types

## Validation Strategy

This app uses **Phalcon's native validation system** (`Phalcon\Validation`) to validate incoming JSON payloads. This is a framework-native feature documented in the official Phalcon documentation.

### Validation Features

- `Phalcon\Validation` - Core validation component
- `Phalcon\Validation\Validator\PresenceOf` - Required field validation
- `Phalcon\Validation\Validator\Numericality` - Numeric type validation
- Nested object validation for complex payloads
- Array validation for collections

### Validated Routes

#### JSON Body Routes
- `POST /json/small` - Validates: name, description, price, tax (optional)
- `POST /json/medium` - Validates: name, price, image{url, name}
- `POST /json/large` - Validates: name, price, seller{name, address{street, city, country{name, code}}}
- `POST /json/very-large` - Validates: name, tags[], images[]{url, name}

### Non-Validated Routes

The following routes echo back data without validation (standard benchmark routes):
- Multipart form routes (`/multipart/*`)
- URL-encoded routes (`/urlencoded/*`)
- Path parameter routes (`/path/*`)
- Query parameter routes (`/query/*`)

## Installation

```bash
composer install
```

## Running

```bash
php -S 127.0.0.1:8000 server.php
```

## Error Handling

Invalid payloads return HTTP 400 with error details:

```json
{
  "errors": [
    "name is required",
    "price must be numeric"
  ]
}
```

## Comparison with phalcon-raw

- **phalcon-validation** (this app): Uses `Phalcon\Validation` for JSON body validation
- **phalcon-raw**: No validation, direct JSON parsing for maximum performance

This demonstrates the performance impact of Phalcon's native validation system.
