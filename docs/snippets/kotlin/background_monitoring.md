```kotlin
import dev.spikard.kt.App
import dev.spikard.ServerConfig
import dev.spikard.BackgroundTaskConfig
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import java.util.concurrent.atomic.AtomicInteger

fun main() = runBlocking {
    val config = ServerConfig().apply {
        this.backgroundTasks = BackgroundTaskConfig().apply {
            this.maxQueueSize = 1024
            this.maxConcurrentTasks = 128
        }
    }

    val app = App()
    val taskCounter = AtomicInteger(0)
    val completedCounter = AtomicInteger(0)

    app.post({ request ->
        taskCounter.incrementAndGet()
        val response = buildJsonObject {
            put("task_id", "bg_${System.currentTimeMillis()}")
            put("status", "queued")
        }
        Json.encodeToString(response)
    }, "/tasks")

    app.get({ request ->
        val response = buildJsonObject {
            put("total_tasks", taskCounter.get())
            put("completed_tasks", completedCounter.get())
            put("queue_size", taskCounter.get() - completedCounter.get())
        }
        Json.encodeToString(response)
    }, "/tasks/status")

    app.run()
}
```
