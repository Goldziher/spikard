```swift
import Foundation
import Spikard

let app = App()

try app.get({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
          let query = json["queryParams"] as? [String: Any] else {
        let error: [String: Any] = ["error": "Invalid request"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    guard let pageStr = query["page"] as? String,
          let page = Int(pageStr), page > 0 else {
        let error: [String: Any] = ["error": "page must be a positive integer"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    guard let limitStr = query["limit"] as? String,
          let limit = Int(limitStr), limit > 0, limit <= 100 else {
        let error: [String: Any] = ["error": "limit must be between 1 and 100"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    let response: [String: Any] = [
        "page": page,
        "limit": limit,
        "items": []
    ]
    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/items")

try app.run()
```
