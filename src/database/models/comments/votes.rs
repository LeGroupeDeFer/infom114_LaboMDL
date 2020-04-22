use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RelCommentVote {
    comment_id: u32,
    vote_id: u32,
    voted_at: Option<NaiveDateTime>,
    vote_value: bool,
}
