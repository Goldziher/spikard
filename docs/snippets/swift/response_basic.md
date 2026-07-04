```swift
import Foundation
import Spikard

let app = App()

try app.get({ _ in
    return #"{"status":"ok"}"#
}, path: "/health")

try app.run()
```
