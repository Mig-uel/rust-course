// Dereference opertor (*)
fn main() {
    // To dereference means to access the data at the memory address stored in a pointer/reference.

    let my_stack_value = 10;
    let my_stack_reference = &my_stack_value; // reference to the stack value
    println!("The value of my_stack_reference is: {}", *my_stack_reference); // Take the address but follow it to get the value

    let my_heap_value = String::from("Hello, world!");
    let my_heap_reference = &my_heap_value; // reference to the heap value
    println!("The value of my_heap_reference is: {}", *my_heap_reference); // Dereference to get the String value

    // Note: In Rust, the Display trait is implemented for references, so you can often print references directly without explicit dereferencing. 
    // So is a reference a type? Yes, references are types in Rust and have their own type signatures (e.g., &i32, &String) and thus implement traits accordingly like Display.
}