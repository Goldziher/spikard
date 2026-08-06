```swift
import XCTest
import Foundation
#if canImport(FoundationNetworking)
import FoundationNetworking
#endif
import Spikard

/// End-to-end tests exercise a real running `App` over HTTP, mirroring
/// the generated e2e harness that ships with Spikard: start the server
/// on a background thread, poll until it accepts connections, then use
/// `URLSession` to drive requests.
final class UserWorkflowIntegrationTests: XCTestCase {
    static var baseURL = "http://127.0.0.1:8010"

    override class func setUp() {
        super.setUp()

        let app = App()
        try? app.get({ _ in #"{"id":1,"name":"Alice"}"# }, path: "/users/1")
        try? app.post({ _ in #"{"id":1,"name":"Alice"}"# }, path: "/users")
        try? app.config(host: "127.0.0.1", port: 8010)

        Thread.detachNewThread {
            try? app.run()
        }

        let deadline = Date(timeIntervalSinceNow: 5)
        while Date() < deadline {
            if let url = URL(string: baseURL + "/users/1"),
               let data = try? Data(contentsOf: url), !data.isEmpty {
                break
            }
            usleep(100_000)
        }
    }

    func testCreateThenRetrieveUser() throws {
        var createRequest = URLRequest(url: URL(string: Self.baseURL + "/users")!)
        createRequest.httpMethod = "POST"
        createRequest.httpBody = #"{"name":"Alice"}"#.data(using: .utf8)

        let (createData, createResponse) = try synchronousDataTask(with: createRequest)
        XCTAssertEqual((createResponse as? HTTPURLResponse)?.statusCode, 200)
        let created = try JSONSerialization.jsonObject(with: createData) as? [String: Any]

        let getRequest = URLRequest(url: URL(string: Self.baseURL + "/users/\(created?["id"] ?? 1)")!)
        let (getData, getResponse) = try synchronousDataTask(with: getRequest)
        XCTAssertEqual((getResponse as? HTTPURLResponse)?.statusCode, 200)
        let retrieved = try JSONSerialization.jsonObject(with: getData) as? [String: Any]

        XCTAssertEqual(retrieved?["name"] as? String, "Alice")
    }

    private func synchronousDataTask(with request: URLRequest) throws -> (Data, URLResponse?) {
        var resultData = Data()
        var resultResponse: URLResponse?
        let semaphore = DispatchSemaphore(value: 0)

        URLSession.shared.dataTask(with: request) { data, response, _ in
            resultData = data ?? Data()
            resultResponse = response
            semaphore.signal()
        }.resume()

        semaphore.wait()
        return (resultData, resultResponse)
    }
}
```
