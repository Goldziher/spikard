```swift
import Foundation
import Spikard

// NOTE: the Swift binding's `App.Method` surface currently covers only
// standard HTTP verbs (GET/POST/PUT/PATCH/DELETE/HEAD/OPTIONS/CONNECT/
// TRACE) — there is no `App.websocket(...)` registration yet. Until
// that lands, an SSE endpoint is the closest supported primitive for
// pushing events to a connected client; inbound messages are accepted
// over a regular POST route.
let app = App()

func echoInbound(_ requestJson: String) -> String {
    let data = requestJson.data(using: .utf8) ?? Data()
    let json = (try? JSONSerialization.jsonObject(with: data) as? [String: Any]) ?? [:]
    let message = json?["message"] ?? NSNull()

    let response: [String: Any] = ["echo": message]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

try app.post(echoInbound, path: "/ws/echo")

try app.run()
```
