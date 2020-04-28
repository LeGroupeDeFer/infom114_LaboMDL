#[derive(Serialize, Deserialize, Debug)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChangeVote {
    pub vote: i32, // -1, 0, +1
}
