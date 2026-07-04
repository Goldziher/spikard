```kotlin
import dev.spikard.kt.App
import dev.spikard.SseEvent
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

fun main() = runBlocking {
    val app = App()

    app.get({ request ->
        // In a real scenario, this would be a streaming endpoint
        // Server-Sent Events are handled by the Rust core
        val events = listOf(
            SseEvent(buildJsonObject { put("tick", 0) }).withId("0"),
            SseEvent(buildJsonObject { put("tick", 1) }).withId("1"),
            SseEvent(buildJsonObject { put("tick", 2) }).withId("2")
        )
        Json.encodeToString(events)
    }, "/events")

    app.run()
}
```
