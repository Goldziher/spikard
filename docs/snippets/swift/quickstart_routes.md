```swift
import Foundation
import Spikard

@main
struct Server {
    static func main() throws {
        let app = App()

        try app.get({ _ in
            return #"{"status":"healthy"}"#
        }, path: "/health")

        try app.post({ request in
            // Echo the request body back
            return request
        }, path: "/users")

        try app.get({ request in
            // Parse path parameter from request
            return request
        }, path: "/users/{id:int}")

        try app.run()
    }
}
```
