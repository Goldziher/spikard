```kotlin
import dev.spikard.kt.App
import dev.spikard.UploadFile
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

fun main() = runBlocking {
    val app = App()

    app.post({ request ->
        // File upload handling is managed by the Rust core
        // The handler receives the file content
        val result = buildJsonObject {
            put("filename", "uploaded_file.txt")
            put("size", request.length)
            put("content_type", "text/plain")
        }
        Json.encodeToString(result)
    }, "/upload")

    app.run()
}
```
