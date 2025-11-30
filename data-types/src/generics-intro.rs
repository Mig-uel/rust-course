// Intro to Generics
// A generic is a type argument that can be used to create definitions
// that work with different data types without being specific to one.

fn main() {
    // std: is the standard library namespace
    // ops: is the operations module within the standard library
    // Range: is a struct within the ops module that represents a range of values

    let month_days: std::ops::Range<u8> = 1..31; // Represents days in a month from 1 to 31
    let letters: std::ops::Range<char> = 'b'..'f'; // Represents letters from 'b' to 'f' (f is exclusive)
}
