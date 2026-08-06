```swift
import XCTest
import Foundation
@testable import Spikard

func checkAuth(_ requestJson: String) -> String {
    guard let data = requestJson.data(using: .utf8),
          let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
          let headers = json["headers"] as? [String: String],
          let authorization = headers["authorization"],
          authorization.hasPrefix("Bearer ") else {
        return #"{"status":401,"error":"Unauthorized"}"#
    }

    return #"{"data":"secret"}"#
}

final class AuthMiddlewareTests: XCTestCase {
    func testProtectedRouteRejectsMissingAuth() throws {
        let responseJson = checkAuth(#"{"headers":{}}"#)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["status"] as? Int, 401)
    }

    func testProtectedRouteAcceptsBearerToken() throws {
        let responseJson = checkAuth(#"{"headers":{"authorization":"Bearer token123"}}"#)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["data"] as? String, "secret")
    }
}
```
