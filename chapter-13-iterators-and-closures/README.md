# Functional Language Features: Iterators and Closures

## Closures
- Closure, unlike functions, can use a immutable object of self and do not need type annotations.

- There are 3 types of functions that can go into a closure.
- 1. FnOnce: applies to closures that can be called once and move the values into the closure.
- 2. FnMut: applies to closures to move values into the function and instead mutate them.
- 3. Fn: applies to closures that don't move the values into the closure and dont mutate the values.

## Iterators
- Iterators allow to perform some pattern on a sequence of items in a turn.
- Iterators can use map(), filter(), collect(), and more functions that allow for simple list queries.

## Performance in Loops vs. Iterators
Loops and Iterator performance is almost identical under the hood due to compiler optimizations.

## Functional Language Features Project (topics)

- Closures
- Processing a Series of Items with Iterators
- Improving Our I/O Project
- Performance in Loops vs. Iterators
NOTE: other sub chapters here.
