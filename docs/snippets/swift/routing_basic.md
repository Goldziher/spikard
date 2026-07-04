```swift
import Foundation
import Spikard

let app = App()

try app.get({ _ in
    return #"{"status":"ok"}"#
}, path: "/health")

try app.post({ request in
    return request
}, path: "/users")

try app.get({ _ in
    return #"{"name":"Alice","email":"alice@example.com"}"#
}, path: "/users/profile")

try app.run()
```
