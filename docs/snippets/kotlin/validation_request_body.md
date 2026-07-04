```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class CreateUserRequest(
    val email: String,
    val name: String,
    val age: Int
)

@Serializable
data class UserResponse(
    val id: String,
    val email: String,
    val name: String
)

fun main() = runBlocking {
    val app = App()

    app.post({ request ->
        val user = Json.decodeFromString<CreateUserRequest>(request)
        val response = UserResponse(
            id = "user_${System.currentTimeMillis()}",
            email = user.email,
            name = user.name
        )
        Json.encodeToString(response)
    }, "/users")

    app.run()
}
```
