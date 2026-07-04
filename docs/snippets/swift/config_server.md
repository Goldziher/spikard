```swift
import Foundation
import Spikard

let app = App()

try app.config(host: "0.0.0.0", port: 8080)

try app.get({ _ in
    return #"{"status":"ok"}"#
}, path: "/health")

try app.run()
```
