use crate::database::models::prelude::{Entity, PostEntity, RelUserPollAnswerEntity};
use crate::database::schema::{poll_answers, users_poll_answers};
use crate::database::tables::{poll_answers_table as table, users_poll_answers_table};
use crate::lib::Consequence;

use diesel::prelude::*;
use diesel::MysqlConnection;
use either::Either;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "poll_answers"]
#[belongs_to(PostEntity, foreign_key = "post_id")]
pub struct PollAnswerEntity {
    pub id: u32,
    pub post_id: u32,
    pub answer: String,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "poll_answers"]
pub struct PollAnswerMinima {
    pub post_id: u32,
    pub answer: String,
}

impl Entity for PollAnswerEntity {
    type Minima = PollAnswerMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table.find(id).first(conn).optional().map(Ok)?
    }

    fn all(_conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    fn insert(_conn: &MysqlConnection, _minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        unimplemented!()
    }

    fn select(_conn: &MysqlConnection, _minima: &Self::Minima) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn update(&self, _conn: &MysqlConnection) -> Consequence<&Self> {
        unimplemented!()
    }

    fn delete(self, _conn: &MysqlConnection) -> Consequence<()> {
        unimplemented!()
    }
}

impl PollAnswerEntity {
    pub fn bulk_insert(conn: &MysqlConnection, post_id: &u32, answers: &[&str]) -> Consequence<()> {
        diesel::insert_into(table)
            .values(
                answers
                    .iter()
                    .map(|&answer| PollAnswerMinima {
                        post_id: post_id.clone(),
                        answer: answer.to_string(),
                    })
                    .collect::<Vec<PollAnswerMinima>>(),
            )
            .execute(conn)?;

        Ok(())
    }

    pub fn by_post_id(conn: &MysqlConnection, post_id: &u32) -> Consequence<Vec<Self>> {
        Ok(table
            .filter(poll_answers::post_id.eq(post_id))
            .load::<Self>(conn)?)
    }

    pub fn count_vote(&self, conn: &MysqlConnection) -> u64 {
        RelUserPollAnswerEntity::count_by_answer(conn, &self.id).unwrap_or(0)
    }

    pub fn get_user_answer(
        conn: &MysqlConnection,
        user_id: &u32,
        post_id: &u32,
    ) -> Consequence<Option<PollAnswerEntity>> {
        Ok(table
            .inner_join(users_poll_answers_table)
            .filter(
                users_poll_answers::user_id
                    .eq(user_id)
                    .and(poll_answers::post_id.eq(post_id)),
            )
            .first::<(Self, RelUserPollAnswerEntity)>(conn)
            .optional()?
            .map(|(entity, _)| entity))
    }
}
