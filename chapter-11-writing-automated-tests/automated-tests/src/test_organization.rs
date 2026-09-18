pub fn run() {
    println!("test-organization.rs");
}

fn public_fn() -> String {
    format!("I'm public")
}

fn private_fn() -> String {
    format!("I'm private")
}

#[cfg(test)]
mod tests {
    use super::*; // NOTE: This line puts all the functions into scope of tests

    #[test]
    fn test_private_fn() {
        assert!(private_fn().contains("private"));       
    }

    #[test]
    fn test_public_fn() {
        assert!(public_fn().contains("public"));       
    }

}

