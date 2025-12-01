fn main() {
    let is_concert = false;
    let is_event = is_concert; // Rust will not move the boolean value because it implements the Copy trait

    println!("Is it a concert? {}", is_concert);
    println!("Is it a event? {}", is_event);

    let sushi = "Salmon";
    let dinner = sushi; // &str also implements the Copy trait
    println!("Sushi for dinner: {}", sushi);
    println!("Dinner choice: {}", dinner);

    let sush = String::from("Tuna");
    let dindin = sush; // String does not implement the Copy trait, so ownership is moved
    // println!("Sush for dinner: {}", sush); // This line would cause a compile-time error
    println!("Dinner choice: {}", dindin);

    eat_meal(dindin); // dindin is moved here
    // println!("Dinner choice after eating: {}", dindin); // This line would cause a compile-time error
}

fn eat_meal(mut meal: String) {
    meal.clear();
}