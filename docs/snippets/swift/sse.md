```swift
import Foundation
import Spikard

let app = App()

try app.get({ request in
    let events = [
        [
            "eventType": "message",
            "data": ["text": "Hello, world!"],
            "id": "1",
            "retry": 3000
        ] as [String : Any],
        [
            "eventType": "update",
            "data": ["timestamp": "2024-01-15T10:30:00Z"],
            "id": "2"
        ] as [String : Any]
    ]

    let response: [String: Any] = [
        "stream": events
    ]

    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/events")

try app.run()
```
