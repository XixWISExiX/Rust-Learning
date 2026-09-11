# Generic Types, Traits, and Lifetimes

- *Generics* types are a type that can represent any type. (generics are as fast as concrete types due to the compiler spliting the function into multiple functions of the various different types; this is called monomorphized)
- *Traits* are Rust's equivalent of interfaces, except you cannot implement external traits on external types. Mainly, there are "Traits" and "Trait Bounds" that act as general templates in Rust.
- *Lifetimes* ensure that references are valid for as long as we need them to be.

There are 3 rules the compiler follows to allow for a *Lifetime Elision*.
1. Compiler assigns a lifetime parameter to each parameter that's a reference (e.g. 3 inputs leads to lifetimes 'a 'b and 'c being applied).
2. If there is only one input lifetime parameter, the lifetime is assigned to all output lifetime parameters.
3. If there are multiple input parameters and one of them is &self, apply the lifetime of &self to the output lifetime parameter.

Most of the programming notes are in the cargo project files under the following project name.

## Collections Project (topics)

Contains units:
- Generic Data Types
- Defining Shared Behavior with Traits
- Validating References with Lifetimes
