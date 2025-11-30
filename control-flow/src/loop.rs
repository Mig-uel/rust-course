// Loop and break keywords
fn main() {
    let mut seconds = 10;

    loop {
      println!("{}!", seconds);
      seconds -= 1;

      if seconds == 0 {
          break;
    }
  }
}