# More About Cargo and Crates.io

## Published Crate Example
https://crates.io/crates/xixwisexix_art

## Topics
- Customize your build through release profiles
- Publish libraries on crates.io
- Organize large projects with workspaces
- Install binaries from crates.io
- Extend Cargo using custom commands

## Customizing Builds with Release Profiles

Has good defaults for development
```bash
cargo build
```

Has good defaults for release
```bash
cargo build --release
```

See the a_crate Cargo.toml to see configuration setting changes.

## Publish libraries on crates.io

/// Contains crate markdown documentation

Read documentation
```bash
cargo doc --open
```

Documentation adheres to markdown rules so these are popular headings
- # Examples
- # Panics (scnarios where the function could panic)
- # Errors (usually noted for functions that return Results type)
- # Safety (usually noted for unsafe functions)

Fun fact, running `cargo test` will test the examples in your documentation!

//! Contains documentation comments

- You can publish a crate with `cargo publish`
- NOTE: A publish is perminant
- You can get a specific version of a crate using a commande like the following `$ cargo yank --vers 1.0.1`

## Cargo Workspaces

NOTE: Each subspace would need to be **published seperately**

- Workspaces can help manage multiple related packages in a crate in tandem.
- `cargo test` applies to all sub-crates
- `cargo run -p adder` runs the specific sub package called adder

## Installing Binaries with cargo install

- `cargo install` is used to install and use binary crates locally (aka it has a main.rs)
- content is installed in `$HOME/.cargo/bin`

## Extending Cargo with Custom Commands

If a binary in your `$PATH` is named `cargo-something` you can run that crate as if it was a cargo subcommand with `cargo something`, this allows you to create custom cargo extensions.
