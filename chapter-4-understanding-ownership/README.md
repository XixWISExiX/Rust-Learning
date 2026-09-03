# Understanding Ownership

Most of the programming notes are in the main.rs files under the following project.

## Ownership Project (topics)

Contains units
- What is Ownership
- References and Borrowing
- The Slice Type

## The concept of ownership

How memory is managed
- Some languages use garbage collectors (java)
- Other langauges use explicit allocations (c++)
- Rust does this through a system of ownership

Whether a value is on the stack or heap changes how the a systems programming language like Rust operates.

- ==Stack==: All data on the stack has a known fixed size. Data on the stack has faster access time.
- ==Heap==: Data which has unknown size at compile time or changes throughout the programs life should be stored on the heap. Data on the heap has slower access time.
