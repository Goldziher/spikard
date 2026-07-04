```swift
import Foundation
import Spikard

let app = App()

try app.post({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
        return #"{"error":"Invalid JSON"}"#
    }

    // Validate required fields
    guard let id = json["id"] as? String,
          !id.isEmpty else {
        return #"{"error":"id is required"}"#
    }

    guard let amount = json["amount"] as? NSNumber,
          amount.doubleValue > 0 else {
        return #"{"error":"amount must be positive"}"#
    }

    // Valid payment
    let response: [String: Any] = [
        "id": id,
        "amount": amount,
        "status": "processed"
    ]

    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/payments")

try app.run()
```
