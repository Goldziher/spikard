```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val app = App()

    app.get({ request ->
        Json.encodeToString(mapOf("message" to "Hello, World!"))
    }, "/hello")

    app.get({ request ->
        Json.encodeToString(mapOf("status" to "success", "code" to 200))
    }, "/status")

    app.run()
}
```
