```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import dev.spikard.CompressionConfig
import dev.spikard.RateLimitConfig
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val config = ServerConfig().apply {
        this.host = "0.0.0.0"
        this.port = 8080
        this.workers = 8
        this.requestTimeout = 30
        this.gracefulShutdown = true
        this.shutdownTimeout = 30
        this.enableRequestId = true
        // Compression configuration
        this.compression = CompressionConfig().apply {
            this.gzip = true
            this.brotli = true
            this.minSize = 1024
        }
        // Rate limiting configuration
        this.rateLimit = RateLimitConfig().apply {
            this.perSecond = 1000
            this.burst = 2000
            this.ipBased = true
        }
    }

    val app = App()

    app.get({ request ->
        Json.encodeToString(mapOf("status" to "healthy"))
    }, "/health")

    app.run()
}
```
