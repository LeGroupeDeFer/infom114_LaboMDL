#[derive(Serialize, Deserialize, Debug)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub kind: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChangeVote {
    pub vote: i32, // -1, 0, +1
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReportData {
    pub reason: Option<String>,
}
