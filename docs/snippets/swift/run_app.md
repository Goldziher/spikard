```swift
import Foundation
import Spikard

@main
struct App {
    static func main() throws {
        let app = Spikard.App()

        try app.get({ _ in
            return #"{"status":"healthy"}"#
        }, path: "/health")

        print("Starting server...")
        try app.run()
    }
}
```
