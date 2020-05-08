use crate::database;
use crate::database::models::prelude::{Entity, PollAnswerEntity};
use diesel::MysqlConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct PollAnswer {
    pub id: u32,
    pub answer: String,
    pub count: u64,
}

impl PollAnswer {
    pub fn by_post_id(conn: &MysqlConnection, post_id: &u32) -> Vec<Self> {
        PollAnswerEntity::by_post_id(conn, post_id)
            .unwrap_or(vec![])
            .into_iter()
            .map(move |entity| Self::from(entity))
            .collect::<Vec<Self>>()
    }
}

impl From<PollAnswerEntity> for PollAnswer {
    fn from(entity: PollAnswerEntity) -> Self {
        let conn = database::connection(&database::url());
        let count = entity.count_vote(&conn);
        Self {
            id: entity.id,
            answer: entity.answer,
            count,
        }
    }
}
