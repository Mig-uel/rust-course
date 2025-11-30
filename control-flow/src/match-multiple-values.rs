fn main() {
    let num = 8;

    match num {
        // `|` acts as a logical OR operator here
        // Multiple patterns can be matched in a single arm

        // 2 | 4 | 6 | 8 | 10 => {
        //     println!("The number {} is even.", num);
        // }
        // 1 | 3 | 5 | 7 | 9 => {
        //     println!("The number {} is odd.", num);
        // }
        // _ => {
        //     println!("The number {} is neither even nor odd.", num);
        // }

        // value is an arbitrary name
        // This arm matches any value and binds it to `value`
        value if value % 2 == 0 => {
            println!("The number {} is even.", value);
        }
        val if val % 2 != 0 => {
            println!("The number {} is odd.", val);
        }
        // _ => {
        //     println!("The number {} is neither even nor odd.", num);
        // }
        // If we know that all possible cases are covered, we can use a macro like `unreachable!()`
        _ => {
            unreachable!();
        }
    }
}
