fn main() {
    let season = "summer";

    // if season == "summer" {
    //     println!("School's out!");
    // } else if season == "winter" {
    //     println!("It's cold outside!");
    // } else {
    //     println!("It's a regular season.");
    // }

    match season {
        "summer" => println!("School's out!"),
        "winter" => println!("It's cold outside!"),
        _ => println!("It's a regular season."), // catch-all case
    }
}
