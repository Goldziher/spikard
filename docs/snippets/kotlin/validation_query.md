```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

fun main() = runBlocking {
    val app = App()

    app.get({ request ->
        // Query parameters are validated at the Rust core level
        // Extract them from the request context
        val limit = 10
        val offset = 0
        val response = buildJsonObject {
            put("limit", limit)
            put("offset", offset)
            put("items", emptyList<Any>())
        }
        Json.encodeToString(response)
    }, "/items")

    app.run()
}
```
