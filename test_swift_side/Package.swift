// swift-tools-version: 6.1
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "test_swift_side",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "test_swift_side",
            targets: ["test_swift_side"]),
    ],
    targets: [

        .executableTarget(
            name: "test_swift_side",
            linkerSettings: [
                .unsafeFlags(["-L/workspaces/bevy_mod_eninge/test_c_lib/target/release", "-ltest_c_lib"])
            ]
        )

        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "test_swift_side"),
        .testTarget(
            name: "test_swift_sideTests",
            dependencies: ["test_swift_side"]
        ),
    ]
)
