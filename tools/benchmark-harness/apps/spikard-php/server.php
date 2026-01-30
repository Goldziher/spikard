<?php
declare(strict_types=1);

/**
 * Spikard-PHP benchmark server
 *
 * Implements all workload types to measure PHP binding performance.
 * Serves both raw endpoints (no validation) and validated endpoints (at /validated/... paths).
 */

require_once __DIR__ . '/../../../../packages/php/vendor/autoload.php';

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Attributes\SchemaRef;
use Spikard\Config\ServerConfig;
use Spikard\Http\Params\Body;
use Spikard\Http\Params\Path;
use Spikard\Http\Params\Query;
use Spikard\Http\Request;

// Load schemas
$schemaDir = dirname(__DIR__) . '/schemas';
$requestSchemas = json_decode(file_get_contents($schemaDir . '/request_schemas.json'), true);
$parameterSchemas = json_decode(file_get_contents($schemaDir . '/parameter_schemas.json'), true);
$responseSchemas = json_decode(file_get_contents($schemaDir . '/response_schemas.json'), true);

// ===== RAW ENDPOINTS (no validation) =====

class RawJsonController
{
    #[Post('/json/small')]
    public function small(array $data = new Body()): array
    {
        return $data;
    }

    #[Post('/json/medium')]
    public function medium(array $data = new Body()): array
    {
        return $data;
    }

    #[Post('/json/large')]
    public function large(array $data = new Body()): array
    {
        return $data;
    }

    #[Post('/json/very-large')]
    public function veryLarge(array $data = new Body()): array
    {
        return $data;
    }
}

class RawMultipartController
{
    #[Post('/multipart/small')]
    public function small(Request $request): array
    {
        return self::countFiles($request);
    }

    #[Post('/multipart/medium')]
    public function medium(Request $request): array
    {
        return self::countFiles($request);
    }

    #[Post('/multipart/large')]
    public function large(Request $request): array
    {
        return self::countFiles($request);
    }

    private static function countFiles(Request $request): array
    {
        $body = $request->body;
        $filesReceived = 0;
        $totalBytes = 0;

        if (is_array($body) && isset($body['files']) && is_array($body['files'])) {
            foreach ($body['files'] as $key => $fileData) {
                if (str_starts_with((string)$key, 'file') && is_array($fileData)) {
                    $filesReceived++;
                    $totalBytes += (int)($fileData['size'] ?? 0);
                }
            }
        }

        return ['files_received' => $filesReceived, 'total_bytes' => $totalBytes];
    }
}

class RawUrlencodedController
{
    #[Post('/urlencoded/simple')]
    public function simple(array $data = new Body()): array
    {
        return $data;
    }

    #[Post('/urlencoded/complex')]
    public function complex(array $data = new Body()): array
    {
        return $data;
    }
}

class RawPathController
{
    #[Get('/path/simple/{id}')]
    public function simple(string $id = new Path()): array
    {
        return ['id' => $id];
    }

    #[Get('/path/multiple/{user_id}/{post_id}')]
    public function multiple(string $user_id = new Path(), string $post_id = new Path()): array
    {
        return ['user_id' => $user_id, 'post_id' => $post_id];
    }

    #[Get('/path/deep/{org}/{team}/{project}/{resource}/{id}')]
    public function deep(
        string $org = new Path(),
        string $team = new Path(),
        string $project = new Path(),
        string $resource = new Path(),
        string $id = new Path(),
    ): array {
        return [
            'org' => $org,
            'team' => $team,
            'project' => $project,
            'resource' => $resource,
            'id' => $id,
        ];
    }

    #[Get('/path/int/{id}')]
    public function intParam(int $id = new Path()): array
    {
        return ['id' => $id];
    }

    #[Get('/path/uuid/{uuid}')]
    public function uuid(string $uuid = new Path()): array
    {
        return ['uuid' => $uuid];
    }

    #[Get('/path/date/{date}')]
    public function date(string $date = new Path()): array
    {
        return ['date' => $date];
    }
}

class RawQueryController
{
    #[Get('/query/few')]
    public function few(
        ?string $q = new Query(default: null),
        ?int $page = new Query(default: null),
        ?int $limit = new Query(default: null),
    ): array {
        $result = [];
        if ($q !== null) { $result['q'] = $q; }
        if ($page !== null) { $result['page'] = $page; }
        if ($limit !== null) { $result['limit'] = $limit; }
        return $result;
    }

    #[Get('/query/medium')]
    public function medium(
        ?string $search = new Query(default: null),
        ?string $category = new Query(default: null),
        ?string $sort = new Query(default: null),
        ?string $order = new Query(default: null),
        ?int $page = new Query(default: null),
        ?int $limit = new Query(default: null),
        ?string $filter = new Query(default: null),
    ): array {
        $result = [];
        if ($search !== null) { $result['search'] = $search; }
        if ($category !== null) { $result['category'] = $category; }
        if ($sort !== null) { $result['sort'] = $sort; }
        if ($order !== null) { $result['order'] = $order; }
        if ($page !== null) { $result['page'] = $page; }
        if ($limit !== null) { $result['limit'] = $limit; }
        if ($filter !== null) { $result['filter'] = $filter; }
        return $result;
    }

    #[Get('/query/many')]
    public function many(
        ?string $q = new Query(default: null),
        ?string $category = new Query(default: null),
        ?string $subcategory = new Query(default: null),
        ?string $brand = new Query(default: null),
        ?float $min_price = new Query(default: null),
        ?float $max_price = new Query(default: null),
        ?string $color = new Query(default: null),
        ?string $size = new Query(default: null),
        ?string $material = new Query(default: null),
        ?int $rating = new Query(default: null),
        ?string $sort = new Query(default: null),
        ?string $order = new Query(default: null),
        ?int $page = new Query(default: null),
        ?int $limit = new Query(default: null),
        ?bool $in_stock = new Query(default: null),
        ?bool $on_sale = new Query(default: null),
    ): array {
        $result = [];
        if ($q !== null) { $result['q'] = $q; }
        if ($category !== null) { $result['category'] = $category; }
        if ($subcategory !== null) { $result['subcategory'] = $subcategory; }
        if ($brand !== null) { $result['brand'] = $brand; }
        if ($min_price !== null) { $result['min_price'] = $min_price; }
        if ($max_price !== null) { $result['max_price'] = $max_price; }
        if ($color !== null) { $result['color'] = $color; }
        if ($size !== null) { $result['size'] = $size; }
        if ($material !== null) { $result['material'] = $material; }
        if ($rating !== null) { $result['rating'] = $rating; }
        if ($sort !== null) { $result['sort'] = $sort; }
        if ($order !== null) { $result['order'] = $order; }
        if ($page !== null) { $result['page'] = $page; }
        if ($limit !== null) { $result['limit'] = $limit; }
        if ($in_stock !== null) { $result['in_stock'] = $in_stock; }
        if ($on_sale !== null) { $result['on_sale'] = $on_sale; }
        return $result;
    }
}

class RawHealthController
{
    #[Get('/health')]
    public function health(): array
    {
        return ['status' => 'ok'];
    }

    #[Get('/')]
    public function root(): array
    {
        return ['status' => 'ok'];
    }
}

// ===== VALIDATED ENDPOINTS (with schemas at /validated/... paths) =====

class ValidatedJsonController
{
    #[Post('/validated/json/small')]
    #[SchemaRef(request: 'json/small', response: 'json/small')]
    public function small(array $data = new Body()): array
    {
        return $data;
    }

    #[Post('/validated/json/medium')]
    #[SchemaRef(request: 'json/medium', response: 'json/medium')]
    public function medium(array $data = new Body()): array
    {
        return $data;
    }

    #[Post('/validated/json/large')]
    #[SchemaRef(request: 'json/large', response: 'json/large')]
    public function large(array $data = new Body()): array
    {
        return $data;
    }

    #[Post('/validated/json/very-large')]
    #[SchemaRef(request: 'json/very-large', response: 'json/very-large')]
    public function veryLarge(array $data = new Body()): array
    {
        return $data;
    }
}

class ValidatedMultipartController
{
    #[Post('/validated/multipart/small')]
    #[SchemaRef(request: 'multipart/small', response: 'multipart/small')]
    public function small(Request $request): array
    {
        return self::countFiles($request);
    }

    #[Post('/validated/multipart/medium')]
    #[SchemaRef(request: 'multipart/medium', response: 'multipart/medium')]
    public function medium(Request $request): array
    {
        return self::countFiles($request);
    }

    #[Post('/validated/multipart/large')]
    #[SchemaRef(request: 'multipart/large', response: 'multipart/large')]
    public function large(Request $request): array
    {
        return self::countFiles($request);
    }

    private static function countFiles(Request $request): array
    {
        $body = $request->body;
        $filesReceived = 0;
        $totalBytes = 0;

        if (is_array($body) && isset($body['files']) && is_array($body['files'])) {
            foreach ($body['files'] as $key => $fileData) {
                if (str_starts_with((string)$key, 'file') && is_array($fileData)) {
                    $filesReceived++;
                    $totalBytes += (int)($fileData['size'] ?? 0);
                }
            }
        }

        return ['files_received' => $filesReceived, 'total_bytes' => $totalBytes];
    }
}

class ValidatedUrlencodedController
{
    #[Post('/validated/urlencoded/simple')]
    #[SchemaRef(request: 'urlencoded/simple', response: 'urlencoded/simple')]
    public function simple(array $data = new Body()): array
    {
        return $data;
    }

    #[Post('/validated/urlencoded/complex')]
    #[SchemaRef(request: 'urlencoded/complex', response: 'urlencoded/complex')]
    public function complex(array $data = new Body()): array
    {
        return $data;
    }
}

class ValidatedPathController
{
    #[Get('/validated/path/simple/{id}')]
    #[SchemaRef(response: 'path/simple', parameters: 'path/simple')]
    public function simple(string $id = new Path()): array
    {
        return ['id' => $id];
    }

    #[Get('/validated/path/multiple/{user_id}/{post_id}')]
    #[SchemaRef(response: 'path/multiple', parameters: 'path/multiple')]
    public function multiple(string $user_id = new Path(), string $post_id = new Path()): array
    {
        return ['user_id' => $user_id, 'post_id' => $post_id];
    }

    #[Get('/validated/path/deep/{org}/{team}/{project}/{resource}/{id}')]
    #[SchemaRef(response: 'path/deep', parameters: 'path/deep')]
    public function deep(
        string $org = new Path(),
        string $team = new Path(),
        string $project = new Path(),
        string $resource = new Path(),
        string $id = new Path(),
    ): array {
        return [
            'org' => $org,
            'team' => $team,
            'project' => $project,
            'resource' => $resource,
            'id' => $id,
        ];
    }

    #[Get('/validated/path/int/{id}')]
    #[SchemaRef(response: 'path/int', parameters: 'path/int')]
    public function intParam(int $id = new Path()): array
    {
        return ['id' => $id];
    }

    #[Get('/validated/path/uuid/{uuid}')]
    #[SchemaRef(response: 'path/uuid', parameters: 'path/uuid')]
    public function uuid(string $uuid = new Path()): array
    {
        return ['uuid' => $uuid];
    }

    #[Get('/validated/path/date/{date}')]
    #[SchemaRef(response: 'path/date', parameters: 'path/date')]
    public function date(string $date = new Path()): array
    {
        return ['date' => $date];
    }
}

class ValidatedQueryController
{
    #[Get('/validated/query/few')]
    #[SchemaRef(response: 'query/few', parameters: 'query/few')]
    public function few(
        ?string $q = new Query(default: null),
        ?int $page = new Query(default: null),
        ?int $limit = new Query(default: null),
    ): array {
        $result = [];
        if ($q !== null) { $result['q'] = $q; }
        if ($page !== null) { $result['page'] = $page; }
        if ($limit !== null) { $result['limit'] = $limit; }
        return $result;
    }

    #[Get('/validated/query/medium')]
    #[SchemaRef(response: 'query/medium', parameters: 'query/medium')]
    public function medium(
        ?string $search = new Query(default: null),
        ?string $category = new Query(default: null),
        ?string $sort = new Query(default: null),
        ?string $order = new Query(default: null),
        ?int $page = new Query(default: null),
        ?int $limit = new Query(default: null),
        ?string $filter = new Query(default: null),
    ): array {
        $result = [];
        if ($search !== null) { $result['search'] = $search; }
        if ($category !== null) { $result['category'] = $category; }
        if ($sort !== null) { $result['sort'] = $sort; }
        if ($order !== null) { $result['order'] = $order; }
        if ($page !== null) { $result['page'] = $page; }
        if ($limit !== null) { $result['limit'] = $limit; }
        if ($filter !== null) { $result['filter'] = $filter; }
        return $result;
    }

    #[Get('/validated/query/many')]
    #[SchemaRef(response: 'query/many', parameters: 'query/many')]
    public function many(
        ?string $q = new Query(default: null),
        ?string $category = new Query(default: null),
        ?string $subcategory = new Query(default: null),
        ?string $brand = new Query(default: null),
        ?float $min_price = new Query(default: null),
        ?float $max_price = new Query(default: null),
        ?string $color = new Query(default: null),
        ?string $size = new Query(default: null),
        ?string $material = new Query(default: null),
        ?int $rating = new Query(default: null),
        ?string $sort = new Query(default: null),
        ?string $order = new Query(default: null),
        ?int $page = new Query(default: null),
        ?int $limit = new Query(default: null),
        ?bool $in_stock = new Query(default: null),
        ?bool $on_sale = new Query(default: null),
    ): array {
        $result = [];
        if ($q !== null) { $result['q'] = $q; }
        if ($category !== null) { $result['category'] = $category; }
        if ($subcategory !== null) { $result['subcategory'] = $subcategory; }
        if ($brand !== null) { $result['brand'] = $brand; }
        if ($min_price !== null) { $result['min_price'] = $min_price; }
        if ($max_price !== null) { $result['max_price'] = $max_price; }
        if ($color !== null) { $result['color'] = $color; }
        if ($size !== null) { $result['size'] = $size; }
        if ($material !== null) { $result['material'] = $material; }
        if ($rating !== null) { $result['rating'] = $rating; }
        if ($sort !== null) { $result['sort'] = $sort; }
        if ($order !== null) { $result['order'] = $order; }
        if ($page !== null) { $result['page'] = $page; }
        if ($limit !== null) { $result['limit'] = $limit; }
        if ($in_stock !== null) { $result['in_stock'] = $in_stock; }
        if ($on_sale !== null) { $result['on_sale'] = $on_sale; }
        return $result;
    }
}

class ValidatedHealthController
{
    #[Get('/validated/health')]
    #[SchemaRef(response: 'health')]
    public function health(): array
    {
        return ['status' => 'ok'];
    }

    #[Get('/validated/')]
    #[SchemaRef(response: 'root')]
    public function root(): array
    {
        return ['status' => 'ok'];
    }
}

// ===== Bootstrap =====

if (PHP_SAPI === 'cli' && __FILE__ === realpath($_SERVER['SCRIPT_FILENAME'])) {
    $port = (int)($argv[1] ?? 8000);

    $app = (new App())
        ->withSchemas($requestSchemas, $responseSchemas, $parameterSchemas);

    // Register raw controllers
    $app = $app->registerController(new RawJsonController());
    $app = $app->registerController(new RawMultipartController());
    $app = $app->registerController(new RawUrlencodedController());
    $app = $app->registerController(new RawPathController());
    $app = $app->registerController(new RawQueryController());
    $app = $app->registerController(new RawHealthController());

    // Register validated controllers
    $app = $app->registerController(new ValidatedJsonController());
    $app = $app->registerController(new ValidatedMultipartController());
    $app = $app->registerController(new ValidatedUrlencodedController());
    $app = $app->registerController(new ValidatedPathController());
    $app = $app->registerController(new ValidatedQueryController());
    $app = $app->registerController(new ValidatedHealthController());

    $config = new ServerConfig(
        host: '0.0.0.0',
        port: $port,
        workers: 1,
    );

    error_log("Starting Spikard-PHP benchmark server on port $port");
    $app->run($config);
}
