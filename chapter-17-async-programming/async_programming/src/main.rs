use std::{thread, time::Duration};

pub mod concurrent_async;
pub mod async_yield_abstraction;
pub mod streams_and_traits_for_async;

fn main() {
    concurrent_async::run();
    println!("+++++++++++++++++++");
    async_yield_abstraction::run();
    println!("+++++++++++++++++++");
    streams_and_traits_for_async::run();
    println!("+++++++++++++++++++");
    putting_it_all_together();
}

fn putting_it_all_together() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
    
}
