// The `Copy` trait with Reference Types
fn main() {
    // Stack types like integers implement the `Copy` trait
    // Meaning they can be copied rather than moved

    // A reference is still a type and references implement the `Copy` trait

    let ice_cream = "Chocolate"; // `&str` is a reference type
    let dessert = ice_cream; // `ice_cream` is copied, not moved

    // There is no movement of ownership here

    println!("I'd like some {} ice cream.", dessert);
}