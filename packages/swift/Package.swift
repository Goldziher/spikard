// swift-tools-version: 5.9
import PackageDescription

// NOTE: Run `cargo build -p spikard-swift` before `swift build`.
// The build step generates Swift + C bridge sources; copy them into Sources/RustBridge
// and Sources/RustBridgeC before building. See BUILDING.md for the full workflow.
let package = Package(
    name: "Spikard",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    products: [
        .library(name: "Spikard", targets: ["Spikard"]),
    ],
    targets: [
        // RustBridgeC: pure C/headers target. Swift files in RustBridge import this
        // to access C types (RustStr, etc.) produced by swift-bridge.
        // publicHeadersPath: "." exposes RustBridgeC.h to dependents.
        .target(
            name: "RustBridgeC",
            path: "Sources/RustBridgeC",
            publicHeadersPath: "."
        ),
        // RustBridge: Swift wrapper around the Rust static library.
        // Depends on RustBridgeC so the generated Swift files can use the C types.
        .target(
            name: "RustBridge",
            dependencies: ["RustBridgeC"],
            path: "Sources/RustBridge",
            linkerSettings: [
                .linkedLibrary("spikard_swift"),
                .unsafeFlags(["-L../../target/debug"]),
            ]
        ),
        .target(name: "Spikard", dependencies: ["RustBridge"], path: "Sources/Spikard"),
        .testTarget(name: "SpikardTests", dependencies: ["Spikard"], path: "Tests/SpikardTests"),
    ]
)
