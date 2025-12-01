// `drop` Function
fn main() {
    let person = String::from("Alice");

    // We can explicitly call the `drop` function to free the memory used by `person`.
    drop(person);

    // let genius = person; // This line would cause a compile-time error because `person` has been moved.

    // After calling `drop`, `person` is no longer valid and cannot be used.
    // println!("{}", person); // This line would cause a compile-time error.
} // Behind the scenes, Rust calls `drop(person)` here to free the memory or whenever a variable goes out of scope.