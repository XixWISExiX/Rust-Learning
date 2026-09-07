# Packages, Crates, and Modules

- Crates: A tree of modules that produces a library or executable.
- Packages: A Cargo feature that lets you build, test, and share crates.
- Paths: A way of naming an item, such as a struct, function, or module.
- Modules and use: Let you control the organization, scope, and privacy of paths.

## Crates

The crate is the smallest amount of code that the Rust compiler considers at a time and there are two types of crates.

### Binary Crate

A Binary Crate involves an executable that you can run in the cmd line and must contain a `main.rs` that defines what happens once the executable is ran.

### Library Crate

A Library Crate on the other hand doesn't need an executable and should only define functionality that will be shared between projects. 

Usually in Rust when people say crate, they mean library in a general programming context.

## Packages

A Packages is a bundle of one or more crates that provides a set functionality. It contains a Cargo.toml file that describes how to build those crates. A package can contain many binary crates, but only one library crate and a Package must contain at least one crate.

## Modules

1. _Start from the crate root:_ When compiling a crate, the compiler first looks in the crate root file for code to compile (usually src/lib.rs for library or src/main.rs for binary crate).
2. _Declaring modules:_ In the crate root file, you can declare new modules; say you declare a "garden" module with `mod garden;`. This code can either be inline inside the crate root file, in `src/garden.rs`, or in `src/garden/main.rs`
3. _Declaring submodules:_ If you declare something like `mod vegetables;` in `src/garden.rs` the compiler will look for either inline inside `src/garden.rs`, in `src/garden/vegetables.rs`, or in `src/garden/vegetables/mod.rs`.
4. _Paths to code in modules:_ Once a module is a part of your crate, you can refer to the code in that module from anywhere else in that same crate, as long as the privacy rules allow. For example if we have Asperagus under vegetables, then we can get it with the following. `crate::garden::vegetables::Asperagus`.
5. _Private vs Public:_ Code in modules are private by default. If you want to change this, declare it with `pub mob` instead of `mob`. To make items in a public module public as well, use `pub` before there declarations.
6. _The use Keyword:_ `use` creates a short cut, so you only have to write the abriviated version afterwards. E.g. after you do `use crate::garden::vegetables::Asperagus`, you can then call `Asperagus` in that scope.

One should note that if a child module can access a parent module's variables, but the vice-versa is not true unless the child modeule variables are set to public.

## Paths

To show Rust where to find an item in a module tree, we use a path. A path can take two forms.

1. An _absolute path_ starting from the crate root.
2. A _relative path_ starting from the current module.

Most of the programming notes are in the main.rs files under the following projects.

## Backyard Project (topics)

Contains units
- Packages and Crates
- Control Scope and Privacy with Modules
- Seperating modules into different files

## Resturant Project (topics)

Contains units
- Paths for referring to an item in the module tree
- Bringing Paths into Scope with the use Keyword
