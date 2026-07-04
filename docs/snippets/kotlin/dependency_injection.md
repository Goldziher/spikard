```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

// Service dependencies
class UserService {
    fun getUserById(id: Int): Map<String, Any> {
        return mapOf("id" to id, "name" to "Alice", "email" to "alice@example.com")
    }
}

class NotificationService {
    fun sendNotification(userId: Int, message: String): Boolean {
        println("Notification sent to user $userId: $message")
        return true
    }
}

fun main() = runBlocking {
    val app = App()

    // Instantiate services
    val userService = UserService()
    val notificationService = NotificationService()

    app.get({ request ->
        val userId = 1
        val user = userService.getUserById(userId)
        Json.encodeToString(user)
    }, "/users/{id:int}")

    app.post({ request ->
        val userId = 1
        val sent = notificationService.sendNotification(userId, "Welcome!")
        val response = buildJsonObject {
            put("user_id", userId)
            put("sent", sent)
        }
        Json.encodeToString(response)
    }, "/notifications")

    app.run()
}
```
