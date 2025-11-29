# Data Types

- Every Rust value has a specific data type that determines what kind of data it can hold and what operations can be performed on it.
- Rust is a statically typed language, meaning that all types must be known at compile time.
- The compiler can often infer types based on context, reducing the need for explicit type annotations.
- Data types fall into several categories, including scalar types (like integers, floating-point numbers, booleans, and characters) and compound types (like tuples and arrays).

## Scalar Types

- A **scalar type** is a type that holds a single value.
- Rust has four primary scalar types:
  - **Integer types**: Represent whole numbers (e.g., `i32`, `u64`).
  - **Floating-point types**: Represent numbers with fractional parts (e.g., `f32`, `f64`).
  - **Boolean type**: Represents a value that can be either `true` or `false` (`bool`).
  - **Character type**: Represents a single Unicode scalar value (`char`).

### Signed and Unsigned Integers

- **Signed integers** can represent both positive and negative numbers (e.g., `i8`, `i16`, `i32`, `i64`, `i128`, `isize`) (the `i` stands for "integer" and indicates that the type is signed).
  - Two's complement representation is used for signed integers in Rust.
- **Unsigned integers** can only represent non-negative numbers (e.g., `u8`, `u16`, `u32`, `u64`, `u128`, `usize`) (the `u` stands for "unsigned").
  - One of the advanteges of using unsigned integers is that unsigned integers do not need to allocate a bit for the sign, allowing them to represent a larger range of positive values compared to their signed counterparts of the same bit size (more memory efficient for non-negative values).
- The size of `isize` and `usize` depends on the architecture (32-bit or 64-bit).

### Bits

Let's look at `i32` as an example of a signed integer type:

- The letter `i` indicates that it is a signed integer.
- The number `32` indicates that it uses 32 bits of memory.
- With 32 bits, `i32` can represent values from -2,147,483,648 to 2,147,483,647.
- The larger the number, the more bits are used, allowing for a wider range of values.
- A bit is the smallest unit of data in a computer and can have a value of either `0` or `1`.
- 8 bits make up 1 byte.
  - 1024 bytes make up 1 kilobyte (KB).
  - 1024 kilobytes make up 1 megabyte (MB).
  - 1024 megabytes make up 1 gigabyte (GB).
- An `i32` uses 32 bits (4 bytes) of memory. An `f64` uses 64 bits (8 bytes) of memory.

### Integer Lower and Upper Bounds

| Type   | Smallest Value                                       | Largest Value                                       |
| ------ | ---------------------------------------------------- | --------------------------------------------------- |
| `i8`   | -128                                                 | 127                                                 |
| `u8`   | 0                                                    | 255                                                 |
| `i16`  | -32,768                                              | 32,767                                              |
| `u16`  | 0                                                    | 65,535                                              |
| `i32`  | -2,147,483,648                                       | 2,147,483,647                                       |
| `u32`  | 0                                                    | 4,294,967,295                                       |
| `i64`  | -9,223,372,036,854,775,808                           | 9,223,372,036,854,775,807                           |
| `u64`  | 0                                                    | 18,446,744,073,709,551,615                          |
| `i128` | -170,141,183,460,469,231,731,687,303,715,884,105,728 | 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| `u128` | 0                                                    | 340,282,366,920,938,463,463,374,607,431,768,211,455 |

### Float Lower and Upper Bounds

| Type  | Precision            |
| ----- | -------------------- |
| `f32` | Approx. 6-7 digits   |
| `f64` | Approx. 15-16 digits |

## Compound Types

- A **compound type** can group multiple values into one type.
- Rust has two primary compound types:
  - **Tuples**: A fixed-size collection of values of different types (e.g., `(i32, f64, char)`).
  - **Arrays**: A fixed-size collection of values of the same type (e.g., `[i32; 5]`).

## Traits - Intro

- A contract is a document that people sign to agree to do something.
- Imaginve a non-specific contract with the following clause:

  - "You promise to arrive at 9 AM at a specified location."
    - A college student can promise to arrive at 9 AM for class.
    - A software engineer can promise to arrive at 9 AM for a meeting.
    - A flight can promise to arrive at 9 AM at the airport.
    - A package can promise to arrive at 9 AM at your home.
    - The situations are different but they honor the same contract.

- In Rust, a **trait** is a contract that requires that a type must implement certain behavior/methods.
- Traits establish consistency between types; methods that represent the same behavior have the same name across different types.
- When a type opts in to honoring a trait's requirements, we say that the type "implements" the trait.
- Types can vary in how they implement the behavior required by a trait, but they must provide the same methods with the same names.
- Traits are similar to interfaces in other programming languages.
- Traits enable polymorphism, allowing functions to operate on different types that implement the same trait.

### Traits - II

- A type can choose to implement a trait or multiple traits.
- A type can implement multiple traits, allowing it to exhibit various behaviors.
- Traits can be defined by the Rust standard library or by users.
- There are hundreds of traits in the Rust standard library, covering a wide range of functionalities.
- A trait is called an interface or protocol in other programming languages.
- Traits can also define default method implementations, which types can override if needed.

### The `Display` Trait I

- The `Display` requires that a type can be represented as a user-friendly, readable string.
- The `Display` trait mandates a format method that returns a string representation of the type.
- When we use the `{}` placeholder syntax in a `println!` macro, Rust looks for the `Display` trait implementation for the type being printed.
- If the type implements the `Display` trait, Rust calls the format method defined by the trait to obtain the string representation of the value.
- If the type does not implement the `Display` trait, Rust will produce a compile-time error, indicating that the type cannot be formatted using `{}`.
- Integers, floats, and booleans all implement the `Display` trait by default, allowing them to be printed using the `{}` syntax.
- Custom types can implement the `Display` trait by defining how they should be formatted as strings.

### The `Display` Trait II

- It is not always clear how a complex type should be represented as a string.
- Not all types implement the `Display` trait by default. One example is the array type.
