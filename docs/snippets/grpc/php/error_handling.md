---
id: grpc_php_error_handling
language: php
title: Error Handling
tags:
  - grpc
  - php
---

```php
<?php declare(strict_types=1);

// Return error response
return Response::error('Error message');

// With status code in metadata
return Response::error(
    'Error message',
    'INVALID_ARGUMENT'
);

// Try-catch pattern
try {
    // Handler logic
} catch (\InvalidArgumentException $e) {
    return Response::error($e->getMessage());
} catch (\Exception $e) {
    return Response::error("Internal error: {$e->getMessage()}");
}
```

Return error responses instead of throwing.
