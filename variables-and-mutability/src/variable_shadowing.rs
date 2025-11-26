/*
 * Variable shadowing allows you to declare a new variable with the same name as a previous variable.
 * The new variable shadows the previous variable, meaning that the previous variable
 * is no longer accessible. The original variable is "replaced" by the new one
 * in the current scope.
 * This can be useful for transforming the value of a variable while keeping the same name.
*/
fn main() {
    // Let's say this comes from a user input as a string
    let grams_of_protein = "100.345";

    // We can parse the string to a floating-point number
    let grams_of_protein = 100.345;

    // And then we can round it to an integer
    let mut grams_of_protein = 100;
    grams_of_protein = 105;
}
