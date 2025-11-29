// dgb! macro
fn main() {
    let seasons = ["Spring", "Summer", "Fall", "Winter"];
    println!("{seasons:#?}"); // Pretty-print the array of seasons

    dbg!(seasons); // Debug-print the array of seasons
}