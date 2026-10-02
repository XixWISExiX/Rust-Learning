use rand::RngExt;

fn main() {
    let num = 10;
    let mut rng = rand::rng();
    let ran_num: i32 = rng.random_range(1..=100);
    println!("Random num from adder crate: {ran_num}");
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
