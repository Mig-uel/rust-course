// Reading and Writing Array Elements
fn main() {
    let mut season = ["Spring", "Summer", "Fall", "Winter"];

    let first = season[0];
    println!("The first season is: {}", first);

    let second = season[1];
    println!("The second season is: {}", second);

    season[2] = "Autumn";
    println!("The third season is: {}", season[2]);
}
