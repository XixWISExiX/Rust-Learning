use trpl::{Either, Html};
use std::future::Future;

// NOTE: Function is compiled into an anonomous type block with the Future trait
async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

// NOTE: Equivalent function to page_title()
fn page_title_2(url: &str) -> impl Future<Output = (&str, Option<String>)> {
    async move {
        let text = trpl::get(url).await.text().await;
        let title = Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html());
        (url, title)
    }
}

// NOTE: How an await works under the hood.
// It periodically runs the code and then switches to the other code, so state must be saved.
enum PageTitleFuture<'a> {
    Initial { url: &'a str },
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { response: trpl::Response },
}

// NOTE: How the Either type works under the hood
enum EitherUnderTheHood<A, B> {
    Left(A),
    Right(B),
}

// NOTE: main() cannot be async because main() needs to contain runtimes, not be a runtime itself
fn main() {
    let args: Vec<String> = std::env::args().collect();

    // block_on takes in a futures argument and blocks the main thread.
    // (uses tokio behind the scenes)
    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("It's page title was '{title}'"),
            None => println!("It had no title"),
        }
    })
}
