```swift
import Foundation
import Spikard

let app = App()

try app.get({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
          let orderId = json["id"] as? Int else {
        return #"{"error":"Invalid request"}"#
    }

    let response: [String: Any] = [
        "id": orderId,
        "details": true
    ]
    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/orders/{order_id:int}")

try app.run()
```
