```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import dev.spikard.BackgroundTaskConfig
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.launch
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

fun main() = runBlocking {
    val config = ServerConfig().apply {
        this.backgroundTasks = BackgroundTaskConfig().apply {
            this.maxQueueSize = 1024
            this.maxConcurrentTasks = 128
            this.drainTimeoutSecs = 30
        }
    }

    val app = App()

    app.post({ request ->
        launch {
            try {
                // Simulate work that might fail
                if (Math.random() > 0.5) {
                    throw Exception("Random task failure")
                }
                println("Task completed successfully")
            } catch (e: Exception) {
                // Handle error gracefully
                println("Task failed with error: ${e.message}")
                // Optionally log or retry
            }
        }
        val response = buildJsonObject {
            put("task_id", "bg_${System.currentTimeMillis()}")
            put("status", "accepted")
        }
        Json.encodeToString(response)
    }, "/tasks/retry")

    app.run()
}
```
