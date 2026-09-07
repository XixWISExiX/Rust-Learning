# Programming a Guessing Game

## Basic Rust Syntax

Basic Rust syntax is covered in the comments inside of `guessing-game/src/main.rs`

## Dependencies

Cargo handles dependencies under the `[dependencies]` inside `Cargo.toml` what happens is that Cargo
reaches for this version number and create name in `Crates.io`.

`Crates.io` contains open source compiled Rust projects (or crates).

### Depenency updating with Cargo update

`cargo update` looks for an update for all the dependencies and enters them into `Cargo.lock`

### Dependency Documentation

`cargo doc --open` opens the documentation for all of your dependencies in your web browser.
