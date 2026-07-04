```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val config = ServerConfig().apply {
        this.host = "0.0.0.0"
        this.port = 8080
        this.enableRequestId = true
        this.enableHttpTrace = true
    }

    val app = App()

    app.get({ request ->
        val data = mapOf(
            "status" to "ok",
            "service" to "api",
            "version" to "1.0.0"
        )
        Json.encodeToString(data)
    }, "/metrics")

    app.run()
}
```
