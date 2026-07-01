// swift-tools-version: 6.0
import PackageDescription

let package = Package(
  name: "E2eSwift",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  dependencies: [
    .package(path: "../../packages/swift"),
  ],
  targets: [
    .executableTarget(
      name: "Harness",
      dependencies: [.product(name: "Spikard", package: "swift")],
      path: "Sources/Harness"
    ),
    .testTarget(
      name: "SpikardE2ETests",
      dependencies: [.product(name: "Spikard", package: "swift")]
    ),
  ]
)
