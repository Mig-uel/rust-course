// References and Borrowing
// Remember that every value in Rust has a single owner.
// The challenge comes when multiple parts of your code need to access the same data.
fn main() {
    // For certain types, like integers, it is lightweight to copy the value.
    // But for larger data types on the heap, copying can be expensive.
    // We can instead use references to allow multiple parts of our code to access the same data without taking ownership.
    // We call the act of creating a reference "borrowing".
    // In the real world, borrowing means using something without taking ownership of it.
    // Techincally speaking, a reference is an address that points to some data on the heap.
    // For the most part, we will be creating references for data stored on the heap rather than the stack because copying stack data is inexpensive.

    // We use the ampersand (&) to create a reference also called the "borrow operator".
    let my_stack_val = 2;
    let my_int_ref = &my_stack_val; // This is an address pointing to the stack value.
    println!("The value of my_int_ref is: {}", my_int_ref);

    let my_heap_val: String = String::from("Porsche");
    let my_heap_ref: &String = &my_heap_val; // This is an address pointing to the heap value.
    println!("The value of my_heap_ref is: {}", my_heap_ref);

    // References must never outlive their referents (the data they point to).
} // All values go out of scope and are cleaned up here.
