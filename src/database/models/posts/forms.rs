#[derive(Serialize, Deserialize, Debug)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub author_token: String,
}
