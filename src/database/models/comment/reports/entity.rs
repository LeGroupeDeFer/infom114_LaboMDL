use crate::database::models::prelude::*;

use crate::database::schema::comments_reports;
use crate::database::tables::comments_reports_table as table;

use crate::lib::{Consequence, EntityError};
use chrono::NaiveDateTime;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::Either;

#[derive(Queryable, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "comments_reports"]
#[belongs_to(PostEntity, foreign_key = "comment_id")]
#[belongs_to(UserEntity, foreign_key = "user_id")]
pub struct RelCommentReportEntity {
    pub comment_id: u32,
    pub user_id: u32,
    pub reported_at: NaiveDateTime,
    pub reason: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "comments_reports"]
pub struct RelCommentReportMinima {
    pub comment_id: u32,
    pub user_id: u32,
    pub reason: Option<String>,
}

impl Entity for RelCommentReportEntity {
    type Minima = RelCommentReportMinima;

    fn by_id(_conn: &MysqlConnection, _id: &u32) -> Consequence<Option<Self>> {
        unimplemented!()
    }

    fn all(_conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        unimplemented!()
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        Ok(match Self::select(conn, &minima)? {
            Some(rel_comment_report) => Either::Left(rel_comment_report),
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
                comments_reports::comment_id
                    .eq(minima.comment_id)
                    .and(comments_reports::user_id.eq(minima.user_id)),
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
                comments_reports::comment_id
                    .eq(self.comment_id)
                    .and(comments_reports::user_id.eq(self.user_id)),
            ),
        )
        .execute(conn)?;
        Ok(())
    }
}

impl RelCommentReportEntity {
    pub fn count_by_comment_id(conn: &MysqlConnection, comment_id: &u32) -> Consequence<u64> {
        table
            .select(count(comments_reports::user_id))
            .filter(comments_reports::comment_id.eq(comment_id))
            .first::<i64>(conn)
            .map(|v: i64| v as u64)
            .map(Ok)?
    }
}
