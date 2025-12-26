// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "SMDiskManager",
    platforms: [
        .macOS(.v13)  // macOS 13 Ventura minimum
    ],
    products: [
        .executable(
            name: "SMDiskManager",
            targets: ["SMDiskManager"]
        ),
    ],
    dependencies: [
        // Add dependencies here if needed
    ],
    targets: [
        .executableTarget(
            name: "SMDiskManager",
            dependencies: [],
            path: "Sources",
            resources: [
                .process("Resources")
            ]
        ),

        .testTarget(
            name: "SMDiskManagerTests",
            dependencies: ["SMDiskManager"]
        ),
    ]
)
