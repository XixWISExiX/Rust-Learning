// tests/integration_test.rs doesn't need #[cfg(test)] declaration, it's implicit

use adder::add;
use adder::add_two;

mod common;

#[test]
fn adding_with_add_fn_and_add_two_fn() {
    common::setup();
    assert_eq!(add_two(add(2,3)), 7);
}
