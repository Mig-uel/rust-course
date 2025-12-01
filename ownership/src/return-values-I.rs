// Return Values I
fn main() {
    let dessert = bake_cake();
    println!("I baked a delicious {} cake!", dessert);
}

fn bake_cake() -> String {
    String::from("Chocolate Mousse")
}