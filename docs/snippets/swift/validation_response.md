```swift
import Foundation
import Spikard

let app = App()

try app.get({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
          let pathParams = json["pathParams"] as? [String: Any],
          let userIdStr = pathParams["id"] as? String,
          let userId = Int(userIdStr) else {
        let error: [String: Any] = ["error": "Invalid user ID"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    let userResponse: [String: Any] = [
        "id": userId,
        "name": "Alice Johnson",
        "email": "alice@example.com",
        "active": true,
        "createdAt": "2024-01-15T10:30:00Z"
    ]

    let headers: [String: String] = [
        "Content-Type": "application/json",
        "X-Total-Count": "1"
    ]

    let response: [String: Any] = [
        "statusCode": 200,
        "headers": headers,
        "body": userResponse
    ]

    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/users/{id:int}")

try app.run()
```
