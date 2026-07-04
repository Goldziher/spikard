```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray

fun main() = runBlocking {
    val app = App()

    app.get({ request ->
        val items = (0..9).map { i ->
            buildJsonObject {
                put("id", i)
                put("value", i * 10)
            }
        }
        Json.encodeToString(items)
    }, "/stream/items")

    app.run()
}
```
