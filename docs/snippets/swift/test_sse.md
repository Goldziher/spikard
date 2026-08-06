```swift
import XCTest
import Foundation
@testable import Spikard

func notificationsHandler(_ requestJson: String) -> String {
    let events = (0..<3).map { ["eventType": "message", "data": ["count": $0]] as [String: Any] }
    let response: [String: Any] = ["stream": events]
    let responseData = (try? JSONSerialization.data(withJSONObject: response)) ?? Data()
    return String(data: responseData, encoding: .utf8) ?? "{}"
}

final class SseStreamTests: XCTestCase {
    func testNotificationsStreamEmitsThreeEvents() throws {
        let responseJson = notificationsHandler("{}")
        let data = try XCTUnwrap(responseJson.data(using: .utf8))
        let response = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        let stream = try XCTUnwrap(response?["stream"] as? [[String: Any]])

        XCTAssertEqual(stream.count, 3)

        let firstEventData = try XCTUnwrap(stream.first?["data"] as? [String: Any])
        XCTAssertEqual(firstEventData["count"] as? Int, 0)

        let lastEventData = try XCTUnwrap(stream.last?["data"] as? [String: Any])
        XCTAssertEqual(lastEventData["count"] as? Int, 2)
    }
}
```
