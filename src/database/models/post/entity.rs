use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::expression::functions::date_and_time::now;
use diesel::MysqlConnection;
use either::*;

use crate::database::models::Entity;
use crate::lib::consequence::*;

use crate::database::schema::posts;
use crate::database::schema::posts::dsl::{self, posts as table};


#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[table_name = "posts"]
pub struct PostEntity {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub post_type: String,
    pub author_id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub hidden_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub votes: u64,
    pub score: i64,
}


#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "posts"]
pub struct PostMinima {
    pub title: String,
    pub content: String,
    pub author_id: u32,
}


impl Entity for PostEntity {

    type Minima = PostMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table.find(id).first::<PostEntity>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        let past = Self::select(conn, minima)?;
        if past.is_some() {
            Ok(Left(past.unwrap()))
        } else {
            diesel::insert_into(table)
                .values(minima)
                .execute(conn)?;
            let future = Self::select(conn, minima)??;
            Ok(Right(future))
        }
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(
                dsl::title
                    .eq(minima.title.clone())
                    .and(dsl::content.eq(minima.content.clone())),
            )
            .first(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::update(&self)
            .set(dsl::deleted_at.eq(now))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
    }

}