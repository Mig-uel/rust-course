// Copy trait
fn main() {
    // The Copy trait mandates that a type can be copied
    // Rust primitive data types all implement the Copy trait
    let time = 2025;
    let year = time; // The value of time will be duplicated

    println!("The time is {time}. It is the year {year}");
}