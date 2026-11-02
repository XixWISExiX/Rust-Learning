use std::time::Duration;
use trpl::Either;
use std::thread;

pub fn run() {
    println!("async_yield_abstraction");
    select_tasks();
    println!("=====");
    our_own_abstractions();
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn select_tasks() {
    trpl::block_on(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            // NOTE: switches tasks after each yield call
            slow("a", 30);            
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("a", 10);            
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("a", 20);            
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            //trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);            
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("b", 10);            
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("b", 15);            
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            slow("b", 350);            
            trpl::yield_now().await;
            //trpl::sleep(one_ms).await;
            //trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        // NOTE: ends all tasks after the fastest task is completed
        trpl::select(a, b).await;
    });   
}

// NOTE: Custom async function
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn our_own_abstractions() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });   
}
