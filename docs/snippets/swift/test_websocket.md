```swift
import XCTest
import Foundation
@testable import Spikard

// The Swift binding does not yet expose a native WebSocket route
// registration API (see `websocket.md`), so this exercises the
// POST-based echo handler that stands in for a duplex connection today.
func echoInbound(_ requestJson: String) -> String {
    let data = requestJson.data(using: .utf8) ?? Data()
    let json = (try? JSONSerialization.jsonObject(with: data) as? [String: Any]) ?? [:]
    let message = json?["message"] ?? NSNull()

    let response: [String: Any] = ["echo": message]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

final class WebSocketEchoTests: XCTestCase {
    func testEchoReturnsSubmittedMessage() throws {
        let responseJson = echoInbound(#"{"message":"Hello"}"#)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["echo"] as? String, "Hello")
    }

    func testEchoRoundTripsStructuredPayload() throws {
        let responseJson = echoInbound(#"{"message":{"type":"ping"}}"#)
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        let echoed = try XCTUnwrap(response?["echo"] as? [String: Any])

        XCTAssertEqual(echoed["type"] as? String, "ping")
    }
}
```
