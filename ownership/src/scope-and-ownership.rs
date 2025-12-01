// Scopes and ownership

fn main() {
    // Every Rust value has an owner
    // Includes values on the stack and heap

    let mut age = 33; // The variable becomes the owner of the value
    // Age is the owner of the value 33
    // The owner is who is responsible for cleaning up the data

    let is_handsome = true;

    // {
    //   let is_handsome = true;
    // } // is_handsome goes out of scope here

    println!("{age}");
    println!("{is_handsome}");

    // How does the owner know when to clean up data?
    // When the variable goes out of scope/block ends
} // age variable goes out of scope here
