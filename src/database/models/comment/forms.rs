use crate::database::models::prelude::Comment;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewComment {
    pub content: String,
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
pub struct ReportedComment {
    pub comment: Comment,
    pub count_flag: u32,
    pub reasons: Vec<String>,
}
