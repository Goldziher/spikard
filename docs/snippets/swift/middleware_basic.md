```swift
import Foundation
import Spikard

let app = App()

/// Logs the incoming method and path before delegating to the real
/// handler body. Swift handlers are plain `(String) -> String` closures,
/// so cross-cutting logic is composed by wrapping the handler itself.
func withRequestLogging(_ path: String, _ handler: @escaping (String) -> String) -> (String) -> String {
    return { requestJson in
        print("request received for \(path)")
        return handler(requestJson)
    }
}

func helloHandler(_ requestJson: String) -> String {
    return #"{"message":"Hello, World!"}"#
}

try app.get(withRequestLogging("/hello", helloHandler), path: "/hello")

try app.run()
```
