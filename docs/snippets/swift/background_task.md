```swift
import Foundation
import Spikard
import RustBridge

// Bounds on the in-process background queue that the Rust core drains.
let backgroundTaskConfig = BackgroundTaskConfig(
    maxQueueSize: 1024,
    maxConcurrentTasks: 8,
    drainTimeoutSecs: 30
)

let app = App()
let backgroundQueue = DispatchQueue(label: "dev.spikard.background", qos: .utility)

func processUpload(fileId: String) {
    // Heavy work happens off the request path.
    print("processing upload \(fileId)")
}

func uploadHandler(_ requestJson: String) -> String {
    let data = requestJson.data(using: .utf8) ?? Data()
    let json = (try? JSONSerialization.jsonObject(with: data) as? [String: Any]) ?? [:]
    let fileId = (json?["fileId"] as? String) ?? UUID().uuidString

    backgroundQueue.async {
        processUpload(fileId: fileId)
    }

    let response: [String: Any] = ["status": "processing", "fileId": fileId]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

try app.post(uploadHandler, path: "/upload")

try app.run()
```
