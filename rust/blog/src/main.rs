#![allow(unused_variables)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // --snip--
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn request_review(&mut self) {
        
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
        
        /*
        if let Some(s) = self.state {
            self.state = Some(s.request_review());
        }
        */
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    //fn request_review(&self) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    //fn request_review(&self) -> Box<dyn State> {
        //Box::new(PendingReview {})
        //Box::new(Draft {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
    //fn request_review(&self) -> Box<dyn State> {
        //let i: i32 = *self.clone();
        //以下行会出现该错误：error[E0507]: cannot move out of borrowed content
        //Box::new(*self.clone())
        self
    }
}

fn main() {
    println!("hello");
}
