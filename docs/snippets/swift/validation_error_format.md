```swift
import Foundation
import Spikard

let app = App()

try app.post({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
          let body = json["body"] as? [String: Any] else {
        let errorJson = """
        {
            "typeUri": "https://spikard.dev/errors/validation",
            "title": "Validation Error",
            "status": 400,
            "detail": "Invalid JSON body"
        }
        """
        return errorJson
    }

    if !(body["email"] is String) || body["email"] == nil {
        let errorJson = """
        {
            "typeUri": "https://spikard.dev/errors/validation",
            "title": "Validation Error",
            "status": 422,
            "detail": "Email field is required",
            "errors": [
                {
                    "field": "email",
                    "message": "must be a string"
                }
            ]
        }
        """
        return errorJson
    }

    let response: [String: Any] = [
        "valid": true,
        "message": "Validation passed"
    ]
    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/validate")

try app.run()
```
