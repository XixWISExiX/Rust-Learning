
pub fn run() {
    println!("design_pattern_implementation");
    state_pattern();
}

// NOTE: Normal OOP: state pattern example
//struct Post {
//    state: Option<Box<dyn State>>,
//    content: String,
//}
//
//impl Post {
//    fn new() -> Post {
//        Post {
//            state: Some(Box::new(Draft {})),
//            content: String::new(),
//        }
//    }
//
//    fn add_text(&mut self, text: &str) {
//        self.content.push_str(text);
//    }
//
//    fn content(&self) -> &str {
//        self.state.as_ref().unwrap().content(self)
//    }
//
//    fn request_review(&mut self) {
//        if let Some(s) = self.state.take() {
//            self.state = Some(s.request_review())
//        }
//    }
//
//    fn approve(&mut self) {
//        if let Some(s) = self.state.take() {
//            self.state = Some(s.approve())            
//        }
//    }
//}
//
//trait State {
//    fn request_review(self: Box<self>) -> Box<dyn State>;
//    fn approve(self: Box<self>) -> Box<dyn State>;
//    fn content<'a>(&self, post: &'a Post) -> &'a str {
//        ""
//    }
//}
//
//// --- Draft State ---
//
//struct Draft {}
//impl State for Draft {
//    fn request_review(self: Box<self>) -> Box<dyn State> {
//        Box::new(PendingReview {})
//    }
//    fn approve(self: Box<self>) -> Box<dyn State> {
//        self
//    }
//}
//
//// -------------------
//
//// --- Pending Review State ---
//
//struct PendingReview {}
//impl State for PendingReview {
//    fn request_review(self: Box<self>) -> Box<dyn State> {
//        self
//    }
//    fn approve(self: Box<self>) -> Box<dyn State> {
//        Box::new(Published {})
//    }
//}
//
//// ----------------------------
//
//// --- Publish State ---
//
//struct Published {}
//impl State for Published {
//    fn request_review(self: Box<self>) -> Box<dyn State> {
//        self
//    }
//    fn approve(self: Box<self>) -> Box<dyn State> {
//        self
//    }
//    fn content<'a>(&self, post: &'a Post) -> &'a str {
//        &post.content
//    }
//}
//
//// ---------------------



// NOTE: Rust way: state pattern example

struct DraftPost {
    content: String,
}
impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

struct PendingReviewPost {
    content: String,
}
impl PendingReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

struct Post {
    content: String,
}
impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    fn content(&self) -> &str {
        &self.content
    }
}



fn state_pattern() {
    let mut draftPost = Post::new();
    draftPost.add_text("I ate a salad for lunch today");

    //assert_eq!("", post.content());

    //post.request_review();
    //assert_eq!("", post.content());

    //post.approve();

    let pendingReviewPost = draftPost.request_review();

    let post = pendingReviewPost.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Post content: {}", post.content());
}
