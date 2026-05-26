// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/Goldziher/spikard.git", from: "0.15.6-rc.7"),
    ],
    targets: [
        .testTarget(
            name: "SpikardE2ETests",
            dependencies: [.product(name: "Spikard", package: "spikard")]
        ),
    ]
)
