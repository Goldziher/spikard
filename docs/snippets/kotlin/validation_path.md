```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

fun main() = runBlocking {
    val app = App()

    // Path parameters with type validation
    app.get({ request ->
        // The Rust core validates path parameters using the type syntax
        val userId = "123"  // Would be validated as integer
        val response = buildJsonObject {
            put("user_id", userId)
            put("found", true)
        }
        Json.encodeToString(response)
    }, "/users/{user_id:int}")

    // Path parameter with UUID validation
    app.get({ request ->
        val resourceId = "550e8400-e29b-41d4-a716-446655440000"
        val response = buildJsonObject {
            put("resource_id", resourceId)
            put("type", "document")
        }
        Json.encodeToString(response)
    }, "/resources/{id:uuid}")

    app.run()
}
```
