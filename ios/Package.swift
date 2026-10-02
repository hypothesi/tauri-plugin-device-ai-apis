// swift-tools-version:5.5
// 5.5 is the minimum that knows .iOS(.v15) — the barcode symbologies used by
// DeviceAiPlugin.swift require the iOS 15 SDK.
// The swift-tools-version declares the minimum required version of Swift build tools.

import PackageDescription

let package = Package(
    name: "tauri-plugin-device-ai-apis",
    platforms: [
        .macOS(.v10_13),
        // iOS 15: the barcode detection path uses .codabar/.gs1DataBar*
        // (VMLkBarcode symbologies introduced in iOS 15). A lower deployment
        // target fails to compile DeviceAiPlugin.swift (caught by the
        // ios-example CI job).
        .iOS(.v15),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "tauri-plugin-device-ai-apis",
            type: .static,
            targets: ["tauri-plugin-device-ai-apis"]),
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri-api")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "tauri-plugin-device-ai-apis",
            dependencies: [
                .byName(name: "Tauri")
            ],
            path: "Sources")
    ]
)
