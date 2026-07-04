```swift
import Foundation
import Spikard

let app = App()

try app.post({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
        return #"{"error":"Invalid JSON"}"#
    }

    let name = json["name"] as? String ?? "Unknown"
    let email = json["email"] as? String ?? "unknown@example.com"

    let response: [String: Any] = [
        "id": 1,
        "name": name,
        "email": email
    ]

    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/users")

try app.run()
```
