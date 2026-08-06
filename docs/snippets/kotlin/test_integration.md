```kotlin
package dev.spikard.docs

import com.fasterxml.jackson.databind.ObjectMapper
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

/** End-to-end workflow: create a user, then fetch it back by id. */
class UserWorkflowTest {

    companion object {
        private val MAPPER = ObjectMapper()
        private val baseUrl = System.getenv("SUT_URL") ?: "http://127.0.0.1:8007"
    }

    @Test
    fun `completes the create-then-fetch workflow`() {
        val client = java.net.http.HttpClient.newHttpClient()

        val createUri = java.net.URI.create("$baseUrl/users")
        val createRequest = java.net.http.HttpRequest.newBuilder(createUri)
            .method("POST", java.net.http.HttpRequest.BodyPublishers.ofString("""{"name":"Alice"}"""))
            .header("Content-Type", "application/json")
            .build()
        val createResponse = client.send(createRequest, java.net.http.HttpResponse.BodyHandlers.ofString())
        assertEquals(200, createResponse.statusCode())

        val created = MAPPER.readTree(createResponse.body())
        assertEquals("Alice", created.get("name").asText())

        val getUri = java.net.URI.create("$baseUrl/users/${created.get("id").asInt()}")
        val getRequest = java.net.http.HttpRequest.newBuilder(getUri).GET().build()
        val getResponse = client.send(getRequest, java.net.http.HttpResponse.BodyHandlers.ofString())
        assertEquals(200, getResponse.statusCode())

        val fetched = MAPPER.readTree(getResponse.body())
        assertEquals(created, fetched)
    }
}
```
