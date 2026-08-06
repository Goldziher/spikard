```kotlin
package dev.spikard.docs

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/** Tests the `/events` SSE route registered in sse.md. */
class SseRouteTest {

    companion object {
        private val baseUrl = System.getenv("SUT_URL") ?: "http://127.0.0.1:8007"
    }

    @Test
    fun `streams server-sent events`() {
        val uri = java.net.URI.create("$baseUrl/events")
        val request = java.net.http.HttpRequest.newBuilder(uri)
            .header("Accept", "text/event-stream")
            .GET()
            .build()

        val response = java.net.http.HttpClient.newHttpClient()
            .send(request, java.net.http.HttpResponse.BodyHandlers.ofLines())

        assertEquals(200, response.statusCode())
        assertTrue(response.headers().firstValue("Content-Type").orElse("").contains("text/event-stream"))

        val dataLines = response.body().filter { it.startsWith("data:") }.limit(3).toList()
        assertEquals(3, dataLines.size)
        assertTrue(dataLines[0].contains("\"tick\":0"))
    }
}
```
