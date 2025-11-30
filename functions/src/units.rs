// The Unit as a Return Type
fn main() {
    let res: () = (); // This is an example of the unit type
    // This is the default return type of functions that do not return a value either
    // explicitly or implicitly.

    let result = mystery(); // The return type of `mystery` is `()`
    println!("The result of mystery() is: {result:?}"); // Prints: The result of mystery() is: ()
}

fn mystery() {} // This function returns the unit type `()`
