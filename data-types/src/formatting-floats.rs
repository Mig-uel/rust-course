fn main () {
    // A format specifier customizes the way a value is printed.
    let pi: f64 = 3.1415926535897932384;
    println!("The value of pi is approximately: {}", pi);

    // Print pi with 3 decimal places
    println!("Pi rounded to three decimal places: {:.3}", pi);
}