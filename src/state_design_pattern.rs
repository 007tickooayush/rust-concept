pub mod state_design {
    /// IMPLEMENTING THE STATE DESIGN PATTERN
    /// IN RUST FRIENDLY WAY

    pub struct Post {
        content: String
    }

    pub struct DraftPost {
        content: String
    }

    pub struct PendingReviewPost {
        content: String,
        approved: bool
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new()
            }
        }
        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self,text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
                approved: false
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> (Post,PendingReviewPost) {
            let content = self.content.clone();

            (Post {
                content: content.clone()
            },
            PendingReviewPost {
                content: content.clone(),
                approved: true
            })
        }
    }

    #[test]
    pub fn test_state_design() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let post = post.approve();
        assert_eq!(post.0.content(),"I ate a salad for lunch today");
        assert!(post.1.approved);
    }
}