```kotlin
import dev.spikard.kt.App
import dev.spikard.ProblemDetails
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val app = App()

    app.post({ request ->
        // Error responses use RFC 9457 Problem Details format
        val error = ProblemDetails.badRequest("Invalid email format").apply {
            this.extensions = mapOf(
                "field" to "email",
                "value" to "invalid-email"
            )
        }
        // In real scenarios, this would be returned with HTTP 400
        Json.encodeToString(error)
    }, "/validate")

    app.run()
}
```
