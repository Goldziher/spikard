```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class RequestPayload(val name: String, val email: String, val age: Int)

@Serializable
data class ResponsePayload(val id: String, val received: String)

fun main() = runBlocking {
    val app = App()

    app.post({ request ->
        val payload = Json.decodeFromString<RequestPayload>(request)
        val response = ResponsePayload(
            id = "req-${System.currentTimeMillis()}",
            received = payload.name
        )
        Json.encodeToString(response)
    }, "/process")

    app.run()
}
```
