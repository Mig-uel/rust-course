/*
 * A constant is a value that is bound to a name and is not allowed to change.
 * Constants are always immutable.
*/

// We declare a constant using the `const` keyword, followed by the name of the constant,
// the type of the constant, and its value. 
//By convention, constant names are written in uppercase with 
// underscores separating words.
const TAX_RATE: f32 = 0.07;
// Constants must have their types explicitly annotated.
// They can be declared in any scope, including the global scope.
// Constants can only be set to a constant expression,
// which means they can only be assigned values that are known at compile time.

fn main() {
  let income: i32 = 1000000;

  println!("My income is ${income} and my tax rate is {TAX_RATE}%");
}