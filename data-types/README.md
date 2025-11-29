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
