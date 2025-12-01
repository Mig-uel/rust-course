// Ownership and Function Parameters in Rust
fn main() {
    // A parameter is live a variable
    // Whether a value will be copied or moved into a function's parameter
    // depends on the type of data being passed and whether that type implements the `Copy` trait
    // Again, stack types like integers implement the `Copy` trait
    // Heap types like `String` do not implement the `Copy` trait
    // So remember that with heap types ownership is moved intead of copied

    // let apples = 6; // `i32` is a stack type and implements the `Copy` trait
    // print(apples); // `apples` is copied into the function not moved/transferred

    let oranges = String::from("Oranges"); // `String` is a heap type and does not implement the `Copy` trait
    print(oranges); // `oranges` is moved into the function not copied
    // println!("I have {} oranges.", oranges); // This would cause a compile-time error because `oranges` has been moved
}

// fn print(val: i32) {
//   println!("The value is: {}", val);
// }

fn print(val: String) {
  println!("The value is: {}", val);
}