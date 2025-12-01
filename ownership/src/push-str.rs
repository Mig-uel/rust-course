// The `push_str` Method
fn main() {
    let mut name = String::from("Miguel");
    // The String's text content will live on the heap
    // The stack's entry will hold 3 pieces of data:
    // Reference, Length, and Capacity

    println!("{}, {}, {}", name, name.len(), name.capacity());

    // Pushing to a String
    name.push_str(" Dolores");
    println!("{}, {}, {}", name, name.len(), name.capacity());
}
