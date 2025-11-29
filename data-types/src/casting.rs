fn main() {
    // Casting types with the 'as' keyword
    let miles_away = 50000000;

    let miles_away_i8 = miles_away as i8;

    // It is important to note that casting a large integer to a smaller integer type can lead to overflow.
    // In this case, 50000000 exceeds the maximum value that an i8 can hold (which is 127),
    // resulting in a wrap-around effect.
    // The resulting value will not be what one might intuitively expect.
    // However, casting a smaller integer to a larger integer type is safe and does not lead to overflow.

    println!("Miles away (i8): {}", miles_away_i8); // Output will be -128 due to overflow

    let miles_away = 100.329032;
    let miles_away_f32 = miles_away as f32;

    println!("Miles away (f32): {}", miles_away_f32); // Output will be 100.32903

    let miles_away_int = miles_away as i32;
    println!("Miles away (i32): {}", miles_away_int); // Output will be 100
}
