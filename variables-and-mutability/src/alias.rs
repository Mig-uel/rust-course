// A type alias is a way to create a new name for an existing type.
// It does not create a new type, but rather a new name for an existing type.
// Type aliases are created using the `type` keyword, followed by the new name
// and the existing type.
type Meters = i32;

fn main() {
    // The benefit for using type aliases is to improve code readability
    // and to make it easier to change the underlying type in one place
    
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;

    println!("A one mile race is {mile_race_length} meters long.");
    println!("A two mile race is {two_mile_race_length} meters long.");
}