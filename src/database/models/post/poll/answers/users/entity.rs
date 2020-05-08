use crate::database::models::prelude::{Entity, PollAnswerEntity, UserEntity};
use crate::database::schema::users_poll_answers;
use crate::database::tables::users_poll_answers_table as table;
use crate::lib::{Consequence, EntityError};

use diesel::dsl::count;
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::Either;

#[derive(Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "users_poll_answers"]
#[belongs_to(PollAnswerEntity, foreign_key = "answer_id")]
#[belongs_to(UserEntity, foreign_key = "user_id")]
pub struct RelUserPollAnswerEntity {
    pub answer_id: u32,
    pub user_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users_poll_answers"]
pub struct RelUserPollAnswerMinima {
    pub answer_id: u32,
    pub user_id: u32,
}

impl Entity for RelUserPollAnswerEntity {
    type Minima = RelUserPollAnswerMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        Err(EntityError::NotIdentifiable)?
    }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        Ok(match Self::select(conn, minima)? {
            Some(entity) => Either::Left(entity),
            None => {
                diesel::insert_into(table).values(minima).execute(conn)?;
                Either::Right(Self::select(conn, minima)??)
            }
        })
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(
                users_poll_answers::user_id
                    .eq(minima.user_id)
                    .and(users_poll_answers::answer_id.eq(minima.answer_id)),
            )
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        unimplemented!()
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(
            table.filter(
                users_poll_answers::user_id
                    .eq(self.user_id)
                    .and(users_poll_answers::answer_id.eq(self.answer_id)),
            ),
        )
        .execute(conn)?;
        Ok(())
    }
}

impl RelUserPollAnswerEntity {
    pub fn count_by_answer(conn: &MysqlConnection, answer_id: &u32) -> Consequence<u64> {
        table
            .select(count(users_poll_answers::user_id))
            .filter(users_poll_answers::answer_id.eq(answer_id))
            .first::<i64>(conn)
            .map(|v: i64| v as u64)
            .map(Ok)?
    }

    pub fn clear_vote_for_user(
        conn: &MysqlConnection,
        user_id: &u32,
        answer_ids: &[&u32],
    ) -> Consequence<()> {
        diesel::delete(
            table.filter(
                users_poll_answers::user_id
                    .eq(user_id)
                    .and(users_poll_answers::answer_id.eq_any(answer_ids)),
            ),
        )
        .execute(conn)?;

        Ok(())
    }
}
