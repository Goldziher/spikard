```kotlin
import dev.spikard.kt.App
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class Todo(val id: Int, val title: String, val completed: Boolean = false)

fun main() = runBlocking {
    val app = App()

    // GET all todos
    app.get({ request ->
        val todos = listOf(
            Todo(1, "Learn Kotlin", true),
            Todo(2, "Build API", false)
        )
        Json.encodeToString(todos)
    }, "/todos")

    // GET single todo
    app.get({ request ->
        val todo = Todo(1, "Learn Kotlin", true)
        Json.encodeToString(todo)
    }, "/todos/{id:int}")

    // POST create todo
    app.post({ request ->
        val todo = Json.decodeFromString<Todo>(request)
        Json.encodeToString(todo.copy(id = System.currentTimeMillis().toInt()))
    }, "/todos")

    // PUT update todo
    app.put({ request ->
        val todo = Json.decodeFromString<Todo>(request)
        Json.encodeToString(todo)
    }, "/todos/{id:int}")

    // DELETE todo
    app.delete({ request ->
        Json.encodeToString(mapOf("deleted" to true))
    }, "/todos/{id:int}")

    app.run()
}
```
