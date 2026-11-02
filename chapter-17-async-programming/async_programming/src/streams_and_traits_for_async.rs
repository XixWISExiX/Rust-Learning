use trpl::StreamExt; // NOTE: stream.next() can now be called, implements StreamExt trait
use std::pin::{Pin, pin};
use std::task::{Context, Poll};
use trpl::Html;
use std::time::Duration;

pub fn run() {
    println!("streams_and_traits_for_async");
    use_stream();
    println!("=====");
    polls_and_futures();
    println!("=====");
}

fn use_stream() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}

trait FutureUnderTheHood {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum PollUnderTheHood<T> {
    Ready(T),
    Pending,
}

trait StreamUnderTheHood {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}

trait StreamExtUnderTheHood: StreamUnderTheHood {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;
    // other methods...
}

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn polls_and_futures() {
    // NOTE: How polling looks under the hood
    //trpl::block_on(async {
    //    let url = "https://www.rust-lang.org";
    //    let mut page_title_fut = page_title(url);
    //    loop {
    //        match page_title_fut.poll() {
    //            Ready(value) => match page_title {
    //                Some(title) => println!("The title for {url} was {title}"),
    //                None => println!("{url} had no title"),
    //            }
    //            Pending => {
    //                // continue
    //            }            
    //        }
    //    }
    //});


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

        // NOTE: Trying to make a list of futures tasks to wait for
        // NOTE: This also errors out because we need a pin.
        //let futures: Vec<Box<dyn Future<Output = ()>>> =
        //    vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        // NOTE: Uses Pin instead of Box
        // NOTE: dyn is for dynamic run time and is executed at runtime. Very similar operationally to C++ inheritance with Vtable.
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];

        trpl::join_all(futures).await;
    });

}
