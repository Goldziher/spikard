```swift
import XCTest
import Foundation
@testable import Spikard

func createUserHandler(_ requestJson: String) -> String {
    guard let data = requestJson.data(using: .utf8),
          let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
          let name = json["name"] as? String,
          let email = json["email"] as? String else {
        return #"{"error":"invalid request"}"#
    }

    let response: [String: Any] = ["id": 1, "name": name, "email": email]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

final class UserCreationTests: XCTestCase {
    func testCreateUserReturnsSubmittedFields() throws {
        let requestJson = #"{"name":"Alice","email":"alice@example.com"}"#

        let responseJson = createUserHandler(requestJson)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["name"] as? String, "Alice")
        XCTAssertEqual(response?["email"] as? String, "alice@example.com")
    }
}
```
