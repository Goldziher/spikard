```swift
import Foundation
import Spikard
import RustBridge

let app = App()

try app.get({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
          let pathParams = json["pathParams"] as? [String: Any],
          let userIdStr = pathParams["id"] as? String,
          let userId = Int(userIdStr) else {
        return #"{"error":"Invalid user ID"}"#
    }

    let response: [String: Any] = [
        "id": userId,
        "name": "Alice",
        "database": "connected",
        "cache": "hit"
    ]
    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/users/{id:int}")

try app.run()
```
