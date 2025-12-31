# Protobuf Type Mapping Across Languages

Reference table showing how proto3 types map to native language types in all Spikard-supported languages.

## Scalar Types

| Proto3 Type | Python | TypeScript | Ruby | PHP | Rust |
|-------------|--------|------------|------|-----|------|
| `double` | `float` | `number` | `Float` | `float` | `f64` |
| `float` | `float` | `number` | `Float` | `float` | `f32` |
| `int32` | `int` | `number` | `Integer` | `int` | `i32` |
| `int64` | `int` | `number\|bigint` | `Integer` | `int` | `i64` |
| `uint32` | `int` | `number` | `Integer` | `int` | `u32` |
| `uint64` | `int` | `number\|bigint` | `Integer` | `int` | `u64` |
| `bool` | `bool` | `boolean` | `Boolean` | `bool` | `bool` |
| `string` | `str` | `string` | `String` | `string` | `String` |
| `bytes` | `bytes` | `Uint8Array` | `String` | `string` | `Bytes` |

## Complex Types

| Proto3 Type | Python | TypeScript | Ruby | PHP | Rust |
|-------------|--------|------------|------|-----|------|
| `message` | `dataclass` | `interface` | `class` | `class` | `struct` |
| `enum` | `Literal[...]` | `union type` | `module` | `class` | `enum` |
| `repeated T` | `list[T]` | `T[]` | `Array<T>` | `array<T>` | `Vec<T>` |
| `map<K,V>` | `dict[K,V]` | `Map<K,V>` | `Hash{K=>V}` | `array<K,V>` | `HashMap<K,V>` |
| `optional T` | `T\|None` | `T\|undefined` | `T\|nil` | `?T` | `Option<T>` |

## Default Values in Proto3

When a field is not set, proto3 uses these default values:

| Type | Default Value |
|------|---------------|
| Numbers | `0` |
| Strings | `""` (empty string) |
| Booleans | `false` |
| Enums | First value (must be 0) |
| Messages | `None`/`null`/`undefined` |
| Repeated | Empty list |
| Maps | Empty map |

## Optional Field Handling

Use `optional` keyword to distinguish between "not set" and "default value":

```protobuf
message User {
  int32 id = 1;                // Never null (defaults to 0)
  string name = 2;             // Never null (defaults to "")
  optional string email = 3;   // Can be null/None/undefined
  optional int32 age = 4;      // Can be null (distinguishes 0 from unset)
}
```

### Checking Optional Fields by Language

**Python**:
```python
if user.HasField("email"):
    print(f"Email: {user.email}")
```

**TypeScript**:
```typescript
if (user.email !== undefined) {
    console.log(`Email: ${user.email}`);
}
```

**Ruby**:
```ruby
if user.respond_to?(:email) && user.email
    puts "Email: #{user.email}"
end
```

**PHP**:
```php
if ($user->hasEmail()) {
    echo "Email: " . $user->getEmail();
}
```

**Rust**:
```rust
if let Some(email) = &user.email {
    println!("Email: {}", email);
}
```
