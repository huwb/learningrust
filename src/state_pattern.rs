pub fn run() {
    let mut p = Post::new();
    p.add_text("Yo yo yo");
    p.add_text(", choo choo choo");
    p.request_review();
    p.approve();

    println!("Published post content: {}", p.content());
    assert_eq!(p.content(), "Yo yo yo, choo choo choo");
}

pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(StateDraft {})),
            content: String::from(""),
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
}

pub struct StateDraft {}

impl State for StateDraft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(StatePendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

pub struct StatePendingReview {}

impl State for StatePendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(StateApproved {})
    }
}

pub struct StateApproved {}

impl State for StateApproved {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_post_empty() {
        let p = Post::new();
        assert_eq!(p.content(), "");
    }

    #[test]
    fn draft_text_not_published() {
        let mut p = Post::new();
        p.add_text("Yo yo yo");
        assert_eq!(p.content(), "");
    }

    #[test]
    fn requested_review_text_not_published() {
        let mut p = Post::new();
        p.add_text("Yo yo yo");
        p.request_review();
        assert_eq!(p.content(), "");
    }

    #[test]
    fn approve_publishes_text() {
        let mut p = Post::new();
        p.add_text("Yo yo yo");
        p.request_review();
        p.approve();
        assert_eq!(p.content(), "Yo yo yo");
    }

    #[test]
    fn multiple_add_texts_append() {
        let mut p = Post::new();
        p.add_text("Yo yo yo");
        p.add_text(", choo choo choo");
        p.request_review();
        p.approve();
        assert_eq!(p.content(), "Yo yo yo, choo choo choo");
    }
}
