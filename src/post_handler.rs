pub struct Post {
    id: u32,
    author: super::user_hander::User,
    title: String,
    content: String,
    comments: Option<Vec<Comment>>
}

pub struct Comment {
    id: u32,
    author: super::user_handler::User,
    content: String
}

impl Post {
    pub fn update(&self, content: String) -> bool {
        todo!();
    }
    pub fn delete(self) -> bool {
        todo!();
    }
}

impl Comment {
    pub fn update(&self, content: String) -> bool {
        todo!();
    }
    pub fn delete(self) -> bool {
        todo!();
    }
}

pub get_comments(post: Post) -> Option<Vec<Comment>> {
    todo!();
}
