// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    targets: [
                .binaryTarget(name: "Spikard", url: "https://github.com/Goldziher/spikard/releases/download/v0.15.6-rc.9/Spikard-rs.artifactbundle.zip", checksum: "__ALEF_SWIFT_CHECKSUM__"),
        .testTarget(
            name: "SpikardE2ETests",
            dependencies: [.target(name: "Spikard")]
        ),
    ]
)
