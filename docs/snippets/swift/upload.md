```swift
import Foundation
import Spikard

let app = App()

try app.post({ request in
    guard let data = request.data(using: .utf8),
          let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
          let body = json["body"] as? [String: Any] else {
        let error: [String: Any] = ["error": "No file provided"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    guard let fileName = body["filename"] as? String else {
        let error: [String: Any] = ["error": "filename is required"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    guard let contentType = body["contentType"] as? String else {
        let error: [String: Any] = ["error": "contentType is required"]
        let errorData = try JSONSerialization.data(withJSONObject: error)
        return String(data: errorData, encoding: .utf8) ?? ""
    }

    let response: [String: Any] = [
        "filename": fileName,
        "contentType": contentType,
        "uploaded": true,
        "size": body["size"] ?? 0
    ]
    let responseData = try JSONSerialization.data(withJSONObject: response)
    return String(data: responseData, encoding: .utf8) ?? ""
}, path: "/upload")

try app.run()
```
