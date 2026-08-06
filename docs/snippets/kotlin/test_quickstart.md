```kotlin
package dev.spikard.docs

import com.fasterxml.jackson.databind.ObjectMapper
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

/** Tests the `/hello` route registered in quickstart_routes.md and response_basic.md. */
class HelloRouteTest {

    companion object {
        private val MAPPER = ObjectMapper()
        private val baseUrl = System.getenv("SUT_URL") ?: "http://127.0.0.1:8007"
    }

    @Test
    fun `returns a greeting`() {
        val uri = java.net.URI.create("$baseUrl/hello")
        val request = java.net.http.HttpRequest.newBuilder(uri).GET().build()

        val response = java.net.http.HttpClient.newHttpClient()
            .send(request, java.net.http.HttpResponse.BodyHandlers.ofString())

        assertEquals(200, response.statusCode())
        val expected = MAPPER.readTree("""{"message":"Hello, World!"}""")
        assertEquals(expected, MAPPER.readTree(response.body()))
    }
}
```
