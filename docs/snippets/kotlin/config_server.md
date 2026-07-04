```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val config = ServerConfig().apply {
        this.host = "0.0.0.0"
        this.port = 8080
        this.workers = 4
        this.requestTimeout = 60
        this.maxBodySize = 5 * 1024 * 1024  // 5MB
    }

    val app = App()

    app.get({ request ->
        Json.encodeToString(mapOf("status" to "ok"))
    }, "/health")

    // Note: Kotlin JVM binding does not currently support configuration injection
    app.run()
}
```
