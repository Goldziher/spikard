```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class User(val id: Int, val name: String)

fun main() = runBlocking {
    val app = App()

    app.get({ request ->
        Json.encodeToString(mapOf("status" to "ok"))
    }, "/health")

    app.post({ request ->
        val user = Json.decodeFromString<User>(request)
        Json.encodeToString(user)
    }, "/users")

    app.run()
}
```
