```kotlin
package dev.spikard.docs

import com.fasterxml.jackson.databind.ObjectMapper
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

/** Unit test for a single `POST /users` handler, isolated from the rest of the app. */
class CreateUserTest {

    companion object {
        private val MAPPER = ObjectMapper()
        private val baseUrl = System.getenv("SUT_URL") ?: "http://127.0.0.1:8007"
    }

    @Test
    fun `creates a user from the request body`() {
        val uri = java.net.URI.create("$baseUrl/users")
        val request = java.net.http.HttpRequest.newBuilder(uri)
            .method(
                "POST",
                java.net.http.HttpRequest.BodyPublishers.ofString(
                    """{"name":"Alice","email":"alice@example.com"}""",
                ),
            )
            .header("Content-Type", "application/json")
            .build()

        val response = java.net.http.HttpClient.newHttpClient()
            .send(request, java.net.http.HttpResponse.BodyHandlers.ofString())

        assertEquals(200, response.statusCode())
        val body = MAPPER.readTree(response.body())
        assertEquals("Alice", body.get("name").asText())
        assertEquals("alice@example.com", body.get("email").asText())
    }
}
```
