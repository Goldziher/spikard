---
id: grpc_php_registration
language: php
title: Registration
tags:
  - grpc
  - php
---

```php
<?php declare(strict_types=1);

use Spikard\Grpc;

// Create service registry
$service = Grpc::createService();

// Register handler
$handler = new UserServiceHandler(
    userRepository: new UserRepository()
);

$service->registerHandler('userservice.UserService', $handler);

// Service ready to handle requests
```
