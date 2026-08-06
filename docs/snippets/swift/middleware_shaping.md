```swift
import Foundation
import Spikard

/// Simple in-memory per-client request shaper. A production deployment
/// should back this with Redis or another shared store, and prefer the
/// Rust-core `RateLimitConfig` where request shaping can live at the
/// tower-http layer instead of inside a Swift handler.
final class RequestShaper: @unchecked Sendable {
    private let lock = NSLock()
    private var hitsByClient: [String: [Date]] = [:]
    private let windowSeconds: TimeInterval = 60
    private let maxRequests = 100

    func isRateLimited(clientId: String) -> Bool {
        lock.lock()
        defer { lock.unlock() }

        let now = Date()
        var hits = hitsByClient[clientId, default: []]
        hits = hits.filter { now.timeIntervalSince($0) < windowSeconds }

        guard hits.count < maxRequests else {
            hitsByClient[clientId] = hits
            return true
        }

        hits.append(now)
        hitsByClient[clientId] = hits
        return false
    }
}

let shaper = RequestShaper()
let app = App()

func shapedHandler(_ requestJson: String) -> String {
    let data = requestJson.data(using: .utf8) ?? Data()
    let json = (try? JSONSerialization.jsonObject(with: data) as? [String: Any]) ?? [:]
    let clientId = (json?["clientIp"] as? String) ?? "unknown"

    if shaper.isRateLimited(clientId: clientId) {
        let problem: [String: Any] = ["title": "Too Many Requests", "status": 429]
        let encoded = (try? JSONSerialization.data(withJSONObject: problem)) ?? Data()
        return String(data: encoded, encoding: .utf8) ?? #"{"status":429}"#
    }

    return #"{"status":"ok"}"#
}

try app.get(shapedHandler, path: "/shaped")

try app.run()
```
