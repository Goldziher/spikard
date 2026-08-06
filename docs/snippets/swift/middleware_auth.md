```swift
import Foundation
import Spikard
import RustBridge

// JWT verification settings understood by the Rust core. The Swift
// binding does not yet expose a way to wire `JwtConfig` into a running
// `App` (only `App.config(host:port:)` is public today), so this shows
// the shape of the config object as documented by the binding types.
let jwtConfig = JwtConfig(
    secret: ProcessInfo.processInfo.environment["JWT_SECRET"] ?? "dev-secret",
    algorithm: "HS256",
    audience: ["spikard-api"],
    issuer: "spikard-auth",
    leeway: 30
)

let app = App()

/// Reads the claims that the Rust core attaches to the request envelope
/// once JWT verification succeeds, and returns them to the caller.
func whoAmI(_ requestJson: String) -> String {
    guard let data = requestJson.data(using: .utf8),
          let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
          let claims = json["claims"] as? [String: Any],
          let subject = claims["sub"] as? String else {
        return #"{"error":"missing claims"}"#
    }

    let response: [String: Any] = ["userId": subject, "algorithm": jwtConfig.algorithm]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

try app.get(whoAmI, path: "/me")

try app.run()
```
