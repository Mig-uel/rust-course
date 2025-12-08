// Ownership with Immutable and Mutable References
fn main() {
    // Because we can have any number of immutable references at a time,
    // it is safe to create a FULL copy of the reference each time.
    // A reference is a cheap pointer (address in memory) to the actual data,
    // so copying the reference itself is inexpensive.
    // Remember, references implement the Copy trait.

    let coffee: String = String::from("coffee");
    let a = &coffee;
    let b = a;

    print!("{} and {}", a, b);

    // In comparision, mutable references DO NOT implement the Copy trait.
    // Remember, we can only have ONE mutable reference at a time.
}