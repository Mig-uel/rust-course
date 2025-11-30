fn main() {
    countdown(5);
}

fn countdown(seconds: u32) {
    if seconds == 0 {
        println!("Liftoff!");
        return;
    }

    println!("T-minus {} seconds", seconds);

    countdown(seconds - 1);
}
