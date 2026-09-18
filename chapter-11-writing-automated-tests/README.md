# Automated Tests

You are able to run a test via `cargo test` which builds the resultant binary and runs tests.

## How to Write Tests

- Tests can be ignored shown as "0 ignored"
- Tests can be filtered shown as "0 filtered out"
- Tests can measure performance shown as "0 measured"
- There is also a *Doc-tests* that compiles code that appears in API documentation, though this is covered in another chapter.

## Controlling How Tests are Run

- You can controll the number of threads that the cargo uses on tests with `cargo test -- --test-threads=1`
- You can controll seeing the println of all tests (including passing tests) with `cargo test -- --show-output`
- You can run a single/multiple test with `cargo test test_name` e.g. `cargo test this_test_will_pass` or `cargo test add2`
- You can run only ignored tests with `cargo test -- --ignored`
- You can run normal and ignored tests with `cargo test -- --include-ignored`

## Test Organization

### Unit Tests
Unit tests are small and more focused testing one module in isolation at a time.

- In Rust, unit tests go in the same src file that the code is written in.
- Rust allows you to test private functions in the unit test.
- `#[cfg(test)]` (configuration only upon "test") on the tests module tells Rust to compile and run the test code only when you run `cargo test` (this also includes function that have `#[test]`)

### Integration Tests
Integration tests are entirely external to your library and use your code in the same way any external code would, only using the external interface and potentially exercising multiple modules per test.

- *IMPORTANT* if you don't have a `src/lib.rs` file, you can't create integration tests with `tests/` directory. This is why you can have a `src/lib.rs` and a `src/main.rs` simultaneously.

This is important to test that parts of the library work with eachother correctly. To create a integration tests, you first need a `tests/` directory.

This `tests/` directory goes on right next to src right under the top level crate directory, so for a crate called `adder` for example, you would have `adder/src/lib.rs` and then you would have `adder/tests/integration_test.rs`.

adder
├── Cargo.lock
├── Cargo.toml
├── src
│    └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
     
`#[cfg(test)]` implicitly is used inside of `tests/integration_test.rs`, so you don't have to envoke it.

- NOTE: If a unit test fails, integration tests will not run.
  - All controll commands that apply to unit tests also apply to integration tests e.g. you can run only integration tests with just `cargo test --test integration_test`.


## Adder Project (topics)

- How to Write Tests

## Automated Tests Project (topics)

- Controlling How Tests Are Run
- Test Organization
