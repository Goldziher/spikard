```swift
import XCTest
import Foundation
@testable import Spikard

final class AuthGuardTests: XCTestCase {
    func testRequireBearerTokenRejectsMissingHeader() {
        let requestJson = #"{"headers":{}}"#

        let responseJson = requireBearerToken(requestJson)
        let data = responseJson.data(using: .utf8) ?? Data()
        let response = try? JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["status"] as? Int, 401)
    }

    func testRequireBearerTokenAcceptsValidToken() {
        let requestJson = #"{"headers":{"authorization":"Bearer dev-token"}}"#

        let responseJson = requireBearerToken(requestJson)
        let data = responseJson.data(using: .utf8) ?? Data()
        let response = try? JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["message"] as? String, "Access granted")
    }
}
```
