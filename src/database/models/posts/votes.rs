use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RelPostVote {
    pub post_id: u32,
    pub vote_author_id: u32,
    pub voted_at: Option<NaiveDateTime>,
    pub vote_value: bool,
}
