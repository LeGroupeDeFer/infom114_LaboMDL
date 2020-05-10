//! Comment crate
//!
//! Here will be stored everything that is related to the commentaries
//! The post linked to them, the user that writes them, the number of replies, ...
use chrono::NaiveDateTime;
use diesel::expression::functions::date_and_time::now;
use diesel::prelude::*;
use either::*;

use crate::database::models::Entity;
use crate::lib::consequence::*;

use crate::database::schema::comments;
use crate::database::schema::comments::dsl::{self, comments as table};

#[derive(
    Identifiable,
    Queryable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
)]
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
    pub parent_id: Option<u32>,
}

impl Entity for CommentEntity {
    type Minima = CommentMinima;

    /* ------------------------------- STATIC ------------------------------ */

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table
            .find(id)
            .first::<CommentEntity>(conn)
            .optional()
            .map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        diesel::insert_into(table).values(minima).execute(conn)?;
        let future = Self::select(conn, minima)??;
        Ok(Right(future))
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        let filtered = table
            .filter(dsl::author_id.eq(&minima.author_id))
            .filter(dsl::post_id.eq(&minima.post_id))
            .filter(dsl::content.eq(&minima.content));

        match &minima.parent_id {
            Some(_id) => filtered
                .filter(dsl::parent_id.eq(&minima.parent_id))
                .order(dsl::created_at.desc())
                .first::<Self>(conn)
                .optional()
                .map(Ok)?,
            None => filtered
                .filter(dsl::parent_id.is_null())
                .order(dsl::created_at.desc())
                .first::<Self>(conn)
                .optional()
                .map(Ok)?,
        }
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(table)
            .set(self)
            .execute(conn)
            .map(|_| self)
            .map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::update(&self)
            .set(dsl::deleted_at.eq(now))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
        // diesel::delete(table.filter(dsl::id.eq(self.id)))
        //     .execute(conn)
        //     .map(|_| ())
        //     .map(Ok)?
    }
}
