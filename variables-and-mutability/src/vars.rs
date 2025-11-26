fn main() {
    // Rust will infer the type of the variable based on the value assigned to it
    let apples = 50; // i32 is the default type for integer literals
    // i = integer, 32 = 32 bits
    // 32 refers to the amount of memory allocated for the variable (32 bits)

    let oranges = 14 + 6; // Rust infers that oranges is also of type i32

    // We can tell Rust that our variable is intentionally unused by prefixing its name with an underscore
    let _fruits = apples + oranges; // fruits is also inferred to be i32

    // We can interpolate variables into strings using {}
    println!("This year, my garden has {apples} apples and {oranges} oranges.");
}
