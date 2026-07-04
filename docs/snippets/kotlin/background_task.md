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
        // Spawn a background task for async processing
        launch {
            // Simulate async work
            kotlinx.coroutines.delay(1000)
            println("Background task completed")
        }
        val response = buildJsonObject {
            put("task_id", "bg_${System.currentTimeMillis()}")
            put("status", "queued")
        }
        Json.encodeToString(response)
    }, "/process-async")

    app.run()
}
```
