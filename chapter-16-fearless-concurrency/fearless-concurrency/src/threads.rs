use std::thread;
use std::time::Duration;


pub fn run() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            // NOTE sleep allows for thread to stop execution
            // and release it's resources to other threads for a short
            // duration.
            thread::sleep(Duration::from_millis(1));
        }
    });

    // the handle thread executes fully before using the below for loop
    //handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // NOTE: wait for all threads to finish
    handle.join().unwrap();

    // NOTE: Moving variables into threads
    let v = vec![1, 2, 3];
    let handle_v = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle_v.join().unwrap();

    println!("threads");
}
