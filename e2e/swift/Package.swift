// swift-tools-version: 6.0
import PackageDescription

let package = Package(
  name: "E2eSwift",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  dependencies: [
    .package(name: "Spikard", path: "../../packages/swift"),
  ],
  targets: [
    .testTarget(
      name: "SpikardE2ETests",
      dependencies: [.product(name: "Spikard", package: "Spikard")]
    ),
  ]
)
