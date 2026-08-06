```kotlin
package dev.spikard.docs

import com.fasterxml.jackson.databind.ObjectMapper
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Verifies that a route backed by [dev.spikard.BackgroundTaskConfig] enqueues work and replies
 * immediately with 202 Accepted, without waiting for the background task to finish.
 *
 * The app under test (see background_task.md) is started separately and exposed through
 * `SUT_URL`, matching the convention used by the generated Kotlin e2e suite.
 */
class BackgroundTaskTest {

    companion object {
        private val MAPPER = ObjectMapper()
        private val baseUrl = System.getenv("SUT_URL") ?: "http://127.0.0.1:8007"
    }

    @Test
    fun `enqueues background work and returns 202 immediately`() {
        val uri = java.net.URI.create("$baseUrl/process-async")
        val request = java.net.http.HttpRequest.newBuilder(uri)
            .method("POST", java.net.http.HttpRequest.BodyPublishers.ofString("{}"))
            .header("Content-Type", "application/json")
            .build()

        val response = java.net.http.HttpClient.newHttpClient()
            .send(request, java.net.http.HttpResponse.BodyHandlers.ofString())

        assertEquals(202, response.statusCode(), "background routes must respond before the task completes")

        val body = MAPPER.readTree(response.body())
        assertTrue(body.get("task_id").asText().startsWith("bg_"), "task_id mismatch")
        assertEquals("queued", body.get("status").asText())
    }
}
```
