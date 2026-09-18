pub fn run() {
    println!("controll-tests.rs");
}

fn prints_and_returns_10() -> i32 {
    println!("I don't have a value");
    10
}

fn add2(val: i32) -> i32 {
    val + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10();
        assert_eq!(value, 10);
    }

    // Should print contents on fail by default
    //#[test]
    //fn this_test_will_fail() {
    //    let value = prints_and_returns_10();
    //    assert_eq!(value, 5);
    //}

    // Both the below tests activate on
    // `cargo test add2`
    #[test]
    fn this_should_add2_to_6() {
        assert_eq!(add2(4), 6);        
    }
    #[test]
    fn this_should_add2_to_14() {
        assert_eq!(add2(12), 14);        
    }

    // Use this for time consuming tests you want to run occationally, but not through
    // normal cargo test
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes a long time
    }
}
