#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RelUserTag {
    pub id: u32,
    pub user_id: u32,
    pub tag_id: u32,
}
