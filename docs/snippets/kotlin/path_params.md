```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class OrderResponse(val id: Int, val details: Boolean)

fun main() = runBlocking {
    val app = App()

    app.get({ request ->
        val orderId = 123
        val includeDetails = false
        Json.encodeToString(
            OrderResponse(id = orderId, details = includeDetails)
        )
    }, "/orders/{order_id:int}")

    app.run()
}
```
