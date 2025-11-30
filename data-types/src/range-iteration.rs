// Range and Range Iteration
fn main() {
    // A range is a sequence/interval of consecutive values.
    let range = 1..5; // This represents the numbers 1, 2, 3, and 4 (5 is exclusive)

    let month_days = 1..=31; // Represents days in a month from 1 to 31
    println!("{month_days:?}"); // Range implements Debug trait but not Display trait

    for day in month_days { // day: is a arbitrary variable name; can be named anything
        println!("{day} ");
    }

    let letters = 'a'..='z'; // Represents letters from 'a' to 'z' (z is inclusive)

    // Iterating over the range of letters
    for letter in letters { // letter: is an arbitrary variable name; can be named anything
        println!("{letter}");
    }

    let colors = ["red", "green", "blue"];

    for color in colors { // color: is an arbitrary variable name; can be named anything
        println!("{color}");
    }
}