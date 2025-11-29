fn main() {
    // A compound type is a type that can group multiple values into one type.
    // The two primitive compound types in Rust are tuples and arrays.
    // Tuples group together a fixed number of values of different types.

    // Arrays group together a fixed number of values of the same type.
    let numbers: [i8; 7] = [4, 8, 15, 16, 23, 42, 108]; // An array of 7 i8 values

    let apples = ["Granny Smith", "Fuji", "Gala"]; // An array of 3 &str values

    println!("Apples: {}", apples.len());
}