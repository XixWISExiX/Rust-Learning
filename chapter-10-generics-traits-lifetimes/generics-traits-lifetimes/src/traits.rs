pub fn run () {
    println!("traits.rs");

    let post = SocialPost {
        username: String::from("horse_ebooks"),        
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in NHL",
        ),
    };
    println!("New article available (custom impl)! {}", article.summarize());
    println!("New article available! (default impl) {}", article.summarize_default());
    println!("{}", article.summarize_author());
    println!("Social Post Default Summary: {}", post.summarize_default());

    // Call function which uses a trait implementation
    notify(&post);
    notify2(&post, &article);
    notify3(&post);

    let obj1 = returns_summarizable("SocialPost");
    if obj1.is_some() {
        println!("obj1: {}", obj1.unwrap().summarize());
    }
    let obj2 = returns_summarizable("not valid");
    if !obj2.is_some() {
        println!("obj2 is None");
    }

}

trait Summary {
    fn summarize(&self) -> String;
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
    fn get_author(&self) -> String;
    fn summarize_author(&self) -> String {
        format!("(Read more from {}...)", self.get_author())        
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn get_author(&self) -> String {
        self.author.clone()
    }
}

struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_default(&self) -> String {
        format!("SocialPost has different summarize_default FYI!")
    }
    fn get_author(&self) -> String {
        self.username.clone()
    }
}



// A function can also take in an implemenation of a specific trait
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This function is the same as the function above.
// It just uses a trait bound, which could be used solve complex problem cases.
fn general_notify<T: Summary>(item: &T) {
    println!("Breaking news? {}", item.summarize());
}

// We can have multiple impl arguments.
fn notify2(item1: &impl Summary, item2: &impl Summary) {
    println!("Item1: {} \nItem2: {}", item1.summarize(), item2.summarize());
}

// This text can be simplifed through trait bound.
fn general_notify2<T: Summary>(item1: &T, item2: &T) {
    println!("Item1: {} \nItem2: {}", item1.summarize(), item2.summarize());
}

trait Display {
    fn display(&self) {
        println!("Display!");
    }
    // This function is not allowed to be called.
    //fn argumentless() {
    //    println!();
    //}
}


impl Display for SocialPost {
    fn display(&self) {
        println!("Display SocialPost!");        
    }
}

impl Display for NewsArticle {
    fn display(&self) {
        println!("Display NewsArticle!");        
    }
}

// We can combine an varible to be more than 1 impl
fn notify3(item: &(impl Summary + Display)) -> String {
    println!("Breaking news! {}", item.summarize());
    item.display();
    format!("Breaking news! {}", item.summarize())
}
fn general_notify3<T: Summary + Display>(item: T)  -> String {
    println!("Breaking news! {}", item.summarize());
    item.display();
    format!("Breaking news! {}", item.summarize())
}

trait Clone {}
trait Debug {}

// To many traits can be hard to read
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

// So to simplify the text, we can use 'where'
fn some_other_function<T, U>(t: &T, u: &U) where
    T: Display + Clone,
    U: Clone + Debug,
{}

// We can also return an impl (or trait) type (though it's only one type)
fn returns_summarizable(return_type: &str) -> Option<impl Summary> {
    match return_type {
        "SocailPost" =>
        {
            Some(SocialPost {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                repost: false,
            })
        }
        _ => None
    }
}

// NOTE: This func doesn't work because it returns multiple summary implementations.
//fn returns_summarizable_dynamically(return_type: &str) -> Option<impl Summary> {
//    match return_type {
//        "SocailPost" =>
//        {
//            Some(SocialPost {
//                username: String::from("horse_ebooks"),
//                content: String::from(
//                    "of course, as you probably already know, people",
//                ),
//                reply: false,
//                repost: false,
//            })
//        }
//    "NewsArticle" =>
//        {
//            Some(NewsArticle {
//                headline: String::from("The dog jumped over the boarder?!"),
//                location: String::from("Mars"),
//                author: String::from("Dog Man"),
//                content: String::from(
//                    "The dogs are jumping over the boarder \
//                    to escape bad owners.",
//                ),
//            })
//        }
//        _ => None
//    }
//}

struct Pair<T> {
    x: T,
    y: T,
}

// Can implement with two traits.
impl<T: std::fmt::Display + PartialOrd> Pair<T>{
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// Can implement a trait (Clone) for all instances of other trait (Display)
impl<T: Display> Clone for T {}
