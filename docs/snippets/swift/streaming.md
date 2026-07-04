```swift
import Foundation
import Spikard

let app = App()

try app.get({ request in
    let chunks = [
        ["index": 1, "data": "chunk-1"],
        ["index": 2, "data": "chunk-2"],
        ["index": 3, "data": "chunk-3"]
    ]

    let response: [String: Any] = [
        "stream": chunks
    ]

    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/stream")

try app.run()
```
