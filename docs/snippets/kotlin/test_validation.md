```kotlin
package dev.spikard.docs

import com.fasterxml.jackson.databind.ObjectMapper
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

/**
 * Verifies that request validation, enforced by the Rust core via the schema attached with
 * [dev.spikard.kt.RouteBuilder.requestSchemaJson], rejects a malformed body with a 422
 * ProblemDetails response before the handler runs.
 */
class ValidationFailureTest {

    companion object {
        private val MAPPER = ObjectMapper()
        private val baseUrl = System.getenv("SUT_URL") ?: "http://127.0.0.1:8007"
    }

    @Test
    fun `rejects a body with the wrong field type`() {
        val uri = java.net.URI.create("$baseUrl/users")
        val request = java.net.http.HttpRequest.newBuilder(uri)
            .method(
                "POST",
                java.net.http.HttpRequest.BodyPublishers.ofString(
                    """{"name":"Bob","age":"not a number"}""",
                ),
            )
            .header("Content-Type", "application/json")
            .build()

        val response = java.net.http.HttpClient.newHttpClient()
            .send(request, java.net.http.HttpResponse.BodyHandlers.ofString())

        assertEquals(422, response.statusCode())
        val body = MAPPER.readTree(response.body())
        assertEquals("https://spikard.dev/errors/validation-error", body.get("type").asText())
    }
}
```
