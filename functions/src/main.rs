fn main() {
    apply_to_jobs(5, "engineer");

    let number = 8;
    if is_even(number) {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    let text = "amazing zebra";
    let (has_a, has_z) = alphabet(text);
    println!("Text contains 'a': {}, contains 'z': {}", has_a, has_z);
}

fn apply_to_jobs (number: i32, title: &str) {
    println!("I'm applying to {} {} jobs", number, title);
}

fn is_even(number: i32) -> bool {
  if number % 2 == 0 {
    true
  } else {
    false
  }
}

fn alphabet(text: &str) -> (bool, bool) {
  let has_letter_a = text.contains('a');
  let has_letter_z = text.contains('z');
  (has_letter_a, has_letter_z)
}