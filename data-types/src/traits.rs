// Traits & Debug
fn main() {
    let x = 5;
    println!("The value of x is: {}", x); // Uses the Display trait behind the scenes

    let seasons: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

    // Debug trait should format the output in a programmer-friendly way
    println!("The seasons of the year are: {:?}", seasons);
    // We always begin with ':' to indicate that formatting options follow
    // The ':' is used to specify formatting options
    // The '?' indicates that we want to use the Debug trait
    // println!("The seasons of the year are: {seasons:?}"); // This is also valid syntax in Rust 1.58 and later

    println!("The seasons of the year are: {:#?}", seasons);
    // The '#' adds pretty-printing to the Debug output
    // The '?' indicates that we want to use the Debug trait
}
