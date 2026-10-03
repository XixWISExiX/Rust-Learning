# Smart Pointers

- Box<T> just store data on the heap, that's it. Because of this they have no storage overhead (outside of being a heap allocation).
- RefCell<T> bypasses compiler checks and instead does runtime checks and the program panics upon incorrect usage. RefCell<T> Doesn't work for multi-threaded programming.

## Deref
- Implementing the Deref trait allows you to customize the behavior of the dereference operator `*`.
Deref Coercion (nested dereferencing basically) happens under 1 of the 3 scenarios.
- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

# Topics
- Using Box<T> to Point to Data on Heap
- Treat Smart Point Like Regular References
- Running Code on Cleanup with Drop Trait
- Rc<T>, the Reference Counted Smart Pointer
- RefCell<T> and the Interior Mutability Pattern
- Reference Cycles Can Leak Memory
