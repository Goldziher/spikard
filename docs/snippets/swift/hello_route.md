```swift
import Foundation
import Spikard

let app = App()

try app.get({ request in
    return #"{"message":"Hello, World!"}"#
}, path: "/hello")

try app.run()
```
