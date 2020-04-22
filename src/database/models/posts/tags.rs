#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RelPostTag {
    post_id: u32,
    tag_id: u32,
}
