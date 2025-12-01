// `clone` trait/method
fn main() {
    let person = String::from("Alice");

    // Another benefit of ownership is the required use of the `clone` method to create deep copies of data. Rust wants to avoid unintended data duplication. Thus, if we want to create a deep copy of `person`, we must explicitly call the `clone` method.

    let genius = person.clone(); // This creates a deep copy of `person`.

    // The clone method is actually a requirement of a trait called `Clone`. Types that implement the `Clone` trait provide a way to create deep copies of their instances.

    println!("Person: {}, Genius: {}", person, genius);
    // There is no move here; both `person` and `genius` are valid.
    // The cost of the clone method is that we have doubled the memory usage since we have two separate instances of the data.
    // We generally want to avoid using `clone` unless absolutely necessary.
}