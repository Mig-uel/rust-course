// Mutable Reference Restrictions
fn main() {
    // A value can only have a single mutale reference at a time

    let mut car = String::from("Red");
    let ref1 = &mut car; // Mutable reference
    ref1.push_str(" and Silver"); // At this point, `ref1` lifetime ends
    let ref3 = &mut car; // This is fine because `ref1` is not being
    // used anymore
    let ref2 = &car; // Another immutable reference

    println!("{ref2}"); // This is allowed because the compiler sees that the
    // mutable reference is not being used anywhere
}
