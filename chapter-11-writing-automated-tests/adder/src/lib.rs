pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(val: u64) -> u64 {
    val + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // Test macro
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4); // assertion statement
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }
}
