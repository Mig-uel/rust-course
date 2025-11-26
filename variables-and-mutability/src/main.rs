/*
 *  In Rust, variables are immutable by default. This means that once a value is bound 
 * to a variable, it cannot be changed. 
 * To make a variable mutable, you need to use the `mut` keyword when declaring it.
*/
fn main() {
  let gym_reps = 10; // Immutable variable
  println!("I plan to do {gym_reps} reps at the gym today.");

  // gym_reps = 15; // This line will cause a compile-time error because `gym_reps` is immutable

  // To make the variable mutable, use the `mut` keyword
  let mut mutable_gym_reps = 10;
  println!("I plan to do {mutable_gym_reps} reps at the gym today.");
  
  // Even though `mutable_gym_reps` is mutable, it must be assigned a value of the same type
  // It cannot change from an integer to a floating-point number
  // mutable_gym_reps = 15.3; // This line will cause a compile-time error

  mutable_gym_reps = 15; // This is valid
  println!("I have updated my plan to do {mutable_gym_reps} reps at the gym today.");
}