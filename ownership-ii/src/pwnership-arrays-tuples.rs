// Ownership with Arrays and Tuples
fn main() {
    let registrations = [true, false, true];
    let first = registrations[0]; // Copying a boolean value
    println!("First registration status: {}", first);
    println!("All registrations: {:?}", registrations);

    let langs = [String::from("Rust"), String::from("JavaScript")];
    // let first = langs[0]; // The String type does not implement the Copy trait in arrays
    // let first = langs[0].clone(); // Cloning the String to avoid ownership issues
    let first = &langs[0]; // Borrowing the String to avoid ownership issues
    println!("First language: {}", first);
    println!("All languages: {:?}", langs);

    let person: (String, u8) = (String::from("Alice"), 30);
    // let name = person.0; // Moving the String out of the tuple
    let name = &person.0; // Borrowing the String to avoid ownership issues
    let age = person.1; // Copying the u8 value
    println!("Name: {}, Age: {}", name, age);
    println!("Person tuple: {:?}", person);
}