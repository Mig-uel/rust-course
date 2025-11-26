// Every Rust program must have a main function as the entry point.
fn main() {
    /*
      The main function is the entry point of the program.
    */
    /*
      In order to output text, we will use a macro called `println!`.
      A macro is similar to a function, but has some differences.
      The exclamation mark `!` indicates that it is a macro.
      A macro is techinically different from a function, but for now you can think of it as a function.
      It is a recipe, a procedure that we can use to perform a task.
      It is somebody else's code that we can use in our code.
    */
    println!("Hello, world!");
}

/*
  We can use either 'cargo fmt' or 'rustfmt' to format our code.
  'cargo fmt' is a Cargo command that formats our entire project.
  'rustfmt' is a standalone tool that formats individual files.
  Both tools use the same underlying formatting engine.
  It is recommended to use 'cargo fmt' to format our entire project.

  Library crates are packages of Rust code. They contain code that someone else has written that we can use in our own programs.
  The Rust standard library is a collection of library crates that come bundled with Rust.

  We can use 'cargo build' to compile our entire project, including any dependencies we have specified in our Cargo.toml file.
  Cargo will automatically download and compile any dependencies we have specified.

  By default, Cargo will compile our code in debug mode. Debug mode includes extra information that is useful for debugging, but it is not optimized for performance.
  We can use 'cargo build --release' to compile our code in release mode. Release mode is optimized for performance, but it does not include the extra debugging information.

  We can also use 'cargo run' to compile and run our code in one step.

  'cargo check' is a command that checks our code for errors without producing an executable. It is faster than 'cargo build' because it does not generate any output files.
*/
