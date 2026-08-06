```swift
import Foundation
import Spikard

let app = App()

func jobHealthHandler(_ requestJson: String) -> String {
    let response: [String: Any] = [
        "activeJobs": BackgroundJobRegistry.shared.activeCount(),
        "queuedJobs": BackgroundJobRegistry.shared.queuedCount(),
    ]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

func jobStatusHandler(_ requestJson: String) -> String {
    let data = requestJson.data(using: .utf8) ?? Data()
    let json = (try? JSONSerialization.jsonObject(with: data) as? [String: Any]) ?? [:]
    let pathParams = (json?["pathParams"] as? [String: Any]) ?? [:]
    let taskId = (pathParams["taskId"] as? String) ?? ""

    let response: [String: Any] = [
        "taskId": taskId,
        "status": BackgroundJobRegistry.shared.status(for: taskId),
    ]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

/// Minimal in-process registry standing in for a real job tracker
/// (e.g. backed by Redis) in these examples.
final class BackgroundJobRegistry: @unchecked Sendable {
    static let shared = BackgroundJobRegistry()

    func activeCount() -> Int { 0 }
    func queuedCount() -> Int { 0 }
    func status(for taskId: String) -> String { "unknown" }
}

try app.get(jobHealthHandler, path: "/health/jobs")
try app.get(jobStatusHandler, path: "/jobs/{taskId}/status")

try app.run()
```
