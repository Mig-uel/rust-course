// Match statements
fn main() {
    let eval = true;

    match eval {
        // A pattern or an arm is one possible value to match against
        true => println!("It's true!"),
        false => println!("It's false!")
    }

    // You can also assign the result of a match to a variable
    let value = match eval {
        true => 20,
        false => 40
    };

    println!("The value is: {}", value);
}