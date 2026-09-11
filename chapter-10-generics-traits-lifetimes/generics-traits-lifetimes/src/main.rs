pub mod generic_types;
pub mod traits;
pub mod lifetimes;

fn main() {
    generic_types::run();
    println!("++++++++++++++++++++");
    traits::run();
    println!("++++++++++++++++++++");
    lifetimes::run();
}

// Put it all together example
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
