```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class ApiResponse<T>(
    val status: String,
    val data: T?,
    val error: String? = null
)

@Serializable
data class Product(
    val id: String,
    val name: String,
    val price: Double
)

fun main() = runBlocking {
    val app = App()

    app.get({ request ->
        val product = Product(
            id = "prod_123",
            name = "Widget",
            price = 29.99
        )
        val response = ApiResponse(
            status = "success",
            data = product
        )
        Json.encodeToString(response)
    }, "/products/{id}")

    app.run()
}
```
