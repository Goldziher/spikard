```swift
import Foundation
import Spikard

let app = App()

/// Wraps a handler with a request ID that is generated (or propagated
/// from the `x-request-id` header) and echoed back in the response body
/// alongside a structured log line.
func withObservability(_ handler: @escaping (String) -> String) -> (String) -> String {
    return { requestJson in
        let start = Date()
        let data = requestJson.data(using: .utf8) ?? Data()
        let json = (try? JSONSerialization.jsonObject(with: data) as? [String: Any]) ?? [:]
        let headers = (json?["headers"] as? [String: String]) ?? [:]
        let requestId = headers["x-request-id"] ?? UUID().uuidString

        print("request_started request_id=\(requestId)")

        let responseBody = handler(requestJson)
        let durationMs = Date().timeIntervalSince(start) * 1000
        print("request_completed request_id=\(requestId) duration_ms=\(durationMs)")

        guard var responseJson = try? JSONSerialization.jsonObject(with: responseBody.data(using: .utf8) ?? Data()) as? [String: Any] else {
            return responseBody
        }
        responseJson["requestId"] = requestId
        let encoded = (try? JSONSerialization.data(withJSONObject: responseJson)) ?? Data()
        return String(data: encoded, encoding: .utf8) ?? responseBody
    }
}

func statusHandler(_ requestJson: String) -> String {
    return #"{"status":"healthy"}"#
}

try app.get(withObservability(statusHandler), path: "/status")

try app.run()
```
