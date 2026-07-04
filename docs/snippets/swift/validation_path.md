```swift
import Foundation
import Spikard

let app = App()

try app.get({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
          let pathParams = json["pathParams"] as? [String: Any],
          let userId = pathParams["id"] as? String else {
        let error: [String: Any] = ["error": "Missing or invalid user ID"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    guard let id = Int(userId), id > 0 else {
        let error: [String: Any] = ["error": "User ID must be a positive integer"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    let response: [String: Any] = [
        "id": id,
        "name": "User \(id)"
    ]
    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/users/{id:int}")

try app.run()
```
