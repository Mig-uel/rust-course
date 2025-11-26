fn main() {
  let coffee_price = 3.5;

  // Scope block
  {
    println!("The price of coffee inside the block is ${coffee_price}");

    let coffee_price = 4.0; // Shadowing the outer variable
    let cookie_price = 2.0;
    println!("The new price of coffee inside the block is ${coffee_price}");
    println!("The price of cookie inside the block is ${cookie_price}");
  }

  println!("The price of coffee outside the block is ${coffee_price}");
  // println!("The price of cookie outside the block is ${cookie_price}"); // This line would cause a compile-time error
}