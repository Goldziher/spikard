// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/Goldziher/spikard.git", from: "0.14.0"),
    ],
    targets: [
        .testTarget(
            name: "SpikardE2ETests",
            dependencies: [.product(name: "Spikard", package: "spikard")]
        ),
    ]
)
