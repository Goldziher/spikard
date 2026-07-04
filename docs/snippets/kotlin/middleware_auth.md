```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import dev.spikard.JwtConfig
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val config = ServerConfig().apply {
        this.host = "0.0.0.0"
        this.port = 8080
        this.jwtAuth = JwtConfig().apply {
            this.secret = "your-secret-key-here"
            this.algorithm = "HS256"
            this.audience = listOf("myapp")
            this.issuer = "myissuer"
            this.leeway = 60
        }
    }

    val app = App()

    app.get({ request ->
        Json.encodeToString(mapOf("authenticated" to true))
    }, "/protected")

    app.run()
}
```
