```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import dev.spikard.CompressionConfig
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val config = ServerConfig().apply {
        this.host = "0.0.0.0"
        this.port = 8080
        this.enableRequestId = true
        this.compression = CompressionConfig().apply {
            this.gzip = true
            this.brotli = true
        }
    }

    val app = App()

    app.get({ request ->
        Json.encodeToString(mapOf("status" to "ok"))
    }, "/health")

    app.run()
}
```
