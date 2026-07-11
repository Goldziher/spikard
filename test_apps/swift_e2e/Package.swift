// swift-tools-version: 6.0
import PackageDescription

let package = Package(
  name: "E2eSwift",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  dependencies: [
    .package(url: "https://github.com/Goldziher/spikard", branch: "release/swift/0.17.0-rc.3"),
  ],
  targets: [
    .executableTarget(
      name: "Harness",
      dependencies: [.product(name: "Spikard", package: "spikard")],
      path: "Sources/Harness"
    ),
    .testTarget(
      name: "SpikardE2ETests",
      dependencies: [.product(name: "Spikard", package: "spikard")]
    ),
  ]
)
