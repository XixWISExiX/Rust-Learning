# Error Handling

Rust errors have two major categories:
- Recoverable (e.g. file not found error)
- Unrecoverable (e.g. invalid index exception error)

Rust doesn't have exceptions, instead it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when an unrecoverable error is encountered.

Most of the programming notes are in the main.rs files under the following project.

## Error Project (topics)

- Unrecoverable Errors with panic!
- Recoverable Errors with Reulut
- To panic! or Not to panic!
