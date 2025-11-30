// While loop

fn main() {
    let mut seconds = 21;

    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{}!", seconds);
            seconds -= 3;
            continue;
        }

        println!("{}!", seconds);
        seconds -= 1;
    }

    println!("Liftoff!");
}