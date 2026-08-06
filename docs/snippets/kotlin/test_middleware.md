```kotlin
package dev.spikard.docs

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

/**
 * Verifies the API-key fallback configured alongside JWT auth in auth_middleware.md: requests
 * without a valid `X-API-Key` header are rejected before reaching the handler, requests with
 * one are not.
 */
class ApiKeyMiddlewareTest {

    companion object {
        private val baseUrl = System.getenv("SUT_URL") ?: "http://127.0.0.1:8007"
    }

    @Test
    fun `rejects requests with an invalid api key`() {
        val uri = java.net.URI.create("$baseUrl/admin/users")
        val request = java.net.http.HttpRequest.newBuilder(uri)
            .method("POST", java.net.http.HttpRequest.BodyPublishers.noBody())
            .header("X-API-Key", "not-a-real-key")
            .build()

        val response = java.net.http.HttpClient.newHttpClient()
            .send(request, java.net.http.HttpResponse.BodyHandlers.ofString())

        assertEquals(401, response.statusCode())
    }

    @Test
    fun `allows requests with a valid api key`() {
        val uri = java.net.URI.create("$baseUrl/admin/users")
        val request = java.net.http.HttpRequest.newBuilder(uri)
            .method("POST", java.net.http.HttpRequest.BodyPublishers.noBody())
            .header("X-API-Key", "sk_live_123456789")
            .build()

        val response = java.net.http.HttpClient.newHttpClient()
            .send(request, java.net.http.HttpResponse.BodyHandlers.ofString())

        assertEquals(200, response.statusCode())
    }
}
```
