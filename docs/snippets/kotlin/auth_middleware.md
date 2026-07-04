```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import dev.spikard.JwtConfig
import dev.spikard.ApiKeyConfig
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val config = ServerConfig().apply {
        this.host = "0.0.0.0"
        this.port = 8080
        // JWT authentication
        this.jwtAuth = JwtConfig().apply {
            this.secret = "your-secret-key"
            this.algorithm = "HS256"
            this.audience = listOf("myapp")
            this.issuer = "myissuer"
        }
        // API Key authentication (fallback)
        this.apiKeyAuth = ApiKeyConfig().apply {
            this.keys = listOf("sk_live_123456789", "sk_test_987654321")
            this.headerName = "X-API-Key"
        }
    }

    val app = App()

    app.post({ request ->
        Json.encodeToString(mapOf("authorized" to true))
    }, "/admin/users")

    app.run()
}
```
