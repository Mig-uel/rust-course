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

## The Stack and the Heap

- The **stack** and the **heap** are two different areas of memory used for different purposes.
- The **stack** and the **heap** read and write data in different ways that offer advantages and disadvantages.
- The **stack** is generally fast but it only supports data with a known, fixed size at compile time.
- The **heap** is generally slower but it supports dynamic data whose size may not be known at compile time or may change over time.

### The Stack

- A **stack** stores values in the sequential order it receives them.
- A stack is last-in, first-out (LIFO). The last value pushed onto the stack is the first value popped off.
- The technical terminology for adding data to the stack is **pushing onto the stack** and for removing data from the stack is **popping off the stack**.

### The Stack II

- All stack data has a fixed, consistent size that is known at compile time.
- Data types like integers, floating-point numbers, booleans, characters, and tuples/arrays with fixed-size elements are stored on the stack.
- The pieces of data on the stack will not grow (change size) or shrink (change size) while the program is running.
- The stack is very fast because it only requires moving the stack pointer up or down to allocate or deallocate memory.
- The stack has limited size, which can lead to stack overflow errors if too much data is pushed onto the stack.

### The Heap

- The **heap** is a larger region of memory. Think of it like a warehouse.
- The heap is for data whose size is not known at compile time (user input, a file's contents, etc.) or data that needs to grow or shrink while the program is running.
- When the Rust program needs dynamic space, it requests it from the heap. A program called the **memory allocator** finds a big enough chunk of memory on the heap and returns a reference to that chunk.
- This reference is a pointer to the location of that chunk of memory.
- The pointer is stored on the stack, but the actual data is stored on the heap.
- Allocating and deallocating memory on the heap is generally slower than on the stack because it involves more complex operations like searching for a suitable memory block and managing fragmentation.

### References

- The memory allocators returns a reference (a pointer), which is an address in memory where the data is stored on the heap.
- The reference points to the memory address of the data on the heap.
- Think of a parking lot giving you a reference (spot "A23") to where your car is parked.
- We can store a reference in a variable in a Rust program. References have a fixed size, so Rust can store them on the stack.

### The Heap II

- Allocating on the heap is slower than pushing to the stack. The memory allocator has to spend time searching for an open spot large enough to fit the data.
- Accessing data is faster on the stack than the heap as well. With a heap, the program has to follow the reference (pointer) to find the data.
- A stack stores the data in sequential order, while a heap stores data in a more scattered manner.
- The heap can become fragmented over time as data is allocated and deallocated, leading to inefficient use of memory.
- The heap has a much larger size limit compared to the stack, making it suitable for storing large amounts of data.

### Ownership

- The purpose of ownership is to assign responsibility for deallocating memory (primarily heap memory) to a specific variable or data structure.
- Ownership is a compiler feature for reducing duplicate heap data and cleaning up heap data that is no longer needed.
