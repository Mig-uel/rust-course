// String, &String, str, &str
fn main() {
    /*
      String - A dynamic string type that owns its data. It is heap-allocated and can be modified.

      &String - A reference to a String. It does not own the data and cannot modify it unless it's a mutable reference.

      str - A hardcoded, read-only piece of string data embedded directly in the binary. It is usually seen as string literals.

      &str - A reference to a str. It is commonly used for string slices, which are views into a String or string literals.
     */

    let ice_cream = "Cookies and Cream"; // &str
    println!("I like {} ice cream.", ice_cream);
}