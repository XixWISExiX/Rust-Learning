# Fundamentals of Asyncronous Programming: Async, Await, Futures, and Streams

Asyncronous code gives us the ability to do jobs on other threads without holding up the main thread.

- Concurrent workflow: Have multiple threads do work in general.
- Parallel workflow: Have multiple threads do SEPEARATE JOBS.
- Future: a value that may not be ready now, but will in the future (is also a trait type in Rust).
- Async: a function or block of code can be "interupted or resumed" (used in a async fashion)
- Await: keyword can be used to wait till the future has a value.
- Stream: an async receiver (recieves multiple values, kind of). Is also a trait.
- StreamExt: a higher-level trait that builds on top of the Stream trait.
- Pin: a trait that pins the value to a specific memory address.
- Unpin: a trait that unpins the Pin trait.



# Topics
- Futures and the Async Syntax `(hello-async crate)`
```bash
cd hello-async
cargo run -- "https://www.rust-lang.org" "https://www.youtube.com"
```
- Applying Concurrency with Async
- Working with any number of futures
- Streams: Futures in Sequence
- A Closer look at the Triats for Async
- Futures, Tasks, and Threads
