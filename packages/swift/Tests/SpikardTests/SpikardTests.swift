import XCTest

@testable import Spikard

final class SpikardTests: XCTestCase {
  /// Round-trips the generated `FieldErrorSpec` DTO through `JSONEncoder`/`JSONDecoder`,
  /// so a broken `Codable` conformance or a field that silently stops encoding fails
  /// `swift test` immediately instead of shipping green with a suite that asserts
  /// nothing about the generated API. Create-only scaffold seed. ~keep
  func testFieldErrorSpecRoundTripsThroughJSON() throws {
    let original = FieldErrorSpec(path: "alef-scaffold", message: "alef-scaffold")
    let data = try JSONEncoder().encode(original)
    let decoded = try JSONDecoder().decode(FieldErrorSpec.self, from: data)
    XCTAssertEqual(decoded, original)
  }
}
