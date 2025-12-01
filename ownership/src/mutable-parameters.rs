// Mutable parameters
// Just like variables, function parameters are immutable by default.
// This means we cannot change the value of a parameter unless we explicitly make it mutable.
fn main() {
      let burger = String::from("Burger");

      println!("Before adding fries: {}", burger);

      add_fries(burger); // Pass ownership of `burger` to the function

      // println!("After adding fries: {}", burger); // This will cause a compile-time error because `burger` has been moved
}

fn add_fries(mut meal: String) {
  meal.push_str(" with Fries");
}