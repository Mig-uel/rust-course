// Multiple Immutable References
fn main() {
    // Rust permits any number of immutable references to a value at the same time
    // Basically, you can create as many immutable references and use them along
    // each other.
    // This is because there is no risk involved for the data to change since it is
    // immutable.
    // In other words, we can have any amount of readers but only one writer at a
    // given time.
    let car = String::from("Red");
    let ref1 = &car; // Immutable reference
    let ref2 = &car; // Another immutable reference

    println!("{ref1}, {ref1}, and {}", &car);
}