// We can use the debugger to step through this recursive countdown function.
// WQE can set breakpoints, inspect variables, and observe the call stack
// as the function counts down to liftoff.

fn main() {
    countdown(5);
    countdown(5);
    countdown(5);
}

fn countdown(seconds: u32) {
    if seconds == 0 {
        println!("Liftoff!");
        return;
    }
    println!("{}", seconds);
    countdown(seconds - 1);
}