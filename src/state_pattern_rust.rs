pub fn run() {
    let mut post = Post::new();
    post.add_text("yo yo yo");
    post.add_text(" mf");

    let post = post.request_review();

    let post = post.approve();

    println!("Approved content: {}", post.content());
}

struct Post {}

impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::from(""),
        }
    }
}

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
    fn approve(self) -> ApprovedPost {
        ApprovedPost {
            content: self.content,
        }
    }
}

struct ApprovedPost {
    content: String,
}

impl ApprovedPost {
    fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_workflow() {
        let mut p = Post::new();
        p.add_text("Yo yo yo");
        p.add_text(", choo choo choo");
        let p = p.request_review();
        let p = p.approve();
        assert_eq!(p.content(), "Yo yo yo, choo choo choo");
    }
}
