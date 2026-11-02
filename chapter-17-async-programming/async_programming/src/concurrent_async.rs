use std::time::Duration;

pub fn run() {
    println!("futures_async_syntax");
    using_spawn();
    println!("=====");
    data_transfer_messaging();
}

fn using_spawn() {
     trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            // NOTE: after each loop iteration, it jumps back to the main thread
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // Can also wait for all jobs to finish.
        handle.await.unwrap();
        println!("=====");

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // All tasks are completed and done in order with join()
        trpl::join(fut1, fut2).await;

    });   
}

fn data_transfer_messaging() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        // NOTE: future set one
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // NOTE: single future set reciever
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // NOTE: future set two
        let tx_fut = async move {
             let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}
