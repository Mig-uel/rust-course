// Return Values II
// There has to be a better way of returning or using
// values than moving ownership around all the time.
fn main() {
    let mut curr_meal = String::new();

    curr_meal = add_flour(curr_meal); // curr_meal is moved here
    add_sugar();
}

fn add_flour(mut meal: String) -> String {
  meal.push_str("Add flour");
  meal
}

fn add_sugar() {
    
}