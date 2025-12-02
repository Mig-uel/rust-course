/*
  Immutable and Mutable Reference Parameters
*/
fn main() {
    let mut current_meal = String::new();

    add_flour(&mut current_meal); // Passing in a mutable reference

    show_meal(&current_meal); // Passing in the meal reference/address
}

// There are 4 possible options to defining a parameter
// meal: String // Full ownership, moves to the param variable, cannot modify
// mut meal: String // Full ownership, moves to the param variable, can modify
// meal: &String // Not a value, a reference to a value, cannot modify
// mut meal: &mut String // Not a value, a reference to a value, can modify

// The `meal` param is also a reference but a mutable reference
// Meaning we can borrow and modify the data at the address
fn add_flour(meal: &mut String) -> () {
    meal.push_str("Add flour");
}

// The `meal` param is not the value but a reference to the value
// Also called an immutable reference
fn show_meal(meal: &String) {
  println!("Meal steps: {meal}");
}