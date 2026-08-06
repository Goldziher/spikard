```swift
import Foundation
import Spikard
import RustBridge

enum BackgroundJobError: Error {
    case temporary(String)
    case permanent(String)
}

/// Retries a background unit of work with exponential backoff, tagging
/// each attempt with `BackgroundJobMetadata` so failures can be traced
/// back to the originating request.
func runWithRecovery(
    metadata: BackgroundJobMetadata,
    maxRetries: Int = 5,
    operation: @escaping () throws -> Void
) {
    var attempt = 0

    func attemptOnce() {
        do {
            try operation()
        } catch BackgroundJobError.permanent(let reason) {
            print("job \(metadata.name) failed permanently: \(reason)")
            sendToDeadLetterQueue(metadata: metadata, reason: reason)
        } catch {
            attempt += 1
            guard attempt <= maxRetries else {
                sendToDeadLetterQueue(metadata: metadata, reason: "\(error)")
                return
            }
            let delaySeconds = min(pow(2.0, Double(attempt)), 60.0)
            DispatchQueue.global(qos: .utility).asyncAfter(deadline: .now() + delaySeconds) {
                attemptOnce()
            }
        }
    }

    attemptOnce()
}

func sendToDeadLetterQueue(metadata: BackgroundJobMetadata, reason: String) {
    print("dead-lettering job \(metadata.name) (request \(metadata.requestId ?? "-")): \(reason)")
}
```
