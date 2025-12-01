// String type
#[allow(unused_variables)]
fn main() {
    // Rust has two string types

    // String literal: "example"
    let food = "pasta"; // Not stored on stack or heap
    // It is embedded directly on the binary becuase the value is
    // already known at compile time

    // Dynamic string
    let text = String::new(); // Is mutable
    let candy = String::from("KitKat");
}