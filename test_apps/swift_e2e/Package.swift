// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/Goldziher/spikard", from: "0.16.0-rc.2"),
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
