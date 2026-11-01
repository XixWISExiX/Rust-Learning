use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    println!("thread_shared_state");
    basic_mutex();
    println!();
    multiple_threads_changing_one_value_atomic_mutex();
}

fn basic_mutex() {
    let m = Mutex::new(5);    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}

fn multiple_threads_changing_one_value_single_mutex() {
    // NOTE: This errors out because ownership is not shared.
    //let counter = Mutex::new(0);

    //let mut handles = vec![];
    //for _ in 0..10 {
    //    let handle = thread::spawn(move || {
    //        let mut num = counter.lock().unwrap();           
    //        *num += 1;
    //    });
    //    handles.push(handle);
    //}
    //for handle in handles {
    //    handle.join().unwrap();
    //}
    //println!("Result: {}", *counter.lock().unwrap());
}

fn multiple_threads_changing_one_value_rc_mutex() {
    // NOTE: This errors out because the RcMutex cannot be shared between threads
    // safely (Send trait needs to be implemented)

    //let counter = Rc::new(Mutex::new(0));
    //let mut handles = vec![];
    //for _ in 0..10 {
    //    let counter = Rc::clone(&counter);
    //    let handle = thread::spawn(move || {
    //        let mut num = counter.lock().unwrap();
    //        *num += 1;
    //    });
    //    handles.push(handle);
    //}
    //for handle in handles {
    //    handle.join().unwrap();
    //}
    //println!("Result: {}", *counter.lock().unwrap());
}

fn multiple_threads_changing_one_value_atomic_mutex() {
    // NOTE: We are now using an atomic
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
