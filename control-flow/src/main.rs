fn main() {
    let res = color_to_number("blue");
    println!("{}", res);

    let fact = factorial(5);
    println!("{}", fact)
}

fn color_to_number(color: &str) -> i32 {
    // if color == "red" {
    //   1
    // } else if color == "green" {
    //   2
    // } else if color == "blue" {
    //   3
    // } else {
    //   0
    // }

    match color {
      "red" => 1,
      "green" => 2,
      "blue" => 3,
      _ => 0
    }
}

fn factorial(num: i32) -> i32 {
  if num == 1 {
    return 1;
  }

  return factorial(num - 1) * num;
}