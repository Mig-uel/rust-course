// Blocks in functions can return values

fn main() {
  let multuplier = 3;

    let calc = {
      let value = 5 + 4;
      value * multuplier // Works like a return statement in a function
    };

    println!("The result is: {}", calc);
}