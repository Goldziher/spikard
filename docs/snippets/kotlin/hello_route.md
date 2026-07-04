```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable

@Serializable
data class User(val id: Int, val name: String)

fun main() = runBlocking {
    val app = App()

    app.get({ request ->
        val userId = 1
        kotlinx.serialization.json.Json.encodeToString(
            User(id = userId, name = "Alice")
        )
    }, "/users/{id:int}")

    app.run()
}
```
