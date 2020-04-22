use crate::database::models::prelude::{Comment, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub created_on: String,
    pub updated_on: String,
    pub locked_on: String,
    pub hidden: bool,
    pub votes: u32,
    pub score: i32,
    pub author: User,
    pub tags: Vec<String>,
    pub comments: Vec<Comment>,
    pub self_vote: u32,
}
