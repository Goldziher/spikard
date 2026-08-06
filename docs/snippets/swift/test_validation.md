```swift
import XCTest
import Foundation
@testable import Spikard

func createUserHandler(_ requestJson: String) -> String {
    guard let data = requestJson.data(using: .utf8),
          let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
          let name = json["name"] as? String,
          let age = json["age"] as? Int else {
        let problem: [String: Any] = [
            "title": "Validation Failed",
            "status": 400,
            "detail": "'age' must be an integer",
        ]
        let encoded = (try? JSONSerialization.data(withJSONObject: problem)) ?? Data()
        return String(data: encoded, encoding: .utf8) ?? #"{"status":400}"#
    }

    let response: [String: Any] = ["name": name, "age": age]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

final class ValidationFailureTests: XCTestCase {
    func testCreateUserRejectsNonNumericAge() throws {
        let requestJson = #"{"name":"Bob","age":"not a number"}"#

        let responseJson = createUserHandler(requestJson)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["status"] as? Int, 400)
        let detail = try XCTUnwrap(response?["detail"] as? String)
        XCTAssertTrue(detail.lowercased().contains("age"))
    }
}
```
