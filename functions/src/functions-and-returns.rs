fn main() {
    open_store("Brooklyn");
    bake_pizza(1, "pepperoni");
    swim_in_profits();

    let i = 10;
    let res = square(i);
    println!("The square of {i} is: {res}");
}

fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}!");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} delicious {topping} pizza(s)!");
}

fn swim_in_profits() {
    println!("So much $$$, so little time!");
}

fn square(number: i32) -> i32 {
    number * number // Implicit return; do not use a semicolon
}
