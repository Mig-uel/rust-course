// Dangling References
fn main() {
    // A dangling reference is a reference that points to data that has been
    // dropped/deallocated. Rust prevents dangling references at compile time.

    // let ref = create_city(); // This line would cause a compile-time error

    let city = create_city(); // This is now valid because we return ownership
                                      // of the String instead of a reference.
    println!("City: {}", city);
}

// fn create_city() -> &String {
//   let city = String::from("New York");

//   return &city; // This will cause a compile-time error
// } //  city goes out of scope and is dropped here
// The function `create_city` attempts to return a reference to a local variable `city`// that goes out of scope when the function ends. Rust's borrow checker
// will catch this and prevent the code from compiling, thus avoiding a dangling reference.

fn create_city() -> String {
    let city = String::from("New York");

    return city; // Return the ownership of the String instead
}
