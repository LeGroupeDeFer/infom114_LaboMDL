#[derive(Serialize, Deserialize, Debug)]
pub struct NewPost {
    pub title: String,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub kind: String,
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChangeVote {
    pub vote: i32, // -1, 0, +1
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReportData {
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PostReport {
    pub month: String,
    pub new: u64,
    pub interaction: u64,
}
