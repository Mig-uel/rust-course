# Ownership

## Intro to Ownership

### Memory

- **Ownership** is a set of rules that governs how a Rust program manages memory.
  - In other words, it is a set of rules that the Rust compiler checks at compile time
    to ensure the program will be free of memory errors.
- Ownership is a Rust compiler feature not a runtime feature. It exists for the benfit of us
  as programmers to help us write safe and efficient code.
- Memory refers to the computer's RAM where data is stored while a program is running.
- It's ideal to free memory when it's no longer needed to avoid memory leaks.
- In languages without ownership, programmers manually manage memory allocation and deallocation, which can lead to errors like dangling pointers, double frees, and memory leaks.
- Rust's ownership system automates memory management, ensuring safety and efficiency without a garbage collector.

### Manual Memory Management

- In languages like **C** and **C++**, programmers are responsible for allocating (requesting memory) and deallocating (freeing memory/releasing memory back to the system) memory.
- Unfortunately, human beings are prone to making mistakes like:
  - Forgetting to deallocate memory that has been allocated, leading to **memory leaks**.
  - Trying to deallocate memory that has already been deallocated, leading to **double frees**.

### Automatic Garbage Collection

- Languages like **Java**, **Python**, **JavaScript**, and many others implement a tool called the **garbage collector** to automatically manage memory for the programmer.
- The garbage collector periodically looks for memory that is no longer being used by the program and frees it automatically. It "automates" the cleanup process.
- The problem with garbage collection is that it can introduce performance overhead like occupying additional memory and causing unpredictable pauses in program execution when the garbage collector runs.

### The Ownership System

- Rust introduces a new paradigm for memory management called the **ownership system**.
- **Ownership** is a set of rules that the Rust compiler checks at compile time to ensure memory safety without needing a garbage collector.
- The Rust compiler does not compile the program if an ownership rule is violated, preventing memory errors before the program even runs.
- The ownership system is designed to provide memory safety and efficiency while minimizing runtime overhead.
- Best of all worlds: the speed of a language like C/C++ with the safety of a language like Java/Python.
- Again, ownership is a compile-time feature, not a runtime feature. It is a set of rules on how we write our code to ensure memory safety.

### What is Ownership?

- The **owner** is who/what is responsible for cleaning up a piece of data when it is no longer needed.
- Every value in a Rust program has one and only one owner.
- The owner can change over the course of the program, but at any given time, there can only be one owner.
- The **owner** is usually a name variable.
  - A variable can be the owner of a value.
  - A parameter can be the owner of a value.
- Ownership also extends to composite data types (like structs, enums, arrays, tuples, etc.) that own their elements.
  - A tuple and array own their values.
