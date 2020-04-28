//! Comment crate
//!
//! Here will be stored everything that is related to the commentaries
//! The post linked to them, the user that writes them, the number of replies, ...
use chrono::NaiveDateTime;
use diesel::prelude::*;
use either::*;

use crate::database::models::Entity;
use crate::lib::consequence::*;

use crate::database::schema::comments;
use crate::database::schema::comments::dsl::{self, comments as table} ;


#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "comments"]
pub struct CommentEntity {
    pub id: u32,
    pub post_id: u32,
    pub parent_id: Option<u32>,
    pub content: String,
    pub author_id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub hidden_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub votes: u32,
    pub score: i32,
}


#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "comments"]
pub struct CommentMinima {
    pub post_id: u32,
    pub content: String,
    pub author_id: u32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub votes: u32,
    pub parent_id: u32,
}


impl Entity for CommentEntity {

    type Minima = CommentMinima;

    /* ------------------------------- STATIC ------------------------------ */

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table.find(id).first::<CommentEntity>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        let past = Self::select(conn, minima)?;
        if past.is_some() {
            Ok(Left(past.unwrap()))
        } else {
            diesel::insert_into(table).values(minima).execute(conn)?;
            let future = Self::select(conn, minima)??;
            Ok(Right(future))
        }
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(
                dsl::author_id.eq(&minima.author_id)
                    .and(dsl::post_id.eq(&minima.post_id))
                    .and(dsl::parent_id.eq(&minima.parent_id))
            )
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(table).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(table.filter(dsl::id.eq(self.id)))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
    }
}
