```swift
import Foundation
import Spikard

let app = App()

/// Guards a route by checking the `authorization` header embedded in the
/// request envelope before running the real handler logic.
func requireBearerToken(_ requestJson: String) -> String {
    guard let data = requestJson.data(using: .utf8),
          let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
          let headers = json["headers"] as? [String: String],
          let authorization = headers["authorization"],
          authorization == "Bearer dev-token" else {
        let problem: [String: Any] = [
            "type": "about:blank",
            "title": "Unauthorized",
            "status": 401,
            "detail": "Missing or invalid bearer token",
        ]
        let data = (try? JSONSerialization.data(withJSONObject: problem)) ?? Data()
        return String(data: data, encoding: .utf8) ?? #"{"error":"unauthorized"}"#
    }

    let response: [String: Any] = ["message": "Access granted"]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

try app.get(requireBearerToken, path: "/protected")

try app.run()
```
