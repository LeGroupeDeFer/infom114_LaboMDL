#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub author_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChangeVote {
    pub upvote: i32, // -1, -2, +1, +2
    pub author_token: String,
}
