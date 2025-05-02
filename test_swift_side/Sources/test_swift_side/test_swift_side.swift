// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

// Declare the Rust function
@_silgen_name("add_c")
func add_c(_ left: UInt64, _ right: UInt64) -> UInt64

// Call the Rust function
let result = add_c(10, 20)
print("Result from Rust: \(result)")