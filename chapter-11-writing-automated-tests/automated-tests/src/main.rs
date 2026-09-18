pub mod write_tests;
pub mod controll_tests;
pub mod test_organization;

fn main() {
    write_tests::run();
    println!("++++++++++++++++++++");
    controll_tests::run();
    println!("++++++++++++++++++++");
    test_organization::run();
}

