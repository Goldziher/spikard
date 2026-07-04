```swift
import Foundation
import Spikard

let app = App()

try app.post({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
          let body = json["body"] as? [String: Any] else {
        let error: [String: Any] = ["error": "Invalid JSON body"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    guard let name = body["name"] as? String, !name.isEmpty else {
        let error: [String: Any] = ["error": "name is required and cannot be empty"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    guard let email = body["email"] as? String,
          email.contains("@"),
          email.contains(".") else {
        let error: [String: Any] = ["error": "email must be a valid email address"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    let age = body["age"] as? NSNumber
    if let age = age, age.intValue < 18 {
        let error: [String: Any] = ["error": "age must be 18 or older"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    let response: [String: Any] = [
        "id": 1,
        "name": name,
        "email": email,
        "validated": true
    ]
    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/users")

try app.run()
```
