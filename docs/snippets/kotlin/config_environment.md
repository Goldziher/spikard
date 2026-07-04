```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val port = System.getenv("PORT")?.toIntOrNull() ?: 8000
    val host = System.getenv("HOST") ?: "127.0.0.1"
    val workers = System.getenv("WORKERS")?.toLongOrNull() ?: 1L

    val config = ServerConfig().apply {
        this.host = host
        this.port = port.toShort()
        this.workers = workers
    }

    val app = App()

    app.get({ request ->
        Json.encodeToString(mapOf("status" to "ok"))
    }, "/health")

    app.run()
}
```
