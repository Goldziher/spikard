```kotlin
package dev.spikard.docs

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

/**
 * Exercises the JWT auth stack configured in auth_middleware.md against a running Spikard app.
 * Kotlin/JVM has no standalone "middleware function" to unit test in isolation — auth is wired
 * through [dev.spikard.ServerConfig], so it is verified the same way the generated e2e suite
 * verifies it: black-box HTTP requests against a live server.
 */
class AuthMiddlewareTest {

    companion object {
        private val baseUrl = System.getenv("SUT_URL") ?: "http://127.0.0.1:8007"
    }

    @Test
    fun `rejects requests without a bearer token`() {
        val uri = java.net.URI.create("$baseUrl/admin/users")
        val request = java.net.http.HttpRequest.newBuilder(uri)
            .method("POST", java.net.http.HttpRequest.BodyPublishers.noBody())
            .build()

        val response = java.net.http.HttpClient.newHttpClient()
            .send(request, java.net.http.HttpResponse.BodyHandlers.ofString())

        assertEquals(401, response.statusCode())
    }

    @Test
    fun `allows requests with a valid bearer token`() {
        val uri = java.net.URI.create("$baseUrl/admin/users")
        val request = java.net.http.HttpRequest.newBuilder(uri)
            .method("POST", java.net.http.HttpRequest.BodyPublishers.noBody())
            .header("Authorization", "Bearer valid-jwt-token")
            .build()

        val response = java.net.http.HttpClient.newHttpClient()
            .send(request, java.net.http.HttpResponse.BodyHandlers.ofString())

        assertEquals(200, response.statusCode())
    }
}
```
