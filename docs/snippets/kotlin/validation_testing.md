```kotlin
package dev.spikard.docs

import com.fasterxml.jackson.databind.ObjectMapper
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/** Exercises every validation branch of a `POST /users` route registered with a schema. */
class UserCreationValidationTest {

    companion object {
        private val MAPPER = ObjectMapper()
        private val baseUrl = System.getenv("SUT_URL") ?: "http://127.0.0.1:8007"

        private fun postUsers(json: String): java.net.http.HttpResponse<String> {
            val uri = java.net.URI.create("$baseUrl/users")
            val request = java.net.http.HttpRequest.newBuilder(uri)
                .method("POST", java.net.http.HttpRequest.BodyPublishers.ofString(json))
                .header("Content-Type", "application/json")
                .build()
            return java.net.http.HttpClient.newHttpClient()
                .send(request, java.net.http.HttpResponse.BodyHandlers.ofString())
        }
    }

    @Test
    fun `accepts a valid request`() {
        val response = postUsers("""{"email":"test@example.com","age":25,"username":"testuser"}""")
        assertEquals(200, response.statusCode())
    }

    @Test
    fun `rejects an invalid email`() {
        val response = postUsers("""{"email":"not-an-email","age":25,"username":"testuser"}""")
        assertEquals(422, response.statusCode())
        val firstError = MAPPER.readTree(response.body()).get("errors").get(0)
        assertTrue(firstError.get("loc").toString().contains("email"))
    }

    @Test
    fun `rejects an age below the minimum`() {
        val response = postUsers("""{"email":"test@example.com","age":16,"username":"testuser"}""")
        assertEquals(422, response.statusCode())
    }

    @Test
    fun `rejects a missing required field`() {
        val response = postUsers("""{"email":"test@example.com","age":25}""")
        assertEquals(422, response.statusCode())
    }
}
```
