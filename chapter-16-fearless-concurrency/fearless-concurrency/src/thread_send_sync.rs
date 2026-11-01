use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    println!("thread_send_sync");

    // --- Send + Sync: Arc<Mutex<T>> is safe to share across threads ---
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    // 4 threads
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        // Moving `counter` into the thread requires Arc<Mutex<i32>>: Send.
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Final count: {}", *counter.lock().unwrap()); // 4000

    // --- NOT Send: Rc<T> uses non-atomic reference counting ---
    let rc = Rc::new(5);
    // Uncommenting the next lines fails to compile:
    //
    // thread::spawn(move || {
    //     println!("{}", rc);
    // });
    //
    // error[E0277]: `Rc<i32>` cannot be sent between threads safely
    //   = help: the trait `Send` is not implemented for `Rc<i32>`
    println!("Rc is fine on one thread: {}", rc);
}

// --- Asserting the traits at compile time ---
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[allow(dead_code)]
fn checks() {
    assert_send::<Arc<Mutex<i32>>>(); // ok
    assert_sync::<Arc<Mutex<i32>>>(); // ok
    // assert_send::<Rc<i32>>();      // error: Rc is not Send
    // assert_sync::<std::cell::Cell<i32>>(); // error: Cell is not Sync
}
