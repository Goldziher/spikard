```swift
import Foundation
import Spikard

let environment = ProcessInfo.processInfo.environment
let host = environment["SPIKARD_HOST"] ?? "127.0.0.1"
let port = UInt16(environment["SPIKARD_PORT"] ?? "8000") ?? 8000

let app = App()
try app.config(host: host, port: port)

// Keep secrets in the environment, never hard-coded.
let apiKey = environment["API_KEY"]
let databaseUrl = environment["DATABASE_URL"]

try app.get({ _ in #"{"status":"healthy"}"# }, path: "/health")

try app.run()
```
