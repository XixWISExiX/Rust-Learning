# Fearless Concurrency

Bugs that can still be present:
- Race conditions
- Deadlocks
- Specific bugs

Implementing traits like `Send` and `Sync` manually is Unsafe.
(these traits are needed for a mutex to be shared amongst multiple concurrent threads)

# Topics
- Using Threads to Run Code Simultaneously
- Transfer Data Between Threads with Message Passing
- Shared-State Concurrency
- Extensible Concurrency with Send and Sync
