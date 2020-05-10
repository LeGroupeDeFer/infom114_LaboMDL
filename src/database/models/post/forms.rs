use crate::database::models::prelude::Post;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPost {
    pub title: String,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub kind: String,
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeVote {
    pub vote: i32, // -1, 0, +1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportData {
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityReport {
    pub month: String,
    pub new: u64,
    pub interaction: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedPost {
    pub post: Post,
    pub count_flag: u32,
    pub reasons: Vec<String>,
}
