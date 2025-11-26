// A compiler directive is a special instruction that provides guidance to the Rust compiler.
// Compiler directives can be used to enable or disable certain features,
// suppress warnings, or modify the behavior of the compiler in specific ways.
// They are typically written as attributes using the `#![attribute_name]` syntax
// at the top of a Rust source file or module.
// Basically, metadata for the compiler and there are many different compiler
// directives available in Rust.

// Directives can be applied at different levels, such as crate-level,
// module-level, or item-level, depending on their intended scope.
#![allow(unused_variables)] // This directive is at the crate-level
// Notice the exclamation mark (!) indicating crate-level scope meaning
// that it applies to the entire crate (project).

// #[allow(unused_variables)] // This directive is at the item-level (function-level)
fn main() {
    type Meters = i32;

    // #[allow(unused_variables)] // This directive is at the block-level (inside a block)
    let mile_race_length: Meters = 1600;
    
    let two_mile_race_length: Meters = 3200;
}