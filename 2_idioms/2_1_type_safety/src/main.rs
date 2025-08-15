mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(String);

    impl Id {
        pub fn new(id: u64) -> Self {
            Self(id)
        }
    }

    impl Title {
        pub fn new(title: String) -> Self {
            Self(title)
        }
    }

    impl Body {
        pub fn new(body: String) -> Self {
            Self(body)
        }
    }
}

mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);

    impl Id {
        pub fn new(id: u64) -> Self {
            Self(id)
        }
    }
}

#[derive(Debug, Clone)]
struct New;

#[derive(Debug, Clone)]
struct Unmoderated;

#[derive(Debug, Clone)]
struct Published;

#[derive(Debug, Clone)]
struct Deleted;

#[derive(Debug, Clone)]
struct Post<State> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    state: State,
}

impl<State> Post<State> {
    fn id(&self) -> &post::Id {
        &self.id
    }

    fn user_id(&self) -> &user::Id {
        &self.user_id
    }

    fn title(&self) -> &post::Title {
        &self.title
    }

    fn body(&self) -> &post::Body {
        &self.body
    }
}

impl Post<New> {
    fn new(
        id: post::Id,
        user_id: user::Id,
        title: post::Title,
        body: post::Body,
    ) -> Post<New> {
        Post { id, user_id, title, body, state: New }
    }

    fn publish(self) -> Post<Unmoderated> {
        Post {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: Unmoderated,
        }
    }
}

impl Post<Unmoderated> {
    fn allow(self) -> Post<Published> {
        Post { 
            id: self.id, 
            user_id: self.user_id, 
            title: self.title, 
            body: self.body, 
            state: Published, 
        }
    }

    fn deny(self) -> Post<Deleted> {
        Post { 
            id: self.id, 
            user_id: self.user_id, 
            title: self.title, 
            body: self.body, 
            state: Deleted, 
        }
    }
}

impl Post<Published> {
    fn delete(self) -> Post<Deleted> {
        Post { 
            id: self.id, 
            user_id: self.user_id, 
            title: self.title, 
            body: self.body, 
            state: Deleted, 
        }
    }
}

impl Post<Deleted> {
    fn is_deleted(&self) -> bool {
        true
    }
}

fn main() {
    let new_post = Post::new(
        post::Id::new(1),
        user::Id::new(42),
        post::Title::new("My First Post".to_string()),
        post::Body::new("This is the content of my first post.".to_string()),
    );
    let post_unmoderated = new_post.publish();
    let post_published = post_unmoderated.allow();
    let post_deleted = post_published.delete();
    println!("{}", post_deleted.is_deleted());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_state_transitions() {
        let new_post = Post::new(
            post::Id::new(1),
            user::Id::new(42),
            post::Title::new("My First Post".to_string()),
            post::Body::new("This is the content of my first post.".to_string()),
        );
        
        assert_eq!(new_post.id(), &post::Id::new(1));
        assert_eq!(new_post.user_id(), &user::Id::new(42));
        assert_eq!(new_post.title(), &post::Title::new("My First Post".to_string()));
        
        let unmoderated_post = new_post.publish();
        assert_eq!(unmoderated_post.id(), &post::Id::new(1));
        assert_eq!(unmoderated_post.user_id(), &user::Id::new(42));
        
        let published_post = unmoderated_post.allow();
        assert_eq!(published_post.id(), &post::Id::new(1));

        let deleted_post = published_post.delete();
        assert_eq!(deleted_post.id(), &post::Id::new(1));
        assert!(deleted_post.is_deleted());
    }
    
    #[test]
    fn test_deny_transition() {
        let new_post = Post::new(
            post::Id::new(2),
            user::Id::new(100),
            post::Title::new("Controversial Post".to_string()),
            post::Body::new("This post will be denied.".to_string()),
        );
        
        let unmoderated_post = new_post.publish();
        let deleted_post = unmoderated_post.deny();
        
        assert_eq!(deleted_post.id(), &post::Id::new(2));
        assert_eq!(deleted_post.user_id(), &user::Id::new(100));
        assert!(deleted_post.is_deleted());
    }
}
