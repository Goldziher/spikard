```swift
import XCTest
import Foundation
@testable import Spikard
@testable import RustBridge

final class BackgroundTaskTests: XCTestCase {
    func testUploadHandlerAcceptsAndReportsProcessing() {
        let requestJson = #"{"fileId":"file-123"}"#

        let responseJson = uploadHandler(requestJson)
        let data = responseJson.data(using: .utf8) ?? Data()
        let response = try? JSONSerialization.jsonObject(with: data) as? [String: Any]

        XCTAssertEqual(response?["status"] as? String, "processing")
        XCTAssertEqual(response?["fileId"] as? String, "file-123")
    }

    func testRunWithRecoveryRetriesOnTransientFailure() {
        let expectation = expectation(description: "job eventually succeeds")
        var attempts = 0
        let metadata = BackgroundJobMetadata(name: "test-job", requestId: "req-1")

        runWithRecovery(metadata: metadata, maxRetries: 3) {
            attempts += 1
            if attempts < 2 {
                throw BackgroundJobError.temporary("not ready yet")
            }
            expectation.fulfill()
        }

        wait(for: [expectation], timeout: 5)
        XCTAssertEqual(attempts, 2)
    }
}
```
