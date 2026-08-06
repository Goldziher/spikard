```swift
import XCTest
import Foundation
@testable import Spikard

func createUserHandler(_ requestJson: String) -> String {
    guard let data = requestJson.data(using: .utf8),
          let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
          let email = json["email"] as? String, email.contains("@"),
          let age = json["age"] as? Int, age >= 18,
          let username = json["username"] as? String else {
        let problem: [String: Any] = [
            "status": 422,
            "details": [["field": "email"]],
        ]
        let encoded = (try? JSONSerialization.data(withJSONObject: problem)) ?? Data()
        return String(data: encoded, encoding: .utf8) ?? #"{"status":422}"#
    }

    let response: [String: Any] = ["email": email, "age": age, "username": username]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

final class UserCreationValidationTests: XCTestCase {
    func testValidRequestSucceeds() throws {
        let requestJson = #"{"email":"test@example.com","age":25,"username":"testuser"}"#
        let responseJson = createUserHandler(requestJson)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["username"] as? String, "testuser")
    }

    func testInvalidEmailRejected() throws {
        let requestJson = #"{"email":"not-an-email","age":25,"username":"testuser"}"#
        let responseJson = createUserHandler(requestJson)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["status"] as? Int, 422)
    }

    func testAgeBelowMinimumRejected() throws {
        let requestJson = #"{"email":"test@example.com","age":16,"username":"testuser"}"#
        let responseJson = createUserHandler(requestJson)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["status"] as? Int, 422)
    }

    func testMissingRequiredFieldRejected() throws {
        let requestJson = #"{"email":"test@example.com","age":25}"#
        let responseJson = createUserHandler(requestJson)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["status"] as? Int, 422)
    }
}
```
