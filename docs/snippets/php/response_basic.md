```php
<?php

declare(strict_types=1);

use Spikard\Http\Response;

// JSON response
return Response::json(['status' => 'ok']);

// JSON with status code
return Response::json(['error' => 'not found'], 404);

// Plain text response
return Response::text('Hello, World!');

// Response with custom headers
return new Response(
    body: ['data' => 'value'],
    statusCode: 200,
    headers: ['X-Custom-Header' => 'value']
);
```
