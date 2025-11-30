// Assigning results of control flow expressions to variables
fn main() {
    even_or_odd(17);
}

fn even_or_odd(num: i32) {
    // if num % 2 == 0 { "even" } else { "odd" }

    let result = if num % 2 == 0 { "even" } else { "odd" };
    println!("The number {} is {}", num, result);
}
