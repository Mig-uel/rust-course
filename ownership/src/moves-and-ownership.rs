// Moves and ownership
// A move is the transfer of ownership of a value from one variable to another.
// Remember that in Rust, each value has a single owner at any given time.
fn main() {
    let time = 2025;
    let year = time; // Copy trait is implemented for integers, so this is a copy, not a move.

    let person = String::from("Miguel");
    println!("Before move: {person}");
    let genius = person; // Move occurs here, 'person' is no longer valid.
    // A heap allocated String does not implement the Copy trait, so ownership is transferred instead because if it did it would occupy more memory.

    // We cannot use 'person' anymore because its ownership has been moved to 'genius'.
    // println!("{person}"); // This line would cause a compile-time error because 'person' is no longer valid.
    println!("After move: {genius}");
}