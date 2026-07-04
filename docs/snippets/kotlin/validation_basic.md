```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class Payment(val id: String, val amount: Double)

fun main() = runBlocking {
    val app = App()

    app.post({ request ->
        val payment = Json.decodeFromString<Payment>(request)
        Json.encodeToString(payment)
    }, "/payments")

    app.run()
}
```
