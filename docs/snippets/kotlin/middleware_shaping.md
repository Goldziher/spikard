```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import dev.spikard.RateLimitConfig
import dev.spikard.CompressionConfig
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val config = ServerConfig().apply {
        this.host = "0.0.0.0"
        this.port = 8080
        this.maxBodySize = 10 * 1024 * 1024  // 10MB
        this.requestTimeout = 30
        // Rate limiting with burst allowance
        this.rateLimit = RateLimitConfig().apply {
            this.perSecond = 100
            this.burst = 200
            this.ipBased = true
        }
        // Response compression
        this.compression = CompressionConfig().apply {
            this.gzip = true
            this.brotli = true
            this.minSize = 1024
        }
    }

    val app = App()

    app.get({ request ->
        Json.encodeToString(mapOf("message" to "rate limited"))
    }, "/limited")

    app.run()
}
```
