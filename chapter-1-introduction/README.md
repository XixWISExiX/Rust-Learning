# Getting Started

## Hello World Rust Analysis

`rustc main.rs` compiles the coding file into an executable binary.

The function `main` runs first in the executable program.

Here, `println!` is calling a macro, this is seen because there is an `!` present.
For reference, a macro is a compile time function.

## Cargo Analysis

`Cargo` is Rust's build system and package manager.

| Rust           | C++                       |
|----------------|---------------------------|
| cargo          | Cmake + Ninja             | 
| Cargo.toml     | CMakeLists.txt            | 
| cargo build    | cmake --build ... / ninja | 
| cargo run      | build + execute           |
| cargo test     | CTest / test target       | 
| crates.io      | package registry          | 
| rustc          | g++                       |

Most of the commands should be ran in the Parent Rust Project level.
In the following example, this would all happen in `hello-cargo/`.

### Cargo new

`cargo new hello-cargo` Creates a Rust Project called `hello-cargo` this also inits a new git repo if you are not already in a gitrepo.

It expects this file structure where all coding files need to be under source.

```
hello-cargo/
    Cargo.toml
    src/
```

### Cargo build

`cargo build` Builds the project into the given compiled files `target/` folder and the `Cargo.lock` file.

`Cargo.lock` is the built version of `Cargo.toml`

`target/debug/` contains the complied binaries.

### Cargo build --release

`cargo build --release` use this command when you want to build the project for release and it will compile with optimizations.

The compiled binaries will also appear under `target/release/`.

### Cargo run

`cargo run` Runs the compiled debug project binary, under `target/debug/`.

### Cargo run --release

`cargo run --release` Runs the compiled release build project binary, under `target/release/`.

### Cargo check

`cargo check` Checks if your code compiles, but doesn't create the binary executable files.

