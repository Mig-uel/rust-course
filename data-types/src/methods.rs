fn main() {
    // A method is a function that lives on a value. It is an action we can
    // ask the value to perform for us.
    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_space: &str = "                my content              ";
    let empty_space: &str = empty_space.trim();
    println!("{}", empty_space);

    println!("{}", value.pow(2))
}
