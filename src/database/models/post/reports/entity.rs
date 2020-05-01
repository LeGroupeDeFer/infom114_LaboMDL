use crate::database::models::prelude::*;

use crate::database::schema::posts_reports;
use crate::database::tables::posts_reports_table as table;

use crate::lib::{Consequence, EntityError};
use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::Either;

#[derive(Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "posts_reports"]
#[belongs_to(PostEntity, foreign_key = "post_id")]
#[belongs_to(UserEntity, foreign_key = "user_id")]
pub struct RelPostReportEntity {
    pub post_id: u32,
    pub user_id: u32,
    pub reported_at: NaiveDateTime,
    pub reason: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "posts_reports"]
pub struct RelPostReportMinima {
    pub post_id: u32,
    pub user_id: u32,
    pub reason: Option<String>,
}

impl Entity for RelPostReportEntity {
    type Minima = RelPostReportMinima;

    fn by_id(_conn: &MysqlConnection, _id: &u32) -> Consequence<Option<Self>> {
        Err(EntityError::NotIdentifiable)?
    }

    fn all(_conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    /// Insert a report for a given couple post and user
    /// `Either::Left` : The user had already reported for this post
    /// `Either::Right` : The user successfully added a new report for this post
    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        Ok(match Self::select(conn, &minima)? {
            Some(rel_post_report) => Either::Left(rel_post_report),
            None => {
                diesel::insert_into(table)
                    .values(minima.clone())
                    .execute(conn)?;
                Either::Right(Self::select(conn, &minima)??)
            }
        })
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(
                posts_reports::post_id
                    .eq(minima.post_id)
                    .and(posts_reports::user_id.eq(minima.user_id)),
            )
            .first(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, _conn: &MysqlConnection) -> Consequence<&Self> {
        unimplemented!()
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(
            table.filter(
                posts_reports::post_id
                    .eq(self.post_id)
                    .and(posts_reports::user_id.eq(self.user_id)),
            ),
        )
        .execute(conn)?;
        Ok(())
    }
}

impl RelPostReportEntity {
    pub fn count_by_post_id(conn: &MysqlConnection, post_id: &u32) -> Consequence<u64> {
        table
            .select(count(posts_reports::user_id))
            .filter(posts_reports::post_id.eq(post_id))
            .first::<i64>(conn)
            .map(|v: i64| v as u64)
            .map(Ok)?
    }
}
