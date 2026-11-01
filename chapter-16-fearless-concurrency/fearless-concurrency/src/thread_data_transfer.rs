use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    println!("thread_data_transfer");
    basic_transmitter_and_receiver();
    println!();
    multiple_values_transferred();
    println!();
    multiple_producers();
}

fn basic_transmitter_and_receiver() {
    // NOTE: MPSC stands for multiple producer single consumer
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        transmitter.send(val).unwrap();
        //println!("val is {val}"); // NOTE: error, value is send to other thread.
    });

    let received = receiver.recv().unwrap();
    println!("Got: {received}");
}

fn multiple_values_transferred(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn multiple_producers(){
    let (tx, rx) = mpsc::channel();

    // Transmitter #1
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    // Transmitter #2
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
