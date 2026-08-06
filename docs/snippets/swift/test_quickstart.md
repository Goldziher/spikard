```swift
import XCTest
import Foundation
@testable import Spikard

/// Handlers are plain `(String) -> String` closures, so the most direct
/// way to unit test route logic is to call the handler function itself.
func helloHandler(_ requestJson: String) -> String {
    return #"{"message":"Hello, World!"}"#
}

final class QuickstartTests: XCTestCase {
    func testHelloReturnsGreeting() throws {
        let responseJson = helloHandler("{}")
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["message"] as? String, "Hello, World!")
    }
}
```
