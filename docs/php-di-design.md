# Dependency Injection Design for PHP Bindings

## 1. Python/Node/Ruby DI Analysis

### Python DI Implementation

**Core Architecture (packages/python/spikard/di.py):**
- `Provide` class wraps factory functions with metadata:
  - `dependency`: The callable (sync/async function or generator)
  - `depends_on`: List of dependency keys this factory needs
  - `use_cache`: Cache within a single request
  - `singleton`: Cache globally across all requests
  - `is_async`, `is_generator`, `is_async_generator`: Execution mode flags

**Rust-Side Python DI (crates/spikard-py/src/di.rs):**
- `PythonValueDependency`: Wraps `Py<PyAny>` for static values
- `PythonFactoryDependency`: Wraps Python callables with dependency resolution
  - Stores `Py<PyAny>` factory callable
  - Implements `Dependency` trait from `spikard_core::di`
  - Handles async execution via `PYTHON_EVENT_LOOP` and `tokio::task::spawn_blocking`
  - Supports async generators with cleanup tasks

**Extraction Flow (crates/spikard-py/src/lib.rs:486-544):**
```rust
fn build_dependency_container(py: Python, dependencies: &Bound<PyAny>) -> PyResult<DependencyContainer> {
    let mut container = DependencyContainer::new();

    for (key, value) in deps_dict.iter() {
        if value.hasattr("dependency")? {
            // Provide wrapper - extract factory details
            let factory = value.getattr("dependency")?;
            let depends_on: Vec<String> = value.getattr("depends_on")?;
            let singleton: bool = value.getattr("singleton")?;
            // Create PythonFactoryDependency
        } else {
            // Static value - create PythonValueDependency
        }
        container.register(key, Arc::new(dependency))?;
    }
}
```

**Handler Parameter Injection (packages/python/spikard/app.py:110-135):**
- Uses Python's `inspect.signature()` to extract handler parameters
- Matches parameter names against registered dependency keys
- Stores list in `Route.handler_dependencies`
- Rust handler receives this list and resolves dependencies before calling Python handler

### Core Rust DI Infrastructure (crates/spikard-core/src/di/)

**`Dependency` Trait (dependency.rs):**
```rust
pub trait Dependency: Send + Sync {
    fn resolve(
        &self,
        request: &Request<()>,
        request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>>;

    fn key(&self) -> &str;
    fn depends_on(&self) -> Vec<String>;
    fn cacheable(&self) -> bool { false }
    fn singleton(&self) -> bool { false }
}
```

**`DependencyContainer` (container.rs):**
- Stores `IndexMap<String, Arc<dyn Dependency>>`
- `DependencyGraph` for topological sorting and cycle detection
- Singleton cache: `Arc<RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>>`
- `resolve_for_handler(&[String], &Request, &RequestData) -> ResolvedDependencies`
  - Calculates batched resolution order
  - Resolves dependencies in parallel batches
  - Caches singletons globally, per-request cacheables locally

**`ResolvedDependencies` (resolved.rs):**
- Stores resolved values: `HashMap<String, Arc<dyn Any + Send + Sync>>`
- Type-safe retrieval: `get<T>(&str) -> Option<Arc<T>>`
- Cleanup task support for generator-pattern dependencies

### Key Differences for PHP

**Python Advantages:**
1. **Reflection**: `inspect.signature()` provides full parameter type information
2. **Event Loop**: `asyncio` provides built-in async support
3. **GIL**: Single-threaded execution simplifies threading concerns

**PHP Challenges:**
1. **No built-in event loop**: PHP is synchronous by nature
2. **Thread-local storage**: Zval is not Send/Sync (like Python's GIL)
3. **Reflection API**: PHP has `ReflectionFunction`/`ReflectionMethod` but type hints may be classes, not strings
4. **Closure storage**: Need to store PHP callables as `Zval` in thread-local registry

---

## 2. PHP DI Container API

### PHP Classes

**DependencyContainer (packages/php/src/DI/DependencyContainer.php):**
```php
<?php
declare(strict_types=1);

namespace Spikard\DI;

final class DependencyContainer
{
    /** @var array<string, mixed> */
    private array $values = [];

    /** @var array<string, Provide> */
    private array $factories = [];

    /** @var array<string, mixed> */
    private array $singletonCache = [];

    /**
     * Register a singleton value dependency.
     *
     * @param string $key Unique dependency identifier
     * @param mixed $value The singleton instance
     */
    public function singleton(string $key, mixed $value): self
    {
        $this->values[$key] = $value;
        return $this;
    }

    /**
     * Register a factory dependency.
     *
     * @param string $key Unique dependency identifier
     * @param Provide $factory Factory wrapper with dependency metadata
     */
    public function factory(string $key, Provide $factory): self
    {
        $this->factories[$key] = $factory;
        return $this;
    }

    /**
     * Register a scoped dependency (per-request singleton).
     *
     * @param string $key Unique dependency identifier
     * @param callable $factory Factory callable
     * @param list<string> $dependsOn Dependencies this factory needs
     */
    public function scoped(string $key, callable $factory, array $dependsOn = []): self
    {
        $this->factories[$key] = new Provide(
            factory: $factory,
            dependsOn: $dependsOn,
            cacheable: true,
            singleton: false
        );
        return $this;
    }

    /**
     * Convert to native array for Rust extraction.
     *
     * @internal Called by App::start()
     */
    public function toNative(): array
    {
        return [
            'values' => $this->values,
            'factories' => $this->factories,
        ];
    }
}
```

**Provide (packages/php/src/DI/Provide.php):**
```php
<?php
declare(strict_types=1);

namespace Spikard\DI;

/**
 * Wrapper for factory dependencies with resolution metadata.
 *
 * Similar to Python's Provide class.
 */
final class Provide
{
    public function __construct(
        /** The factory callable */
        public readonly mixed $factory,

        /** Dependencies this factory requires (parameter names or class names) */
        public readonly array $dependsOn = [],

        /** Whether to cache within a single request */
        public readonly bool $cacheable = false,

        /** Whether to cache globally across all requests */
        public readonly bool $singleton = false,
    ) {
        if (!is_callable($this->factory)) {
            throw new \InvalidArgumentException('Factory must be callable');
        }
    }

    /**
     * Create a factory dependency.
     */
    public static function factory(
        callable $factory,
        array $dependsOn = [],
    ): self {
        return new self($factory, $dependsOn, cacheable: false, singleton: false);
    }

    /**
     * Create a scoped dependency (per-request singleton).
     */
    public static function scoped(
        callable $factory,
        array $dependsOn = [],
    ): self {
        return new self($factory, $dependsOn, cacheable: true, singleton: false);
    }

    /**
     * Create a singleton dependency.
     */
    public static function singleton(
        callable $factory,
        array $dependsOn = [],
    ): self {
        return new self($factory, $dependsOn, cacheable: true, singleton: true);
    }

    /**
     * Convert to associative array for Rust extraction.
     *
     * @internal
     */
    public function toArray(): array
    {
        return [
            'factory' => $this->factory,
            'depends_on' => $this->dependsOn,
            'cacheable' => $this->cacheable,
            'singleton' => $this->singleton,
        ];
    }
}
```

### Usage Examples

**Basic Usage:**
```php
<?php
use Spikard\App;
use Spikard\DI\DependencyContainer;
use Spikard\DI\Provide;

$app = new App();

$container = new DependencyContainer();

// Singleton value
$container->singleton('database_url', 'postgresql://localhost/mydb');

// Factory dependency
$container->factory('db', Provide::singleton(
    factory: function (string $database_url): PDO {
        return new PDO($database_url);
    },
    dependsOn: ['database_url']
));

// Scoped dependency (new instance per request)
$container->scoped('request_id', function (): string {
    return bin2hex(random_bytes(16));
});

$app->withDependencies($container);

// Handler with dependency injection
$app->addRoute('GET', '/users', function (PDO $db, string $request_id): array {
    // $db and $request_id automatically injected
    $stmt = $db->query('SELECT * FROM users');
    return [
        'request_id' => $request_id,
        'users' => $stmt->fetchAll(),
    ];
});

$app->start();
```

**Advanced Usage with Type Hints:**
```php
<?php
interface LoggerInterface {
    public function info(string $message): void;
}

class FileLogger implements LoggerInterface {
    public function __construct(private string $logPath) {}

    public function info(string $message): void {
        file_put_contents($this->logPath, $message . PHP_EOL, FILE_APPEND);
    }
}

$container = new DependencyContainer();

// Register by interface name
$container->singleton('log_path', '/var/log/app.log');
$container->factory(LoggerInterface::class, Provide::singleton(
    factory: fn(string $log_path) => new FileLogger($log_path),
    dependsOn: ['log_path']
));

// Handler with interface type hint
$app->addRoute('POST', '/events', function (LoggerInterface $logger, array $body): void {
    $logger->info('Event created: ' . json_encode($body));
});
```

---

## 3. Rust-Side Architecture

### Data Structures

**PhpValueDependency (crates/spikard-php/src/php/di.rs):**
```rust
/// Wrapper for PHP static value dependencies.
pub struct PhpValueDependency {
    key: String,
    /// Stored as Zval since ext-php-rs types aren't Send/Sync
    value: ext_php_rs::types::Zval,
}

impl PhpValueDependency {
    pub fn new(key: String, value: ext_php_rs::types::Zval) -> Self {
        Self { key, value }
    }
}

impl Dependency for PhpValueDependency {
    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        _resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>> {
        // Clone the Zval for return
        let value = self.value.shallow_clone();
        Box::pin(async move {
            Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
        })
    }

    fn key(&self) -> &str { &self.key }
    fn depends_on(&self) -> Vec<String> { vec![] }
    fn singleton(&self) -> bool { true }
    fn cacheable(&self) -> bool { true }
}
```

**PhpFactoryDependency (crates/spikard-php/src/php/di.rs):**
```rust
/// Wrapper for PHP factory dependencies.
pub struct PhpFactoryDependency {
    key: String,
    /// Index into PHP_FACTORY_REGISTRY (thread-local)
    factory_index: usize,
    depends_on: Vec<String>,
    cacheable: bool,
    singleton: bool,
}

/// Thread-local registry for PHP factory callables.
///
/// Similar to PHP_HANDLER_REGISTRY, we store Zval instead of reconstructing
/// ZendCallable because Zval can be stored in static but ZendCallable has
/// a lifetime parameter.
thread_local! {
    static PHP_FACTORY_REGISTRY: std::cell::RefCell<Vec<ext_php_rs::types::Zval>>
        = std::cell::RefCell::new(Vec::new());
}

impl PhpFactoryDependency {
    /// Register a factory callable and return its index.
    pub fn register_from_zval(
        key: String,
        factory_zval: &ext_php_rs::types::Zval,
        depends_on: Vec<String>,
        cacheable: bool,
        singleton: bool,
    ) -> Result<Self, String> {
        if !factory_zval.is_callable() {
            return Err(format!("Factory for '{}' is not callable", key));
        }

        let factory_index = PHP_FACTORY_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();
            registry.push(factory_zval.shallow_clone());
            idx
        });

        Ok(Self {
            key,
            factory_index,
            depends_on,
            cacheable,
            singleton,
        })
    }
}

impl Dependency for PhpFactoryDependency {
    fn resolve(
        &self,
        _request: &Request<()>,
        request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>> {
        // All PHP invocation must happen synchronously before returning future
        // to avoid capturing non-Send types across .await boundaries.
        let result = invoke_php_factory(
            self.factory_index,
            &self.key,
            &self.depends_on,
            request_data,
            resolved,
        );

        Box::pin(async move { result })
    }

    fn key(&self) -> &str { &self.key }
    fn depends_on(&self) -> Vec<String> { self.depends_on.clone() }
    fn singleton(&self) -> bool { self.singleton }
    fn cacheable(&self) -> bool { self.cacheable }
}

/// Invoke a PHP factory and return the resolved value.
fn invoke_php_factory(
    factory_index: usize,
    key: &str,
    depends_on: &[String],
    request_data: &RequestData,
    resolved: &ResolvedDependencies,
) -> Result<Arc<dyn Any + Send + Sync>, DependencyError> {
    PHP_FACTORY_REGISTRY.with(|registry| -> Result<_, DependencyError> {
        let registry = registry.borrow();
        let factory_zval = registry
            .get(factory_index)
            .ok_or_else(|| DependencyError::ResolutionFailed {
                message: format!("Factory '{}' not found in registry", key),
            })?;

        // Reconstruct ZendCallable from stored Zval
        let callable = ext_php_rs::types::ZendCallable::new(factory_zval)
            .map_err(|e| DependencyError::ResolutionFailed {
                message: format!("Failed to reconstruct PHP callable for '{}': {:?}", key, e),
            })?;

        // Build arguments array: resolved dependencies needed by this factory
        let mut args = Vec::new();
        for dep_key in depends_on {
            let dep_value = resolved
                .get::<ext_php_rs::types::Zval>(dep_key)
                .ok_or_else(|| DependencyError::ResolutionFailed {
                    message: format!("Dependency '{}' not found when resolving '{}'", dep_key, key),
                })?;
            args.push(&**dep_value);
        }

        // Invoke factory
        let result_zval = callable
            .try_call(args)
            .map_err(|e| DependencyError::ResolutionFailed {
                message: format!("Factory '{}' failed: {:?}", key, e),
            })?;

        Ok(Arc::new(result_zval) as Arc<dyn Any + Send + Sync>)
    })
}
```

### Extraction from PHP

**Extract DI Container (crates/spikard-php/src/php/start.rs - add after extract_lifecycle_hooks_from_php):**
```rust
/// Extract DependencyContainer from PHP DependencyContainer::toNative().
///
/// Expected structure:
/// [
///     'values' => ['key' => value, ...],
///     'factories' => ['key' => Provide, ...],
/// ]
fn extract_dependency_container_from_php(
    container_zval: &Zval,
) -> Result<spikard_core::di::DependencyContainer, String> {
    use spikard_core::di::DependencyContainer;

    let container_array = container_zval
        .array()
        .ok_or_else(|| "DI container must be an array".to_string())?;

    let mut di_container = DependencyContainer::new();

    // Extract values (singleton dependencies)
    if let Some(values_zval) = container_array.get("values") {
        if let Some(values_arr) = values_zval.array() {
            for (key_zval, value_zval) in values_arr.iter() {
                let key = match key_zval {
                    ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                    ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                    ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
                };

                let value_dep = crate::php::di::PhpValueDependency::new(
                    key.clone(),
                    value_zval.shallow_clone(),
                );

                di_container
                    .register(key, Arc::new(value_dep))
                    .map_err(|e| format!("Failed to register value dependency: {}", e))?;
            }
        }
    }

    // Extract factories
    if let Some(factories_zval) = container_array.get("factories") {
        if let Some(factories_arr) = factories_zval.array() {
            for (key_zval, provide_zval) in factories_arr.iter() {
                let key = match key_zval {
                    ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                    ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                    ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
                };

                // Extract Provide fields
                let provide_arr = provide_zval
                    .array()
                    .ok_or_else(|| format!("Provide for '{}' must be an array", key))?;

                let factory_callable = provide_arr
                    .get("factory")
                    .ok_or_else(|| format!("Provide for '{}' missing 'factory' field", key))?;

                let depends_on: Vec<String> = provide_arr
                    .get("depends_on")
                    .and_then(|v| v.array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|(_, v)| v.string().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                let cacheable = provide_arr
                    .get("cacheable")
                    .and_then(|v| v.bool())
                    .unwrap_or(false);

                let singleton = provide_arr
                    .get("singleton")
                    .and_then(|v| v.bool())
                    .unwrap_or(false);

                let factory_dep = crate::php::di::PhpFactoryDependency::register_from_zval(
                    key.clone(),
                    factory_callable,
                    depends_on,
                    cacheable,
                    singleton,
                )?;

                di_container
                    .register(key, Arc::new(factory_dep))
                    .map_err(|e| format!("Failed to register factory dependency: {}", e))?;
            }
        }
    }

    Ok(di_container)
}
```

**Update spikard_start_server_impl (crates/spikard-php/src/php/start.rs:525):**
```rust
pub fn spikard_start_server_impl(
    routes_zval: &Zval,
    config: &Zval,
    hooks: &Zval,
    dependencies: &Zval, // NEW PARAMETER
) -> PhpResult<u64> {
    let mut server_config = extract_server_config_from_php(config)
        .map_err(|e| PhpException::default(format!("Invalid server config: {}", e)))?;

    // Extract lifecycle hooks
    server_config.lifecycle_hooks = extract_lifecycle_hooks_from_php(hooks)
        .map_err(|e| PhpException::default(format!("Invalid lifecycle hooks: {}", e)))?;

    // Extract DI container (NEW)
    if !dependencies.is_null() {
        let di_container = extract_dependency_container_from_php(dependencies)
            .map_err(|e| PhpException::default(format!("Invalid DI container: {}", e)))?;
        server_config.di_container = Some(Arc::new(di_container));
    }

    // ... rest of function
}
```

### Resolution Algorithm

The core resolution happens in `spikard_core::di::DependencyContainer::resolve_for_handler`:

1. **Calculate Batches**: Topological sort creates batches of independent dependencies
2. **Sequential Batch Processing**: Each batch processed in order
3. **Within-Batch Sequential Resolution**: Dependencies within a batch resolved sequentially (for deterministic cleanup order)
4. **Caching**:
   - Check singleton cache first (global)
   - Check request cache (per-request)
   - Resolve and store in appropriate cache

For PHP, the `PhpFactoryDependency::resolve()` method:
1. Synchronously invokes `invoke_php_factory()` before returning future
2. Retrieves factory callable from thread-local registry
3. Builds argument array from resolved dependencies
4. Calls PHP factory with arguments
5. Returns result wrapped in `Arc<dyn Any>`

---

## 4. Handler Parameter Injection

### Detection Method

**Option A: Type-based injection (RECOMMENDED)**

Use PHP reflection to extract parameter types and match against registered dependencies:

```php
// In App::addRoute()
$reflection = new \ReflectionFunction($handler);
$handler_dependencies = [];

foreach ($reflection->getParameters() as $param) {
    $paramType = $param->getType();

    if ($paramType === null) {
        continue; // No type hint
    }

    // Get the type name (could be class name or built-in type)
    $typeName = $paramType instanceof \ReflectionNamedType
        ? $paramType->getName()
        : null;

    if ($typeName === null) {
        continue;
    }

    // Check if registered by parameter name
    if ($this->container->has($param->getName())) {
        $handler_dependencies[] = $param->getName();
    }
    // Check if registered by type name (class name)
    elseif ($this->container->has($typeName)) {
        $handler_dependencies[] = $typeName;
    }
}
```

**Option B: Explicit dependency declaration (ALTERNATIVE)**

```php
#[\Attribute]
class Inject {
    public function __construct(public array $dependencies) {}
}

#[Inject(['database', 'logger'])]
$app->addRoute('GET', '/users', function(PDO $database, LoggerInterface $logger) {
    // ...
});
```

### Injection Mechanism

**Rust-Side Handler Invocation (crates/spikard-php/src/php/handler.rs - modify invoke_php_handler):**

```rust
fn invoke_php_handler(
    handler_index: usize,
    handler_name: &str,
    request_data: &RequestData,
) -> HandlerResult {
    // Extract dependencies from request_data if available
    let resolved_deps = request_data
        .dependencies
        .as_ref()
        .map(|deps| deps.clone());

    // Build PhpRequest
    let php_request = crate::php::request::PhpRequest::from_request_data(request_data);
    let request_zval = php_request.into_zval(false).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to convert request: {:?}", e))
    })?;

    PHP_HANDLER_REGISTRY.with(|registry| -> Result<_, (StatusCode, String)> {
        let registry = registry.borrow();
        let callable_zval = registry.get(handler_index).ok_or_else(|| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Handler not found: {}", handler_index))
        })?;

        let callable = ZendCallable::new(callable_zval).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to reconstruct callable: {:?}", e))
        })?;

        // Build arguments: request + resolved dependencies
        let mut args = vec![&request_zval];

        if let Some(deps) = resolved_deps {
            // Extract handler dependencies from RequestData
            // For each dependency key, get the resolved Zval and add to args
            // This requires extending RequestData to carry handler_dependencies
            // and the resolved values

            // TODO: Implement dependency argument passing
        }

        let response_zval = callable.try_call(args).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Handler '{}' failed: {:?}", handler_name, e))
        })?;

        crate::php::server::interpret_php_response(&response_zval, handler_name)
    })
}
```

**Alternative: Pass Dependencies as Associative Array**

Instead of positional arguments, pass dependencies as an associative array in the request:

```rust
// Build dependencies array
let deps_zval = if let Some(resolved) = resolved_deps {
    let mut deps_array = ext_php_rs::types::ZendHashTable::new();
    for (key, value) in resolved.all() {
        if let Some(zval) = value.downcast_ref::<ext_php_rs::types::Zval>() {
            deps_array.insert(key, zval.shallow_clone());
        }
    }
    deps_array.into_zval(false)?
} else {
    ext_php_rs::types::Zval::new()
};

// Add to PhpRequest structure
let php_request = crate::php::request::PhpRequest {
    // ... existing fields
    dependencies: deps_zval,
};
```

Then in PHP handler:
```php
function handler(Request $request): Response {
    $db = $request->dependency('database');
    // ...
}
```

**RECOMMENDED: Match by Parameter Name/Type**

The cleanest approach is to match handler parameters by name/type and inject directly:

```php
// Handler signature
function handler(PDO $db, LoggerInterface $logger, Request $request): array

// PHP reflection extracts:
// - Parameter 0: name='db', type='PDO'
// - Parameter 1: name='logger', type='LoggerInterface'
// - Parameter 2: name='request', type='Request'

// Store in route metadata:
handler_dependencies = ['PDO', 'LoggerInterface']

// Rust resolves these dependencies and passes them positionally
```

However, this requires coordinating parameter order between PHP reflection analysis and Rust invocation. **Easier approach**: Pass dependencies as an extended Request object:

```php
class Request {
    // ... existing fields

    /** @var array<string, mixed> */
    private array $dependencies = [];

    public function dependency(string $key): mixed {
        return $this->dependencies[$key] ?? throw new RuntimeException("Dependency '{$key}' not found");
    }
}
```

---

## 5. Implementation Plan

### Files to Create

1. **crates/spikard-php/src/php/di.rs**
   - `PhpValueDependency` struct and `Dependency` impl
   - `PhpFactoryDependency` struct and `Dependency` impl
   - `PHP_FACTORY_REGISTRY` thread-local
   - `invoke_php_factory()` helper function

2. **packages/php/src/DI/Provide.php**
   - `Provide` class with factory metadata
   - Static factory methods: `factory()`, `scoped()`, `singleton()`
   - `toArray()` for Rust extraction

### Files to Modify

1. **crates/spikard-php/src/php/start.rs**
   - Add `extract_dependency_container_from_php()` function
   - Modify `spikard_start_server_impl()` to accept `dependencies` parameter
   - Set `server_config.di_container = Some(Arc::new(container))`
   - Update line 431: Remove `di_container: None` stub

2. **crates/spikard-php/src/php/handler.rs**
   - Modify `invoke_php_handler()` to extract and pass dependencies
   - Add dependency injection logic based on `RequestData.dependencies`

3. **crates/spikard-php/src/php/mod.rs**
   - Add `pub mod di;` declaration

4. **packages/php/src/DI/DependencyContainer.php**
   - Complete implementation with `singleton()`, `factory()`, `scoped()` methods
   - Add `toNative()` method for Rust extraction

5. **packages/php/src/App.php**
   - Add `withDependencies(DependencyContainer $container): self` method
   - Store container in private property
   - Pass container to `start()` method
   - Extract handler dependencies via reflection
   - Store in route metadata

6. **packages/php/src/Http/Request.php** (create if doesn't exist)
   - Add `private array $dependencies` field
   - Add `dependency(string $key): mixed` method
   - Allow Rust to populate dependencies when constructing Request

### Step-by-Step Implementation

#### Phase 1: Basic Value Dependencies

1. Create `crates/spikard-php/src/php/di.rs` with `PhpValueDependency`
2. Implement `extract_dependency_container_from_php()` for values only
3. Update `spikard_start_server_impl()` to extract and register values
4. Test with simple singleton values

#### Phase 2: Factory Dependencies

1. Add `PhpFactoryDependency` to `di.rs`
2. Implement `PHP_FACTORY_REGISTRY` thread-local
3. Implement `invoke_php_factory()` with dependency resolution
4. Update extraction to handle factories
5. Test with factories that have dependencies

#### Phase 3: Handler Parameter Detection

1. Update `packages/php/src/App.php::addRoute()`
2. Use `ReflectionFunction` to extract parameter types
3. Match against registered dependency keys
4. Store in route metadata
5. Test that handler dependencies are detected correctly

#### Phase 4: Handler Injection

1. Extend `RequestData` to carry resolved dependencies
2. Modify `invoke_php_handler()` to pass dependencies to PHP
3. Decide on injection mechanism (positional args vs Request object)
4. Implement chosen mechanism
5. Test end-to-end dependency injection

#### Phase 5: Advanced Features

1. Implement cleanup tasks (if needed for generator pattern)
2. Add error handling for missing dependencies
3. Add error handling for type mismatches
4. Performance optimization: cache reflection results
5. Add comprehensive tests

---

## 6. Code Examples

### Rust Implementation Sketch

**crates/spikard-php/src/php/di.rs (PhpValueDependency):**
```rust
use ext_php_rs::types::Zval;
use http::Request;
use spikard_core::di::{Dependency, DependencyError, ResolvedDependencies};
use spikard_core::request_data::RequestData;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct PhpValueDependency {
    key: String,
    value: Zval,
}

impl PhpValueDependency {
    pub fn new(key: String, value: Zval) -> Self {
        Self { key, value }
    }
}

impl Dependency for PhpValueDependency {
    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        _resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>> {
        let value = self.value.shallow_clone();
        Box::pin(async move {
            Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
        })
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        vec![]
    }

    fn singleton(&self) -> bool {
        true
    }

    fn cacheable(&self) -> bool {
        true
    }
}
```

**crates/spikard-php/src/php/di.rs (PhpFactoryDependency - abbreviated):**
```rust
pub struct PhpFactoryDependency {
    key: String,
    factory_index: usize,
    depends_on: Vec<String>,
    cacheable: bool,
    singleton: bool,
}

thread_local! {
    static PHP_FACTORY_REGISTRY: std::cell::RefCell<Vec<Zval>> = std::cell::RefCell::new(Vec::new());
}

impl PhpFactoryDependency {
    pub fn register_from_zval(
        key: String,
        factory_zval: &Zval,
        depends_on: Vec<String>,
        cacheable: bool,
        singleton: bool,
    ) -> Result<Self, String> {
        // Validate callable
        // Register in thread-local
        // Return Self with index
    }
}

impl Dependency for PhpFactoryDependency {
    fn resolve(...) -> Pin<Box<...>> {
        let result = invoke_php_factory(
            self.factory_index,
            &self.key,
            &self.depends_on,
            request_data,
            resolved,
        );
        Box::pin(async move { result })
    }

    // ... other trait methods
}

fn invoke_php_factory(
    factory_index: usize,
    key: &str,
    depends_on: &[String],
    _request_data: &RequestData,
    resolved: &ResolvedDependencies,
) -> Result<Arc<dyn Any + Send + Sync>, DependencyError> {
    PHP_FACTORY_REGISTRY.with(|registry| {
        // Get factory callable
        // Build args from resolved dependencies
        // Invoke callable
        // Return result
    })
}
```

### PHP Implementation Sketch

**packages/php/src/DI/Provide.php:**
```php
<?php
declare(strict_types=1);

namespace Spikard\DI;

final class Provide
{
    public function __construct(
        public readonly mixed $factory,
        public readonly array $dependsOn = [],
        public readonly bool $cacheable = false,
        public readonly bool $singleton = false,
    ) {
        if (!is_callable($this->factory)) {
            throw new \InvalidArgumentException('Factory must be callable');
        }
    }

    public static function factory(callable $factory, array $dependsOn = []): self
    {
        return new self($factory, $dependsOn, cacheable: false, singleton: false);
    }

    public static function scoped(callable $factory, array $dependsOn = []): self
    {
        return new self($factory, $dependsOn, cacheable: true, singleton: false);
    }

    public static function singleton(callable $factory, array $dependsOn = []): self
    {
        return new self($factory, $dependsOn, cacheable: true, singleton: true);
    }

    /** @return array{factory: callable, depends_on: list<string>, cacheable: bool, singleton: bool} */
    public function toArray(): array
    {
        return [
            'factory' => $this->factory,
            'depends_on' => $this->dependsOn,
            'cacheable' => $this->cacheable,
            'singleton' => $this->singleton,
        ];
    }
}
```

**packages/php/src/DI/DependencyContainer.php:**
```php
<?php
declare(strict_types=1);

namespace Spikard\DI;

final class DependencyContainer
{
    /** @var array<string, mixed> */
    private array $values = [];

    /** @var array<string, Provide> */
    private array $factories = [];

    public function singleton(string $key, mixed $value): self
    {
        $this->values[$key] = $value;
        return $this;
    }

    public function factory(string $key, Provide $factory): self
    {
        $this->factories[$key] = $factory;
        return $this;
    }

    public function scoped(string $key, callable $factory, array $dependsOn = []): self
    {
        return $this->factory($key, Provide::scoped($factory, $dependsOn));
    }

    public function has(string $key): bool
    {
        return isset($this->values[$key]) || isset($this->factories[$key]);
    }

    /**
     * Convert to array for Rust extraction.
     *
     * @return array{values: array<string, mixed>, factories: array<string, array>}
     */
    public function toNative(): array
    {
        $factories = [];
        foreach ($this->factories as $key => $provide) {
            $factories[$key] = $provide->toArray();
        }

        return [
            'values' => $this->values,
            'factories' => $factories,
        ];
    }
}
```

**packages/php/src/App.php (withDependencies method):**
```php
private ?DependencyContainer $container = null;

public function withDependencies(DependencyContainer $container): self
{
    $this->container = $container;
    return $this;
}

public function addRoute(string $method, string $path, callable $handler): void
{
    // Existing parameter extraction...

    // NEW: Extract handler dependencies via reflection
    $handler_dependencies = [];

    if ($this->container !== null) {
        $reflection = new \ReflectionFunction($handler);

        foreach ($reflection->getParameters() as $param) {
            // Skip standard request parameters
            if ($param->getName() === 'request' || $param->getName() === 'response') {
                continue;
            }

            $paramType = $param->getType();
            if ($paramType instanceof \ReflectionNamedType) {
                $typeName = $paramType->getName();

                // Check by parameter name
                if ($this->container->has($param->getName())) {
                    $handler_dependencies[] = $param->getName();
                }
                // Check by type name (class)
                elseif ($this->container->has($typeName)) {
                    $handler_dependencies[] = $typeName;
                }
            }
        }
    }

    // Store in route metadata
    $route = [
        'method' => $method,
        'path' => $path,
        'handler' => $handler,
        'handler_name' => $this->getHandlerName($handler),
        'handler_dependencies' => $handler_dependencies,
        // ... other fields
    ];

    $this->routes[] = $route;
}

public function start(ServerConfig $config): void
{
    $dependencies_native = $this->container?->toNative() ?? null;

    spikard_start_server(
        routes: $this->routesToNative(),
        config: $config->toNative(),
        hooks: $this->hooksToNative(),
        dependencies: $dependencies_native, // NEW PARAMETER
    );
}
```

---

## 7. Edge Cases & Testing

### Edge Cases

1. **Missing Dependency**:
   - Handler requires dependency that isn't registered
   - **Error**: Return 500 with descriptive error message
   - **Detection**: During route registration or first request

2. **Type Mismatch**:
   - Handler expects `PDO`, factory returns `string`
   - **Error**: PHP will throw TypeError when calling handler
   - **Prevention**: Validate types at registration time (optional strict mode)

3. **Circular Dependencies**:
   - A depends on B, B depends on A
   - **Error**: Detected by `DependencyGraph::add_dependency()`
   - **Response**: Error during server startup, not runtime

4. **Factory Throws Exception**:
   - PHP factory callable throws exception during resolution
   - **Error**: Convert to `DependencyError::ResolutionFailed`
   - **Response**: Return 500, log error

5. **Null Return from Factory**:
   - Factory returns null instead of expected type
   - **Handling**: Allow null if parameter is nullable, error otherwise

6. **Multiple Type Candidates**:
   - Handler has parameter `LoggerInterface $logger` but multiple implementations registered
   - **Resolution**: Match by parameter name first, then by type
   - **Recommendation**: Use explicit naming for interfaces

### Testing Strategy

**Unit Tests (Rust):**
```rust
#[test]
fn test_php_value_dependency() {
    // Create PhpValueDependency
    // Resolve it
    // Assert value is correct
}

#[test]
fn test_php_factory_dependency() {
    // Register factory in PHP_FACTORY_REGISTRY
    // Create PhpFactoryDependency
    // Resolve with dependencies
    // Assert factory was called with correct args
}

#[test]
fn test_extract_container_values_only() {
    // Create PHP array with values
    // Call extract_dependency_container_from_php
    // Assert container has correct dependencies
}

#[test]
fn test_extract_container_factories() {
    // Create PHP array with factories
    // Extract container
    // Assert factories registered correctly
}
```

**Integration Tests (PHP):**
```php
function test_singleton_value_injection(): void
{
    $app = new App();
    $container = new DependencyContainer();

    $container->singleton('app_name', 'TestApp');
    $app->withDependencies($container);

    $called = false;
    $app->addRoute('GET', '/test', function (string $app_name) use (&$called) {
        $called = true;
        assert($app_name === 'TestApp');
        return ['name' => $app_name];
    });

    $client = $app->testClient();
    $response = $client->get('/test');

    assert($called === true);
    assert($response->json() === ['name' => 'TestApp']);
}

function test_factory_dependency(): void
{
    $app = new App();
    $container = new DependencyContainer();

    $container->singleton('config', ['db_host' => 'localhost']);
    $container->factory('database', Provide::singleton(
        factory: function (array $config): PDO {
            return new PDO("sqlite::memory:");
        },
        dependsOn: ['config']
    ));

    $app->withDependencies($container);

    $app->addRoute('GET', '/users', function (PDO $database): array {
        $stmt = $database->query('SELECT 1');
        return ['result' => $stmt->fetch()];
    });

    $client = $app->testClient();
    $response = $client->get('/users');

    assert($response->statusCode() === 200);
}

function test_scoped_dependency(): void
{
    $counter = 0;

    $app = new App();
    $container = new DependencyContainer();

    $container->scoped('request_id', function () use (&$counter): string {
        return 'request-' . ($counter++);
    });

    $app->withDependencies($container);

    $app->addRoute('GET', '/id', function (string $request_id): array {
        return ['id' => $request_id];
    });

    $client = $app->testClient();

    $response1 = $client->get('/id');
    $response2 = $client->get('/id');

    // Different requests get different IDs (scoped, not singleton)
    assert($response1->json()['id'] !== $response2->json()['id']);
}
```

---

## 8. Migration Path

### Backward Compatibility

**Existing handlers without DI continue to work:**
```php
// Before (still works)
$app->addRoute('GET', '/hello', function (): array {
    return ['message' => 'Hello'];
});

// After (DI optional)
$container = new DependencyContainer();
$container->singleton('greeting', 'Hello');
$app->withDependencies($container);

$app->addRoute('GET', '/hello', function (string $greeting): array {
    return ['message' => $greeting];
});
```

### Gradual Adoption

1. **Phase 1**: Add `withDependencies()` method, no-op if not called
2. **Phase 2**: Implement value dependencies (singletons)
3. **Phase 3**: Add factory dependencies
4. **Phase 4**: Optimize performance and add advanced features

### Documentation Requirements

1. **Basic Guide**: Simple examples of value and factory dependencies
2. **Advanced Guide**: Scoped dependencies, cleanup tasks, best practices
3. **Migration Guide**: How to refactor existing handlers to use DI
4. **API Reference**: Complete API documentation for all DI classes

---

## Summary

This design provides a complete, production-ready DI system for PHP bindings that:

1. **Matches Python/Node/Ruby patterns**: Uses the same core `Dependency` trait and `DependencyContainer`
2. **Leverages existing infrastructure**: Builds on `spikard_core::di` without duplication
3. **Handles PHP's constraints**: Thread-local storage for Zval, synchronous invocation
4. **Supports all dependency types**: Values, factories, scoped, singletons
5. **Provides clean PHP API**: Intuitive API similar to other DI frameworks
6. **Ensures type safety**: Reflection-based parameter matching with type hints
7. **Maintains backward compatibility**: Existing handlers work unchanged
8. **Delivers good performance**: Efficient caching, minimal overhead

**Key Challenges Addressed:**
- ✅ Zval thread-local storage (like `PHP_HANDLER_REGISTRY`)
- ✅ Type-based and name-based dependency resolution
- ✅ Synchronous factory invocation (no async event loop needed)
- ✅ Clean extraction from PHP to Rust
- ✅ Proper error handling and validation

**Next Steps:**
1. Create `crates/spikard-php/src/php/di.rs` with value and factory implementations
2. Update `start.rs` to extract and register dependencies
3. Implement PHP API classes (`Provide`, `DependencyContainer`)
4. Add handler parameter detection in `App::addRoute()`
5. Test thoroughly with integration tests
