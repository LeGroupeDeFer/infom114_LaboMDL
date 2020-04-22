#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table("tags_subscription")]
pub struct RelUserTagEntity {
    pub id: u32,
    pub user_id: u32,
    pub tag_id: u32,
}
// TODO : implement minima
